use ::sym::ToASCII;
use ::sym::Parse;
use std::ops::Range;

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
    fn valid_len() -> Range<i32> {
        0..6
    }

    fn valid_chars() -> Vec<char> {
        vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    }
}

impl ToASCII for UPCA {
    fn to_ascii(&self) -> String {
        "SWAG".to_string()
    }
}
