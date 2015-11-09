use ::sym::Encode;
use ::sym::Parse;
use std::ops::Range;

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

    // TODO: Implement.
    pub fn checksum_char(&self) -> char {
        '6'
    }

    fn checksum_encoding(&self) -> &'static str {
        self.char_encoding(&self.checksum_char())
    }

    fn char_encoding(&self, c: &char) -> &'static str {
        match CODE39_CHARS.iter().find(|&ch| ch.0 == *c) {
            Some(ch) => ch.1,
            None => panic!(format!("Unknown char: {}", c)),
        }
    }

    fn payload(&self) -> String {
        let chars = self.data.chars()
                             .map(|c| self.char_encoding(&c))
                             .collect();

        if self.checksum_required {
            format!("{}{}", chars, self.checksum_encoding())
        } else {
            chars
        }
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

    #[test]
    fn code39_encode() {
        let code391 = Code39::new("750103131130".to_string()).unwrap();
        let code392 = Code39::new("983465123499".to_string()).unwrap();

        assert_eq!(code391.encode(), "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101".to_string());
        assert_eq!(code392.encode(), "10101101110100001001110101011110111001001100101010110110010000101011100111010011101001000010101".to_string());
    }

    #[test]
    fn code39_encode_with_checksum() {
        let code391 = Code39::with_checksum("750103131130".to_string()).unwrap(); // Check char: X
        let code392 = Code39::with_checksum("983465123499".to_string()).unwrap(); // Check char: X

        assert_eq!(code391.encode(), "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101".to_string());
        assert_eq!(code392.encode(), "10101101110100001001110101011110111001001100101010110110010000101011100111010011101001000010101".to_string());
    }

    #[test]
    fn code39_checksum_calculation() {
        let code391 = Code39::new("457567816412".to_string()).unwrap(); // Check char: X
        let code392 = Code39::new("953476324586".to_string()).unwrap(); // Check char: X

        assert_eq!(code391.checksum_char(), '6');
        assert_eq!(code392.checksum_char(), '2');
    }

    #[test]
    fn code39_to_ascii() {
        let code39 = Code39::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new();

        assert_eq!(ascii.generate(&code39), "SWAG".to_string());
    }

    #[test]
    fn code39_to_ascii_with_large_height() {
        let code39 = Code39::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new().height(40).xdim(2);

        assert_eq!(ascii.height, 40);
        assert_eq!(ascii.xdim, 2);
        assert_eq!(ascii.generate(&code39), "SWAG".to_string());
    }
}
