use ::sym::ToASCII;

pub struct UPCA {
    pub data: String,
}

impl UPCA {

    pub fn parse<'a>(data: String) -> Result<UPCA, &'a str> {
        Ok(UPCA{data: data})
    }

}

impl ToASCII for UPCA {
    fn to_ascii(&self) -> String {
        "SWAG".to_string()
    }
}
