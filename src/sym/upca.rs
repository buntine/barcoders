use ::sym::ToASCII;
use ::sym::Parse;
use std::ops::Range;
use std::char;

pub struct UPCA {
    pub data: String,
}

impl UPCA {
    pub fn new(data: String) -> Result<UPCA, String> {
        match UPCA::parse(data) {
            Ok(d) => Ok(UPCA{data: d}),
            Err(e) => Err(e),
        }
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
