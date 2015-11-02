use ::sym::ToASCII;

pub struct Code39 {
    pub data: String,
}

impl Code39 {

    pub fn parse<'a>(data: String) -> Result<Code39, &'a str> {
        Ok(Code39{data: data})
    }

}

impl ToASCII for Code39 {
    fn to_ascii(&self) -> String {
        "SWOLE".to_string()
    }
}
