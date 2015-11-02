pub mod upca;
pub mod code39;

pub trait ToASCII {
    fn to_ascii(&self) -> String;
}

pub trait Parse {
    fn valid_chars() -> Vec<char>;

    fn parse(data: String) -> Result<String, String> {
        let valid_chars = Self::valid_chars();
        let bad_chars: Vec<char> = data.chars().filter(|c| !valid_chars.binary_search(&c).is_ok() ).collect();

        if bad_chars.len() > 0 {
            Err(format!("Invalid characters: {:?}", bad_chars))
        } else {
            Ok(data)
        }
    }
}
