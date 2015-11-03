use ::sym::ToASCII;
use ::sym::Encode;
use ::sym::Parse;
use std::ops::Range;
use std::char;

pub const ENCODINGS: [[&'static str; 10]; 2] = [
    // Left.
    ["0001101", "0011001", "0010011", "0111101", "0100011",
     "0110001", "0101111", "0111011", "0110111", "0001011",],
    // Right.
    ["0001101", "1100110", "1101100", "1000010", "1011100",
     "1001110", "1010000", "1000100", "1001000", "1110100",],
];

pub const GUARDS: [&'static str; 3] = [
    "101",   // Left.
    "01010", // Middle.
    "101",   // Right.
];

pub struct UPCA {
    data: String,
    pub xdim: u32,
    pub ydim: u32,
    pub height: u32,
}

impl UPCA {
    pub fn new(data: String) -> Result<UPCA, String> {
        match UPCA::parse(data) {
            Ok(d) => Ok(UPCA{data: d, xdim: 1, ydim: 1, height: 100}),
            Err(e) => Err(e),
        }
    }

    pub fn raw_data(&self) -> &str {
        &self.data[..]
    }

    // TODO: Implement as per https://en.wikipedia.org/wiki/Universal_Product_Code#Numbering
    fn checksum_digit(&self) -> char {
        '2'
    }

    fn checksum_encoding(&self) -> &'static str {
        self.char_encoding(1, &self.checksum_digit())
    }

    fn char_encoding(&self, side: usize, c: &char) -> &'static str {
        let digit = c.to_digit(10).expect("Invalid UPC-A digit.");

        ENCODINGS[side][digit as usize]
    }

    fn left_digits(&self) -> &str {
        &self.data[0..6]
    }

    fn right_digits(&self) -> &str {
        &self.data[6..]
    }

    fn left_payload(&self) -> String {
        self.left_digits()
            .chars()
            .map(|d| self.char_encoding(0, &d))
            .collect::<Vec<&str>>()
            .concat()
    }

    fn right_payload(&self) -> String {
        self.right_digits()
            .chars()
            .map(|d| self.char_encoding(1, &d))
            .collect::<Vec<&str>>()
            .concat()
    }
}

impl Parse for UPCA {
    fn valid_len() -> Range<u32> {
        11..12
    }

    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

impl Encode for UPCA {
    fn encode(&self) -> String {
        format!("{}{}{}{}{}{}", GUARDS[0], self.left_payload(), GUARDS[1], self.right_payload(), self.checksum_encoding(), GUARDS[2])
    }
}

impl ToASCII for UPCA {
    fn to_ascii(&self) -> String {
        "SWAG".to_string()
    }
}
