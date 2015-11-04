use ::sym::Encode;

pub struct ASCII {
    pub height: u32,
    pub xdim: u32,
    pub ydim: u32,
}

impl ASCII {
    pub fn new() -> ASCII {
        ASCII{height: 10, xdim: 1, ydim: 1}
    }

    pub fn generate<T: Encode>(&self, barcode: &T) -> String {
        "SWAG".to_string()
    }
}
