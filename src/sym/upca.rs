use ::sym::ToASCII;

pub struct UPCA {
    pub data: String,
}

impl UPCA {

    pub fn new(data: String) -> UPCA {
        UPCA{data: data}
    }

}

impl ToASCII for UPCA {
    fn to_ascii(&self) -> String {
        "SWAG".to_string()
    }
}
