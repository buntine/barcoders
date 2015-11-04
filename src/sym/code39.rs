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
        let code39 = Code39::new("1111112222222333333".to_string());

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
