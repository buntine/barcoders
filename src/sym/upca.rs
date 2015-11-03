use ::sym::ToASCII;
use ::sym::Parse;
use std::ops::Range;
use std::char;

pub const ENCODINGS: [[&'static str; 10]; 2] = [
    [
     "0001101",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
    ],
    [
     "1110010",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
     "0000000",
    ],
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
}

impl Parse for UPCA {
    fn valid_len() -> Range<u32> {
        12..13
    }

    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

impl ToASCII for UPCA {
    fn to_ascii(&self) -> String {
        "SWAG".to_string()
    }
}
