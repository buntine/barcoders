use ::sym::Encode;
use ::sym::Parse;
use std::ops::Range;
use std::char;

pub const ENCODINGS: [[&'static str; 10]; 3] = [
    // Left odd parity.
    ["0001101", "0011001", "0010011", "0111101", "0100011",
     "0110001", "0101111", "0111011", "0110111", "0001011",],
    // Left even parity.
    ["0100111", "0110011", "0011011", "0100001", "0011101",
     "0111001", "0000101", "0010001", "0001001", "0010111",],
    // Right.
    ["1110010", "1100110", "1101100", "1000010", "1011100",
     "1001110", "1010000", "1000100", "1001000", "1110100",],
];

pub const PARITY: [[usize; 5]; 10] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 1],
    [0, 1, 1, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 1, 1],
    [1, 1, 0, 0, 1],
    [1, 1, 1, 0, 0],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 1, 0],
    [1, 1, 0, 1, 0],
];

pub const GUARDS: [&'static str; 3] = [
    "101",   // Left.
    "01010", // Middle.
    "101",   // Right.
];

pub struct EAN13 {
    data: Vec<u32>,
}

impl EAN13 {
    pub fn new(data: String) -> Result<EAN13, String> {
        match EAN13::parse(data) {
            Ok(d) => {
                let digits = d.chars().map(|c| c.to_digit(10).expect("Unknown character")).collect();
                Ok(EAN13{data: digits})
            }
            Err(e) => Err(e),
        }
    }

    pub fn raw_data(&self) -> String {
        self.data.iter().map(|d| char::from_digit(*d, 10).unwrap()).collect::<String>()
    }

    fn checksum_digit(&self) -> u32 {
        let mut odds = 0;
        let mut evens = 0;

        for (i, d) in self.data.iter().enumerate() {
            match i % 2 {
                1 => { odds += *d }
                _ => { evens += *d }
            }
        }

        10 - (((odds * 3) + evens) % 10)
    }

    fn number_system_digit(&self) -> u32 {
        self.data[1]
    }

    fn number_system_encoding(&self) -> &'static str {
        self.char_encoding(0, &self.number_system_digit())
    }

    fn checksum_encoding(&self) -> &'static str {
        self.char_encoding(2, &self.checksum_digit())
    }

    fn char_encoding(&self, side: usize, d: &u32) -> &'static str {
        ENCODINGS[side][*d as usize]
    }

    fn left_digits(&self) -> &[u32] {
        &self.data[2..7]
    }

    fn right_digits(&self) -> &[u32] {
        &self.data[7..]
    }

    fn parity_mapping(&self) -> [usize; 5] {
        PARITY[self.data[0] as usize]
    }

    fn left_payload(&self) -> String {
        self.left_digits()
            .iter()
            .zip(self.parity_mapping().iter())
            .map(|d| self.char_encoding(*d.1, &d.0))
            .collect::<Vec<&str>>()
            .concat()
    }

    fn right_payload(&self) -> String {
        self.right_digits()
            .iter()
            .map(|d| self.char_encoding(2, &d))
            .collect::<Vec<&str>>()
            .concat()
    }
}

impl Parse for EAN13 {
    fn valid_len() -> Range<u32> {
        12..13
    }

    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

impl Encode for EAN13 {
    fn encode(&self) -> String {
        format!("{}{}{}{}{}{}{}", GUARDS[0], self.number_system_encoding(), self.left_payload(),
                                  GUARDS[1], self.right_payload(), self.checksum_encoding(), GUARDS[2])
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::generators::ascii::*;
    use ::sym::Encode;

    #[test]
    fn new_ean13() {
        let ean13 = EAN13::new("123456123456".to_string());

        assert!(ean13.is_ok());
    }

    #[test]
    fn invalid_data_ean13() {
        let ean13 = EAN13::new("1234er123412".to_string());

        assert!(ean13.is_err());
    }

    #[test]
    fn invalid_len_ean13() {
        let ean13 = EAN13::new("1111112222222333333".to_string());

        assert!(ean13.is_err());
    }

    #[test]
    fn ean13_raw_data() {
        let ean13 = EAN13::new("123456123456".to_string()).unwrap();

        assert_eq!(ean13.raw_data(), "123456123456".to_string());
    }

    #[test]
    fn ean13_encode_as_upca() {
        let ean131 = EAN13::new("012345612345".to_string()).unwrap(); // Check digit: 8
        let ean132 = EAN13::new("000118999561".to_string()).unwrap(); // Check digit: 3

        assert_eq!(ean131.encode(), "10100110010010011011110101000110110001010111101010110011011011001000010101110010011101001000101".to_string());
        assert_eq!(ean132.encode(), "10100011010001101001100100110010110111000101101010111010011101001001110101000011001101000010101".to_string());
    }

    #[test]
    fn ean13_encode() {
        let ean131 = EAN13::new("750103131130".to_string()).unwrap(); // Check digit: 8
        let ean132 = EAN13::new("983465123499".to_string()).unwrap(); // Check digit: 3

        assert_eq!(ean131.encode(), "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101".to_string());
        assert_eq!(ean132.encode(), "10101101110100001001110101011110111001001100101010110110010000101011100111010011101001000010101".to_string());
    }

    #[test]
    fn ean13_as_upca_checksum_calculation() {
        let ean131 = EAN13::new("003600029145".to_string()).unwrap(); // Check digit: 2
        let ean132 = EAN13::new("012345612345".to_string()).unwrap(); // Check digit: 8
        let two_encoding = ENCODINGS[2][2];
        let eight_encoding = ENCODINGS[2][8];
        let checksum_digit1 = &ean131.encode()[85..92];
        let checksum_digit2 = &ean132.encode()[85..92];

        assert_eq!(checksum_digit1, two_encoding);
        assert_eq!(checksum_digit2, eight_encoding);
    }

    #[test]
    fn ean13_checksum_calculation() {
        let ean131 = EAN13::new("457567816412".to_string()).unwrap(); // Check digit: 6
        let ean132 = EAN13::new("953476324586".to_string()).unwrap(); // Check digit: 2
        let six_encoding = ENCODINGS[2][6];
        let two_encoding = ENCODINGS[2][2];
        let checksum_digit1 = &ean131.encode()[85..92];
        let checksum_digit2 = &ean132.encode()[85..92];

        assert_eq!(checksum_digit1, six_encoding);
        assert_eq!(checksum_digit2, two_encoding);
    }

    #[test]
    fn ean13_to_ascii() {
        let ean13 = EAN13::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new();

        assert_eq!(ascii.generate(&ean13), "SWAG".to_string());
    }

    #[test]
    fn ean13_to_ascii_with_large_height() {
        let ean13 = EAN13::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new().height(40).xdim(2);

        assert_eq!(ascii.height, 40);
        assert_eq!(ascii.xdim, 2);
        assert_eq!(ascii.generate(&ean13), "SWAG".to_string());
    }
}
