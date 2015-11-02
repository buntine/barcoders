use ::sym::ToASCII;

const VALID_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct UPCA {
    pub data: String,
}

impl UPCA {
    pub fn parse(data: String) -> Result<UPCA, String> {
        Ok(UPCA{data: data})
    }
}

impl ToASCII for UPCA {
    fn to_ascii(&self) -> String {
        "SWAG".to_string()
    }
}
