//! Encoder for 2-of-5 barcodes.
//!
//! 2-of-5 barcodes are often used by Airlines and in some industrial settings.
//!
//! They also make an appearance in retail where they are sometimes used for the outer cartons on
//! groups of products (cartons of Cola, etc).
//!
//! Most of the time you will want to use the interleaved barcode over the standard option.

use super::*;

const ITF_GUARD_SIZE: usize = 4;
const ITF_START: [u8; ITF_GUARD_SIZE] = [1, 0, 1, 0];
const ITF_STOP: [u8; ITF_GUARD_SIZE] = [1, 1, 0, 1];
const ITF_CHAR_WIDTH_DOUBLE: usize = ITF_CHAR_WIDTH * 2;
const ITF_CHAR_WIDTH: usize = 9;

const STF_GUARD_SIZE: usize = 8;
const STF_START: [u8; STF_GUARD_SIZE] = [1, 1, 0, 1, 1, 0, 1, 0];
const STF_STOP: [u8; STF_GUARD_SIZE] = [1, 1, 0, 1, 0, 1, 1, 0];

const CHAR_WIDTH: usize = 14;
// Used only by the standard barcode
const CHARS: [[u8; CHAR_WIDTH]; 10] = [
    [1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0],
    [1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0],
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0],
    [1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
    [1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0],
];

// Used only by the interleaved barcode
const LENGTH_MODIFIER_SIZE: usize = 5;
const LENGTH_MODIFIERS: [[u8; LENGTH_MODIFIER_SIZE]; 10] = [
    [1, 1, 3, 3, 1],
    [3, 1, 1, 1, 3],
    [1, 3, 1, 1, 3],
    [3, 3, 1, 1, 1],
    [1, 1, 3, 1, 3],
    [3, 1, 3, 1, 1],
    [1, 3, 3, 1, 1],
    [1, 1, 1, 3, 3],
    [3, 1, 1, 3, 1],
    [1, 3, 1, 3, 1],
];

/// The standard 2-of-5 barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToF<'a>(&'a [u8]);

impl<'a> BarcodeDevExt<'a> for ToF<'a> {
    const CHARS: &'static [u8] = b"0123456789";
    const SIZE: Range<u16> = 1..256;
}

impl<'a> ToF<'a> {
    /// Create a new interleaved 2-of-5 barcode.
    #[inline]
    pub fn interleaved(data: &'a [u8]) -> Result<ToFI<'a>> {
        ToFI::new(data)
    }
}

impl<'a> ToF<'a> {
    #[inline]
    fn calc_sum(&self) -> usize {
        STF_GUARD_SIZE +
        self.0.len() * CHAR_WIDTH +
        STF_GUARD_SIZE
    }
    fn encode_into(&self, buffer: &mut [u8]) {
        let mut i = 0;
        
        // Start guard
        for bit in STF_START {
            buffer[i] = bit;
            i += 1;
        }

        // Payload
        for &byte in self.0 {
            let index = byte - b'0';
            for &bit in &CHARS[index as usize] {
                buffer[i] = bit;
                i += 1;
            }
        }

        // Stop guard
        for bit in STF_STOP {
            buffer[i] = bit;
            i += 1;
        }
    }
}

impl<'a> Barcode<'a> for ToF<'a> {
    fn new(data: &'a [u8]) -> Result<Self> where Self: Sized {
        Self::validate(data).map(Self)
    }
    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        let sum = self.calc_sum();
        if buffer.len() < sum {
            return None;
        }
        self.encode_into(buffer);
        Some(())
    }

    #[cfg(feature = "alloc")]
    fn encode(&self) -> Vec<u8> {
        let sum = self.calc_sum();
        let mut buffer = vec![0; sum];
        self.encode_into(&mut buffer);
        buffer
    }
}

/// The interleaved 2-of-5 barcode type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToFI<'a>(&'a [u8]);

impl<'a> BarcodeDevExt<'a> for ToFI<'a> {
    const CHARS: &'static [u8] = b"0123456789";
    const SIZE: Range<u16> = 1..256;
}

impl<'a> ToFI<'a> {
    fn calc_sum(&self) -> usize {
        let mut len = self.0.len();
        if len % 2 == 1 {
            len += 1;
        }
        ITF_GUARD_SIZE +
        len * ITF_CHAR_WIDTH +
        ITF_GUARD_SIZE
    }
    fn interleave(bars: u8, spaces: u8) -> [u8; ITF_CHAR_WIDTH_DOUBLE] {
        let bwidths = LENGTH_MODIFIERS[bars as usize];
        let swidths = LENGTH_MODIFIERS[spaces as usize];
        let mut buffer = [0; ITF_CHAR_WIDTH_DOUBLE];
        let mut index = 0;

        for modifier in 0..LENGTH_MODIFIER_SIZE {
            for _ in 0..bwidths[modifier] {
                buffer[index] = 1;
                index += 1;
            }
            for _ in 0..swidths[modifier] {
                buffer[index] = 0;
                index += 1;
            }
        }
        
        buffer
    }
    fn encode_into(&self, buffer: &mut [u8]) {
        let mut i = 0;
        
        // Start guard
        for bit in ITF_START {
            buffer[i] = bit;
            i += 1;
        }

        // Payload
        for byte in self.0.chunks(2) {
            let bars = byte[0] - b'0';

            // In the case of an odd number of input bytes
            let mut spaces = 0;
            if byte.len() == 2 {
                spaces = byte[1] - b'0';
            }
            
            let interleaved = Self::interleave(bars, spaces);
            for &bit in &interleaved {
                buffer[i] = bit;
                i += 1;
            }
        }

        // Stop guard
        for bit in ITF_STOP {
            buffer[i] = bit;
            i += 1;
        }
    }
}

impl<'a> Barcode<'a> for ToFI<'a> {
    fn new(data: &'a [u8]) -> Result<Self> where Self: Sized {
        Self::validate(data).map(Self)
    }
    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        let sum = self.calc_sum();
        if buffer.len() < sum {
            return None;
        }
        self.encode_into(buffer);
        Some(())
    }

    #[cfg(feature = "alloc")]
    fn encode(&self) -> Vec<u8> {
        let sum = self.calc_sum();
        let mut buffer = vec![0; sum];
        self.encode_into(&mut buffer);
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_itf() {
        let itf = ToF::interleaved(b"12345679");

        assert!(itf.is_ok());
    }

    #[test]
    fn new() {
        let tof = ToF::new(b"12345");

        assert!(tof.is_ok());
    }

    #[test]
    fn invalid_data_itf() {
        let itf = ToF::interleaved(b"1234er123412");

        assert_eq!(itf.err().unwrap(), Error::Character);
    }

    #[test]
    fn invalid_data() {
        let tof = ToF::new(b"WORDUP");

        assert_eq!(tof.err().unwrap(), Error::Character);
    }

    #[test]
    fn itf_encode() {
        let itf = ToF::interleaved(b"1234567").unwrap(); // Check digit: 0

        assert_eq!(
            "10101110100010101110001110111010001010001110100011100010101010100011100011101101",
            collapse_vec(itf.encode())
        );
    }

    #[test]
    fn encode() {
        let tof = ToF::new(b"1234567").unwrap();

        assert_eq!(
            "110110101110101010111010111010101110111011101010101010111010111011101011101010101110111010101010101110111011010110",
            collapse_vec(tof.encode())
        );
    }
}
