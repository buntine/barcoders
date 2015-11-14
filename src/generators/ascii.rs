//! This module provides types for generating ASCII representations of barcodes. This is useful for
//! testing and simple verification of barcode correctness.

use ::sym::EncodedBarcode;
use std::iter::repeat;

/// The ASCII barcode generator type.
pub struct ASCII {
    /// The height of the barcode (```self.height``` characters high for ASCII).
    pub height: usize,
    /// The X dimension. Specifies the width of the "narrow" bars. 
    /// For ASCII, each will be ```self.xdim``` characters wide.
    pub xdim: usize,
}

/// Maps binary digits to ASCII representation (0=' ', 1='#')
pub const ASCII_CHARS: [char; 2] = [' ', '#'];

impl ASCII {
    /// Returns a new ASCII with default values.
    pub fn new() -> ASCII {
        ASCII{height: 10, xdim: 1}
    }

    /// Sets the height of the barcode and returns self.
    pub fn height(mut self, h: usize) -> ASCII {
        self.height = h;
        self
    }

    /// Sets the X dimension of the barcode and returns self.
    pub fn xdim(mut self, x: usize) -> ASCII {
        self.xdim = x;
        self
    }

    fn generate_row(&self, barcode: &EncodedBarcode) -> String {
        barcode.iter()
               .flat_map(|&d| repeat(ASCII_CHARS[d as usize]).take(self.xdim))
               .collect()
    }

    /// Generates the given EncodedBarcode. Returns a String.
    pub fn generate(&self, barcode: &EncodedBarcode) -> Result<String, &str> {
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
    use ::sym::ean8::*;
    use ::sym::code39::*;
    use ::generators::ascii::*;

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

    #[test]
    fn ean_8_as_ascii() {
        let ean8 = EAN8::new("1234567".to_string()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&ean8.encode()).unwrap();

        assert_eq!(generated,
"
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
# #  ##  #  #  ## #### # #   ## # # #  ### # #    #   #  ###  # # #
".trim().to_string());
    }

    #[test]
    fn ean_8_as_ascii_small_height_double_width() {
        let ean8 = EAN8::new("1234567".to_string()).unwrap();
        let ascii = ASCII::new().height(5).xdim(2);
        let generated = ascii.generate(&ean8.encode()).unwrap();

        assert_eq!(generated,
"
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
".trim().to_string());
    }

    #[test]
    fn code_39_as_ascii() {
        let code39 = Code39::new("TEST8052".to_string()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&code39.encode()).unwrap();

        assert_eq!(generated,
"
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
#  # ## ## ## # ## ##  ### # ##  # ## ## # ##  ## # ## ##  ### #  # ## ## #  ## ## ### #  ## # ## ##  # # ###  # ## ## #
".trim().to_string());
    }

    #[test]
    fn code_39_as_ascii_small_height_double_weight() {
        let code39 = Code39::new("1234".to_string()).unwrap();
        let ascii = ASCII::new().height(7).xdim(2);
        let generated = ascii.generate(&code39.encode()).unwrap();

        assert_eq!(generated,
"
##    ##  ####  ####  ######  ##    ##  ##  ######  ####    ##  ##  ########  ####    ##  ##  ####  ##    ####  ##  ######    ##  ####  ####  ##
##    ##  ####  ####  ######  ##    ##  ##  ######  ####    ##  ##  ########  ####    ##  ##  ####  ##    ####  ##  ######    ##  ####  ####  ##
##    ##  ####  ####  ######  ##    ##  ##  ######  ####    ##  ##  ########  ####    ##  ##  ####  ##    ####  ##  ######    ##  ####  ####  ##
##    ##  ####  ####  ######  ##    ##  ##  ######  ####    ##  ##  ########  ####    ##  ##  ####  ##    ####  ##  ######    ##  ####  ####  ##
##    ##  ####  ####  ######  ##    ##  ##  ######  ####    ##  ##  ########  ####    ##  ##  ####  ##    ####  ##  ######    ##  ####  ####  ##
##    ##  ####  ####  ######  ##    ##  ##  ######  ####    ##  ##  ########  ####    ##  ##  ####  ##    ####  ##  ######    ##  ####  ####  ##
##    ##  ####  ####  ######  ##    ##  ##  ######  ####    ##  ##  ########  ####    ##  ##  ####  ##    ####  ##  ######    ##  ####  ####  ##
".trim().to_string());
    }
}
