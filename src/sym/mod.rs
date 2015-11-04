pub mod upca;
pub mod code39;

use std::ops::Range;

pub trait Parse {
    fn valid_chars() -> Vec<char>;
    fn valid_len() -> Range<u32>;

    fn parse(data: String) -> Result<String, String> {
        let valid_chars = Self::valid_chars();
        let valid_len = Self::valid_len();
        let data_len = data.len() as u32;

        if data_len < valid_len.start || data_len > valid_len.end {
            return Err(format!("Data does not fit within range of {}-{}", valid_len.start, valid_len.end - 1));
        }

        let bad_chars: Vec<char> = data.chars().filter(|c| !valid_chars.binary_search(&c).is_ok() ).collect();

        if bad_chars.is_empty() {
            Ok(data)
        } else {
            Err(format!("Invalid characters: {:?}", bad_chars))
        }
    }
}

pub trait Encode {
    fn encode(&self) -> String;
}
