use ::sym::ToASCII;

const VALID_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct Code39 {
    pub data: String,
}

impl Code39 {
    pub fn parse(data: String) -> Result<Code39, String> {
        let bad_chars: Vec<char> = data.chars().filter(|c| !VALID_CHARS.binary_search(&c).is_ok() ).collect();

        if bad_chars.len() > 0 {
            Err(format!("Invalid characters: {:?}", bad_chars))
        } else {
            Ok(Code39{data: data})
        }
    }
}

impl ToASCII for Code39 {
    fn to_ascii(&self) -> String {
        "SWOLE".to_string()
    }
}
