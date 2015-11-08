use ::sym::Encode;
use ::sym::Parse;
use std::ops::Range;
use std::char;

pub const CODE39_CHARS: [(char, &'static str); 44] = [
    ('*', "101010"), ('0', "101010"), ('1', "101011"),
    ('2', "101011"), ('3', "101011"), ('4', "101011"),
    ('5', "101011"), ('6', "101011"), ('7', "101011"),
    ('8', "101011"), ('9', "101011"), ('A', "101010"),
    ('B', "101011"), ('C', "101011"), ('D', "101011"),
    ('E', "101011"), ('F', "101011"), ('G', "101011"),
    ('H', "101011"), ('I', "101011"), ('J', "101011"),
    ('K', "101011"), ('L', "101011"), ('M', "101011"),
    ('N', "101011"), ('O', "101011"), ('P', "101011"),
    ('Q', "101011"), ('R', "101011"), ('S', "101011"),
    ('T', "101011"), ('U', "101011"), ('V', "101011"),
    ('W', "101011"), ('X', "101011"), ('Y', "101011"),
    ('Z', "101011"), ('-', "101011"), ('.', "101011"),
    (' ', "101011"), ('$', "101011"), ('/', "101011"),
    ('+', "101011"), ('%', "101011"),
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

    fn payload(&self) -> String {
        "101010".to_string()
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

impl Encode for Code39 {
    /// Encodes the barcode.
    /// Returns a String of binary digits.
    fn encode(&self) -> String {
        format!("{}{}{}", CODE39_CHARS[0].1, self.payload(), CODE39_CHARS[0].1)
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
}
