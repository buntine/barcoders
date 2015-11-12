use ::sym::EncodedBarcode;
use std::iter::repeat;

pub struct ASCII {
    pub height: usize,
    pub xdim: usize,
}

pub const ASCII_CHARS: [char; 2] = [' ', '#'];

impl ASCII {
    pub fn new() -> ASCII {
        ASCII{height: 10, xdim: 1}
    }

    pub fn height(mut self, h: usize) -> ASCII {
        self.height = h;
        self
    }

    pub fn xdim(mut self, x: usize) -> ASCII {
        self.xdim = x;
        self
    }

    fn generate_row(&self, barcode: &EncodedBarcode) -> String {
        barcode.iter()
               .flat_map(|&d| repeat(ASCII_CHARS[d as usize]).take(self.xdim))
               .collect()
    }

    pub fn generate(&self, barcode: &EncodedBarcode) -> Result<String, String> {
        let mut output = String::new();
        let row = self.generate_row(&barcode);

        for (i, _l) in (0..self.height).enumerate() {
            output.push_str(&row[..]);

            if i < self.height - 1 {
                output.push_str("\n");
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    //use ::sym::ean8::*;
   // use ::sym::code39::*;
    use ::generators::ascii::*;
    use ::sym::Encode;

    #[test]
    fn ean_13_as_ascii() {
        let ean13 = EAN13::new("750103131130".to_string()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&ean13.encode()).unwrap();

        assert_eq!(generated,
"
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
".trim().to_string());
    }

    #[test]
    fn ean_13_as_ascii_small_height_double_width() {
        let ean13 = EAN13::new("750103131130".to_string()).unwrap();
        let ascii = ASCII::new().height(6).xdim(2);
        let generated = ascii.generate(&ean13.encode()).unwrap();

        assert_eq!(generated,
"
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
".trim().to_string());
    }

}
 
