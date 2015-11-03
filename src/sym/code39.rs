use ::sym::ToASCII;
use ::sym::Parse;
use std::ops::Range;

pub struct Code39 {
    pub data: String,
}

impl Code39 {
    pub fn new(data: String) -> Result<Code39, String> {
        match Code39::parse(data) {
            Ok(d) => Ok(Code39{data: d}),
            Err(e) => Err(e),
        }
    }
}

impl Parse for Code39 {
    fn valid_len() -> Range<u32> {
        0..6
    }

    fn valid_chars() -> Vec<char> {
        vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    }
}

impl ToASCII for Code39 {
    fn to_ascii(&self) -> String {
        "SWOLE".to_string()
    }
}
