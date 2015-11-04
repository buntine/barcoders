use ::sym::Parse;
use std::ops::Range;
use std::char;

pub struct Code39 {
    data: String,
}

impl Code39 {
    pub fn new(data: String) -> Result<Code39, String> {
        match Code39::parse(data) {
            Ok(d) => Ok(Code39{data: d}),
            Err(e) => Err(e),
        }
    }

    pub fn raw_data(&self) -> &str {
        &self.data[..]
    }
}

impl Parse for Code39 {
    fn valid_len() -> Range<u32> {
        0..6
    }

    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}
