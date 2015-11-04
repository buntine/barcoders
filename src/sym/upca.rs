use ::sym::Encode;
use ::sym::Parse;
use std::ops::Range;
use std::char;

pub const ENCODINGS: [[&'static str; 10]; 2] = [
    // Left.
    ["0001101", "0011001", "0010011", "0111101", "0100011",
     "0110001", "0101111", "0111011", "0110111", "0001011",],
    // Right.
    ["0001101", "1100110", "1101100", "1000010", "1011100",
     "1001110", "1010000", "1000100", "1001000", "1110100",],
];

pub const GUARDS: [&'static str; 3] = [
    "101",   // Left.
    "01010", // Middle.
    "101",   // Right.
];

pub struct UPCA {
    data: Vec<u32>,
}

impl UPCA {
    pub fn new(data: String) -> Result<UPCA, String> {
        match UPCA::parse(data) {
            Ok(d) => {
                let digits = d.chars().map(|c| c.to_digit(10).expect("Unknown character")).collect();
                Ok(UPCA{data: digits})
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

        6 
    }

    fn checksum_encoding(&self) -> &'static str {
        self.char_encoding(1, &self.checksum_digit())
    }

    fn char_encoding(&self, side: usize, d: &u32) -> &'static str {
        ENCODINGS[side][*d as usize]
    }

    fn left_digits(&self) -> &[u32] {
        &self.data[0..6]
    }

    fn right_digits(&self) -> &[u32] {
        &self.data[6..]
    }

    fn left_payload(&self) -> String {
        self.left_digits()
            .iter()
            .map(|d| self.char_encoding(0, &d))
            .collect::<Vec<&str>>()
            .concat()
    }

    fn right_payload(&self) -> String {
        self.right_digits()
            .iter()
            .map(|d| self.char_encoding(1, &d))
            .collect::<Vec<&str>>()
            .concat()
    }
}

impl Parse for UPCA {
    fn valid_len() -> Range<u32> {
        11..12
    }

    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

impl Encode for UPCA {
    fn encode(&self) -> String {
        format!("{}{}{}{}{}{}", GUARDS[0], self.left_payload(), GUARDS[1], self.right_payload(), self.checksum_encoding(), GUARDS[2])
    }
}

#[cfg(test)]
mod tests {
    use ::sym::upca::*;
    use ::generators::ascii::*;
    use ::sym::Encode;

    #[test]
    fn new_upca() {
        let upca = UPCA::new("123456123456".to_string());

        assert!(upca.is_ok());
    }

    #[test]
    fn invalid_data_upca() {
        let upca = UPCA::new("1234er123412".to_string());

        assert!(upca.is_err());
    }

    #[test]
    fn invalid_len_upca() {
        let upca = UPCA::new("1111112222222333333".to_string());

        assert!(upca.is_err());
    }

    #[test]
    fn upca_raw_data() {
        let upca = UPCA::new("123456123456".to_string()).unwrap();

        assert_eq!(upca.raw_data(), "123456123456".to_string());
    }

    #[test]
    fn upca_encode() {
        let upca1 = UPCA::new("12345612345".to_string()).unwrap();
        let upca2 = UPCA::new("00118999561".to_string()).unwrap();

        assert_eq!(upca1.encode(), "10100110010010011011110101000110110001010111101010110011011011001000010101110010011101101100101".to_string());
        assert_eq!(upca2.encode(), "10100011010001101001100100110010110111000101101010111010011101001001110101000011001101101100101".to_string());
    }

    #[test]
    fn upca_checksum_calculation() {
        let upca1 = UPCA::new("03600029145".to_string()).unwrap();
        let two_encoding = ENCODINGS[1][2];
        let checksum_digit = &upca1.encode()[85..92];


        assert_eq!(checksum_digit, two_encoding);
    }

    #[test]
    fn upca_to_ascii() {
        let upca = UPCA::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new();

        assert_eq!(ascii.generate(&upca), "SWAG".to_string());
    }

    #[test]
    fn upca_to_ascii_with_large_height() {
        let upca = UPCA::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new().height(40).xdim(2);

        assert_eq!(ascii.height, 40);
        assert_eq!(ascii.xdim, 2);
        assert_eq!(ascii.generate(&upca), "SWAG".to_string());
    }
}
