use ::sym::EncodedBarcode;

pub struct ASCII {
    pub height: u32,
    pub xdim: u32,
}

impl ASCII {
    pub fn new() -> ASCII {
        ASCII{height: 10, xdim: 1}
    }

    pub fn height(mut self, h: u32) -> ASCII {
        self.height = h;
        self
    }

    pub fn xdim(mut self, x: u32) -> ASCII {
        self.xdim = x;
        self
    }

    fn generate_row(&self, barcode: &EncodedBarcode) -> String {
        "##  ".to_string()
    }

    // TODO: Implement.
    pub fn generate(&self, barcode: &EncodedBarcode) -> Result<String, String> {
        let mut output = String::new();
        let row = self.generate_row(&barcode);

        for _l in 0..self.height {
            output.push_str(&row[..]);
            output.push_str("\n");
        }

        Ok(output)
    }
}
