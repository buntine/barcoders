use ::sym::Encode;

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

    // TODO: Implement.
    pub fn generate<T: Encode>(&self, barcode: &T) -> String {
        let payload = barcode.encode();
        let mut output = String::new();

        for _l in 0..self.height {
            for d in &payload {
                match d {
                    &0 => output.push_str(" "),
                    _ => output.push_str("#"),
                }
            }
            output.push_str("\n");
        }

        output
    }
}
