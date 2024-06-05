use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code128A<'a> {
    data: &'a [u8],
}

macro_rules! wide_array {
    [$val:literal $(, $vals:literal)*] => (
        [$val as u16 $(, $vals as u16)*]
    );
}

impl<'a> BarcodeDevExt<'a, u16> for Code128A<'a> {
    const SIZE: Range<u16> = 1..256;
    const CHARS: &'static [u16] = &wide_array![
        // ASCII characters
        b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
        b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'[', b'\\', b']', b'^', b'_',

        // Special characters
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,

        // Very special characters
        0x017B, // FNC3
        0x017A, // FNC2
        0x017D, // SHIFT
        'Ć', // Code C
        'Ɓ', // Code B
        0x017C, // FNC4
        0x0179, // FNC1
        0xFFFA, // Start A
        0xFFFB, // Start B
        0xFFFC, // Start C
        0xFFFF  // Stop
    ];
}

impl<'a> Barcode<'a> for Code128A<'a> {
    fn new(data: &'a [u8]) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
    fn encode_in_place(&self, buffer: &mut [u8]) -> Option<()> {
        todo!()
    }
    fn encode(&self) -> Vec<u8> {
        todo!()
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
    fn new_code128a() {
        let code128_a = Code128A::new(" !! Ć0201".as_bytes());
        let code128_b = Code128A::new("!!  \" ".as_bytes());

        assert!(code128_a.is_ok());
        assert!(code128_b.is_ok());
    }

    #[test]
    fn invalid_length_code128a() {
        let code128_a = Code128A::new("".as_bytes());

        assert_eq!(code128_a.err().unwrap(), error::Error::Length);
    }

    #[test]
    fn invalid_data_code128a() {
        let code128_a = Code128A::new("☺ ".as_bytes()); // Unknown character.
        let code128_b = Code128A::new("HELLOĆ12352".as_bytes()); // Trailing carry at the end.

        assert_eq!(code128_a.err().unwrap(), error::Error::Character);
        assert_eq!(code128_b.err().unwrap(), error::Error::Character);
    }

    #[test]
    fn code128a_encode() {
        let code128_a = Code128A::new("HELLO".as_bytes()).unwrap();
        let code128_b = Code128A::new("XYĆ2199".as_bytes()).unwrap();

        assert_eq!(collapse_vec(code128_a.encode()), "110100001001100010100010001101000100011011101000110111010001110110110100010001100011101011");
        assert_eq!(collapse_vec(code128_b.encode()), "110100001001110001011011101101000101110111101101110010010111011110100111011001100011101011");
    }

    #[test]
    fn code128a_encode_special_chars() {
        let code128_a = Code128A::new("B\u{0006}".as_bytes()).unwrap();

        assert_eq!(
            collapse_vec(code128_a.encode()),
            "110100001001000101100010110000100100110100001100011101011"
        );
    }

    #[test]
    fn code128_encode_longhand() {
        let code128_a = Code128A::new("HELLO".as_bytes()).unwrap();
        let code128_b = Code128A::new("XY\u{0106}2199".as_bytes()).unwrap();

        assert_eq!(collapse_vec(code128_a.encode()), "110100001001100010100010001101000100011011101000110111010001110110110100010001100011101011");
        assert_eq!(collapse_vec(code128_b.encode()), "110100001001110001011011101101000101110111101101110010010111011110100111011001100011101011");
    }
}
