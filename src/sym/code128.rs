//! Encoder for Code128 barcodes.
//!
//! Code128 is a popular,  high-density symbology that allows for the encoding of alphanumeric
//! data along with many special characters by utilising three separate character-sets.
//!
//! Code128 also offers double-density encoding of digits.
//!
//! Barcoders provides special Unicode syntax for specifying the character set(s) which should be
//! used in the barcode:
//!
//!   \u{00C0} => Switch to character-set A (À)
//!   \u{0181} => Switch to character-set B (Ɓ)
//!   \u{0106} => Switch to character-set C (Ć)
//!
//! The default character-set is A.
//!
//! You must provide both the starting character-set along with any changes during the data. This
//! means all Code128 barcodes must start with either "\a", "\b" or "\c". Simple alphanumeric data
//! can generally use character-set A solely.
//!
//! As an example, this barcode uses character-set B:
//!   \u{0181}HE1234A*1
//! Or:
//!   ƁHE1234A*1
//!
//! And this one starts at character-set A (the default) and then switches to C to encode the digits more
//! effectively:
//!   HE@$A\u{0106}123456
//! Or:
//!   HE@$AĆ123456

use sym::helpers;
use error::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    A(usize),
    B(usize),
    C(usize),
}

type Encoding = [u8; 11];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterSet {
    A,
    B,
    C,
}

// Character -> Binary mappings for each of the allowable characters in each character-set.
const CODE128_CHARS: [([&'static str; 3], Encoding); 6] = [
    ([" ", " ", "00"], [1,0,1,0,0,1,1,0,1,1,0]), 
    (["!", "!", "01"], [1,1,1,0,0,1,1,0,1,1,0]), 
    (["\"", "\"", "02"], [1,1,1,1,0,1,1,0,1,1,0]), 
    (["??", "À", "À"], [1,1,1,1,0,1,1,0,1,1,0]), 
    (["ɓ", "??", "ɓ"], [1,1,1,1,0,1,1,0,1,1,0]), 
    (["Ć", "Ć", "??"], [1,1,1,1,0,1,1,0,1,1,0]), 
];

/// The Code128 barcode type.
#[derive(Debug)]
pub struct Code128(Vec<Unit>);

impl CharacterSet {
    fn from_char(c: char) -> Result<CharacterSet> {
        match ch {
            'À' => Ok(CharacterSet::A),
            'ɓ' => Ok(CharacterSet::B),
            'Ć' => Ok(CharacterSet::C),
            _ => Err(Error::Character),
        }
    }

    fn unit(&self, n: usize) -> Unit {
        match *self {
            CharacterSet::A => Unit::A(n),
            CharacterSet::B => Unit::B(n),
            CharacterSet::C => Unit::C(n),
        }
    }

    fn index(&self) -> usize {
        match *self {
            CharacterSet::A => 0,
            CharacterSet::B => 1,
            CharacterSet::C => 2,
        }
    }

    fn lookup(&self, s: &str) -> Result<Unit> {
        let p = self.index();
        let mut i: usize = 0;

        for c in CODE128_CHARS.iter() {
            if c.0[p] == s {
                return Ok(self.unit(i))
            } else {
                i = i+1;
            }
        }

        Err(Error::Character)
    }
}

impl Code128 {
    /// Creates a new barcode.
    /// Returns Result<Code128, Error> indicating parse success.
    pub fn new(data: String) -> Result<Code128> {
        match Code128::parse(data.chars().collect()) {
            Ok(u) => Ok(Code128(u)),
            Err(e) => Err(e),
        }
    }

    // Tokenizes and collects the data into the appropriate character-sets.
    fn parse(chars: Vec<char>) -> Result<Vec<Unit>> {
        let mut units: Vec<Unit> = vec![];
        let mut char_set = CharacterSet::A;
        let mut carry: Option<char> = None;

        for ch in chars {
            match ch {
                'À' | 'ɓ' | 'Ć' => { 
                    if !units.is_empty() {
                        if char_set == CharacterSet::C && carry.is_some() {
                            return Err(Error::Character);
                        } else {
                            let u = try!(char_set.lookup(&ch.to_string()[..]));
                            units.push(u);
                        }
                    }

                    char_set = try!(CharacterSet::from_char(ch));
                },
                d if d.is_digit(10) && char_set == CharacterSet::C => {
                    match carry {
                        None => carry = Some(d),
                        Some(n) => {
                            let num = format!("{}{}", n, d);
                            let u = try!(char_set.lookup(&num[..]));
                            units.push(u);
                            carry = None;
                        }
                    }
                },
                _ => {
                    let u = try!(char_set.lookup(&ch.to_string()[..]));
                    units.push(u);
                },
            }
        }

        match carry {
            None => Ok(units),
            Some(_) => Err(Error::Character)
        }
    }

    /// Calculates the checksum unit using a modulo-103 algorithm.
    pub fn checksum_value(&self) -> Option<u32> {
        Some(23)
    }

    fn checksum_encoding(&self) -> Encoding {
        match self.checksum_value() {
            Some(u) => self.unit_encoding(&Unit::A(u as usize)),
            None => panic!("Cannot compute checksum"),
        }
    }

    fn unit_encoding(&self, c: &Unit) -> Encoding {
        [1,1,1,0,0,0,1,1,1,0,0]
    }

    fn push_encoding(&self, into: &mut Vec<u8>, from: Encoding) {
        into.extend(from.iter().cloned());
        into.push(0);
    }

    fn payload(&self) -> Vec<u8> {
        let mut enc = vec![0];

        for c in &self.0 {
            self.push_encoding(&mut enc, self.unit_encoding(c));
        }

        self.push_encoding(&mut enc, self.checksum_encoding());

        enc
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        helpers::join_slices(&[&self.payload()[..]][..])
    }
}

#[cfg(test)]
mod tests {
    use sym::code128::*;
    use error::Error;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_code128() {
        let code128_a = Code128::new("  !! Ć0201".to_owned());
        let code128_b = Code128::new("!!  \" ".to_owned());

        assert!(code128_a.is_ok());
        assert!(code128_b.is_ok());
    }

    #[test]
    fn invalid_data_code128() {
        let code128_a = Code128::new("☺ ".to_owned());
        let code128_b = Code128::new("HELLOĆ12352".to_owned());

        assert_eq!(code128_a.err().unwrap(), Error::Character);
        assert_eq!(code128_b.err().unwrap(), Error::Character);
    }

//
//    #[test]
//    fn code128_raw_data() {
//        let code128 = Code128::new("12001".to_owned()).unwrap();
//
//        assert_eq!(code128.raw_data(), &[Unit::A("1".to_string()), Unit::A("2".to_string())]);
//    }
}
