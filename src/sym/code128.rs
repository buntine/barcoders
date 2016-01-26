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

#[derive(Debug, PartialEq, Eq)]
pub enum Unit {
    A(usize),
    B(usize),
    C(usize),
}

type Encoding = [u8; 11];

#[derive(Debug, PartialEq, Eq)]
pub enum CharacterSet {
    A,
    B,
    C,
}

// Character -> Binary mappings for each of the allowable characters in each character-set.
const CODE128_CHARS: [([&'static str; 3], Encoding); 3] = [
    ([" ", " ", "00"], [1,0,1,0,0,1,1,0,1,1,0]), 
    (["!", "!", "01"], [1,1,1,0,0,1,1,0,1,1,0]), 
    (["\"", "\"", "02"], [1,1,1,1,0,1,1,0,1,1,0]), 
];

/// The Code128 barcode type.
#[derive(Debug)]
pub struct Code128(Vec<Unit>);

impl CharacterSet {
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

    fn lookup(&self, s: &str) -> Result<usize> {
        let p = self.index();
        let mut i: usize = 0;

        for c in CODE128_CHARS.iter() {
            if c.0[p] == s {
                return Ok(i)
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
                'À' => { 
                    if !units.is_empty() {
                        units.push(char_set.unit(0)); // TODO: Lookup "CODE-A"

                        if char_set == CharacterSet::C && carry.is_some() {
                            return Err(Error::Character);
                        }
                    }

                    char_set = CharacterSet::A;
                },
                'ɓ' => { 
                    if !units.is_empty() {
                        units.push(char_set.unit(0)); // TODO: lookup "CODE-B"

                        if char_set == CharacterSet::C && carry.is_some() {
                            return Err(Error::Character);
                        }
                    }

                    char_set = CharacterSet::B;
                },
                'Ć' => { 
                    if !units.is_empty() {
                        units.push(char_set.unit(0)); // TODO: Lookup "CODE-C"

                        if char_set == CharacterSet::C && carry.is_some() {
                            return Err(Error::Character);
                        }
                    }

                    char_set = CharacterSet::C;
                },
                d if d.is_digit(10) && char_set == CharacterSet::C => {
                    match carry {
                        None => carry = Some(d),
                        Some(n) => {
                            let num = format!("{}{}", n, d);
                            let i = try!(char_set.lookup(&num[..]));

                            units.push(char_set.unit(i));
                            carry = None;
                        }
                    }
                },
                _ => {
                    if char_set == CharacterSet::C {
                        return Err(Error::Character);
                    } else {
                        let i = try!(char_set.lookup(&ch.to_string()[..]));

                        units.push(char_set.unit(i))
                    }
                },
            }
        }

        Ok(units)

//        let mut units: Vec<Encoding> = vec![];
//        let mut control_char = false;
//        let mut unit_type: Option<CharacterSet> = None;
//        let mut carry: Option<u32> = None;
//
//        for ch in chars {
//            match ch {
//                '\\' => {
//                    carry = None;
//
//                    if control_char {
//                        match unit_type {
//                            Some(ut) => {
//                                let b = try!(Code128::unit_encoding("\\", ut));
//                                units.push(b);
//                            },
//                            None => return Err(Error::Character('\\'))
//                        }
//                    } else {
//                        control_char = true;
//                    }
//                },
//                d if d.is_digit(10) => {
//                    match carry {
//                        Some(c) if unit_type == CharacterSet::C => {
//                            units.pop();
//
//                            let b = try!(Code128::unit_encoding("00", ut));
//                            units.push(b);
//                        },
//                        _ => {
//
//                        }
//                    }
//                },
//                'a' if control_char && (unit_type == Some(CharacterSet::B) || unit_type == Some(CharacterSet::C)) => {
//                }
//                'b' if control_char && (unit_type == Some(CharacterSet::A) || unit_type == Some(CharacterSet::C)) => {
//                }
//                'c' if control_char && (unit_type == Some(CharacterSet::A) || unit_type == Some(CharacterSet::B)) => {
//                }
//                _ => {
//                    carry = None;
//
//
//                }
//            }
//        }
//
//        Ok(units)
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
        let code128 = Code128::new("  !! Ć0201".to_owned());

        assert!(code128.is_ok());
    }

//    #[test]
//    fn invalid_data_code128() {
//        let code128 = Code128::new("☺ ".to_owned());
//
//        assert_eq!(code128.err().unwrap(), Error::Character);
//    }
//
//    #[test]
//    fn invalid_len_code128() {
//        let code128 = Code128::new("".to_owned());
//
//        assert_eq!(code128.err().unwrap(), Error::Length);
//    }
//
//    #[test]
//    fn code128_raw_data() {
//        let code128 = Code128::new("12001".to_owned()).unwrap();
//
//        assert_eq!(code128.raw_data(), &[Unit::A("1".to_string()), Unit::A("2".to_string())]);
//    }
}
