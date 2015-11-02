pub mod upca;
pub mod code39;

use std::ops::Range;

pub trait ToASCII {
    fn to_ascii(&self) -> String;
}

pub trait Parse {
    fn valid_chars() -> Vec<char>;
    fn valid_len() -> Range<i32>;

    fn parse(data: String) -> Result<String, String> {
        let valid_chars = Self::valid_chars();
        let valid_len = Self::valid_len();
        let data_len = data.len() as i32;

        if data_len < valid_len.start || data_len > valid_len.end {
            return Err(format!("Size of {} does not fit within range of {:?}", data_len, valid_len));
        }

        let bad_chars: Vec<char> = data.chars().filter(|c| !valid_chars.binary_search(&c).is_ok() ).collect();

        if bad_chars.len() > 0 {
            Err(format!("Invalid characters: {:?}", bad_chars))
        } else {
            Ok(data)
        }
    }
}
