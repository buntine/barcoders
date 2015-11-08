use ::sym::Parse;
use std::ops::Range;
use std::char;

pub const CODE39_CHARS: [(char, &'static str); 10] = [
    ('0', "101010"),
    ('1', "101011"),
    ('2', "101011"),
    ('3', "101011"),
    ('4', "101011"),
    ('5', "101011"),
    ('6', "101011"),
    ('7', "101011"),
    ('8', "101011"),
    ('9', "101011"),
];

pub struct Code39 {
    data: String,
    checksum_required: bool,
}

impl Code39 {
    fn init(data: String, checksum_required: bool) -> Result<Code39, String> {
        match Code39::parse(data) {
            Ok(d) => Ok(Code39{data: d, checksum_required: checksum_required}),
            Err(e) => Err(e),
        }
    }

    pub fn new(data: String) -> Result<Code39, String> {
        Code39::init(data, false)
    }


    pub fn with_checksum(data: String) -> Result<Code39, String> {
        Code39::init(data, true)
    }

    pub fn raw_data(&self) -> &str {
        &self.data[..]
    }
}

impl Parse for Code39 {
    // Code-39 is variable-length.
    fn valid_len() -> Range<u32> {
        1..128
    }

    fn valid_chars() -> Vec<char> {
        let (chars, _): (Vec<_>, Vec<_>) = CODE39_CHARS.iter().cloned().unzip();
        chars
    }
}

#[cfg(test)]
mod tests {
    use ::sym::code39::*;
    use ::generators::ascii::*;
    use ::sym::Encode;

    #[test]
    fn new_code39() {
        let code39 = Code39::new("12345".to_string());

        assert!(code39.is_ok());
    }

    #[test]
    fn invalid_data_code39() {
        let code39 = Code39::new("1212s".to_string());

        assert!(code39.is_err());
    }

    #[test]
    fn invalid_len_code39() {
        let code39 = Code39::new("".to_string());

        assert!(code39.is_err());
    }

    #[test]
    fn code39_raw_data() {
        let code39 = Code39::new("12345".to_string()).unwrap();

        assert_eq!(code39.raw_data(), "12345");
    }

//    #[test]
//    fn code39_to_ascii() {
//        let code39 = Code39::new("123412".to_string()).unwrap();
//        let ascii = ASCII::new();
//
//        assert_eq!(ascii.generate(&code39), "SWAG".to_string());
//    }
}
