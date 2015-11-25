//! This module provides types for generating ASCII representations of barcodes. This is useful for
//! testing and simple verification of barcode correctness.

use std::iter::repeat;

/// The ASCII barcode generator type.
#[derive(Copy, Clone, Debug)]
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
        ASCII {
            height: 10,
            xdim: 1,
        }
    }

    fn generate_row(&self, barcode: &[u8]) -> String {
        barcode.iter()
               .flat_map(|&d| repeat(ASCII_CHARS[d as usize]).take(self.xdim))
               .collect()
    }

    /// Generates the given barcode. Returns a String.
    pub fn generate(&self, barcode: &[u8]) -> Result<String, &str> {
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
    use ::sym::ean_supp::*;
    use ::sym::code39::*;
    use ::sym::tf::*;
    use ::generators::ascii::*;

    #[test]
    fn ean_13_as_ascii() {
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&ean13.encode()[..]).unwrap();

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
".trim().to_owned());
    }

    #[test]
    fn ean_13_as_ascii_small_height_double_width() {
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let ascii = ASCII{height: 6, xdim: 2};
        let generated = ascii.generate(&ean13.encode()[..]).unwrap();

        assert_eq!(generated,
"
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
##  ##  ####      ##  ##    ######    ####    ##  ##    ######  ########  ##  ####    ####  ##  ##  ##        ##  ####    ####  ####    ####  ##        ##  ######    ##  ######  ##    ##  ##
".trim().to_owned());
    }

    #[test]
    fn ean_8_as_ascii() {
        let ean8 = EAN8::new("1234567".to_owned()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&ean8.encode()[..]).unwrap();

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
".trim().to_owned());
    }

    #[test]
    fn ean_8_as_ascii_small_height_double_width() {
        let ean8 = EAN8::new("1234567".to_owned()).unwrap();
        let ascii = ASCII{height: 5, xdim: 2};
        let generated = ascii.generate(&ean8.encode()[..]).unwrap();

        assert_eq!(generated,
"
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
##  ##    ####    ##    ##    ####  ########  ##  ##      ####  ##  ##  ##    ######  ##  ##        ##      ##    ######    ##  ##  ##
".trim().to_owned());
    }

    #[test]
    fn code_39_as_ascii() {
        let code39 = Code39::new("TEST8052".to_owned()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&code39.encode()[..]).unwrap();

        assert_eq!(generated,
"
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
#  # ## ## # # # ## ##  # ## # ##  # # # ## # ##  # # # ## ##  # ## #  # ## # # #  ## ## # ## #  ## # # # ##  # # ## #  # ## ## #
".trim().to_owned());
    }

    #[test]
    fn code_39_as_ascii_small_height_double_weight() {
        let code39 = Code39::new("1234".to_owned()).unwrap();
        let ascii = ASCII{height: 7, xdim: 2};
        let generated = ascii.generate(&code39.encode()[..]).unwrap();

        assert_eq!(generated,
"
##    ##  ####  ####  ##  ####  ##    ##  ##  ####  ##  ####    ##  ##  ####  ####  ####    ##  ##  ##  ##  ##    ####  ##  ####  ##    ##  ####  ####  ##
##    ##  ####  ####  ##  ####  ##    ##  ##  ####  ##  ####    ##  ##  ####  ####  ####    ##  ##  ##  ##  ##    ####  ##  ####  ##    ##  ####  ####  ##
##    ##  ####  ####  ##  ####  ##    ##  ##  ####  ##  ####    ##  ##  ####  ####  ####    ##  ##  ##  ##  ##    ####  ##  ####  ##    ##  ####  ####  ##
##    ##  ####  ####  ##  ####  ##    ##  ##  ####  ##  ####    ##  ##  ####  ####  ####    ##  ##  ##  ##  ##    ####  ##  ####  ##    ##  ####  ####  ##
##    ##  ####  ####  ##  ####  ##    ##  ##  ####  ##  ####    ##  ##  ####  ####  ####    ##  ##  ##  ##  ##    ####  ##  ####  ##    ##  ####  ####  ##
##    ##  ####  ####  ##  ####  ##    ##  ##  ####  ##  ####    ##  ##  ####  ####  ####    ##  ##  ##  ##  ##    ####  ##  ####  ##    ##  ####  ####  ##
##    ##  ####  ####  ##  ####  ##    ##  ##  ####  ##  ####    ##  ##  ####  ####  ####    ##  ##  ##  ##  ##    ####  ##  ####  ##    ##  ####  ####  ##
".trim().to_owned());
    }

    #[test]
    fn ean2_as_ascii() {
        let ean2 = EANSUPP::new("34".to_owned()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&ean2.encode()[..]).unwrap();

        assert_eq!(generated,
"
# ## #    # # #   ##
# ## #    # # #   ##
# ## #    # # #   ##
# ## #    # # #   ##
# ## #    # # #   ##
# ## #    # # #   ##
# ## #    # # #   ##
# ## #    # # #   ##
# ## #    # # #   ##
# ## #    # # #   ##
".trim().to_owned());
    }

    #[test]
    fn ean5_as_ascii() {
        let ean5 = EANSUPP::new("50799".to_owned()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&ean5.encode()[..]).unwrap();

        assert_eq!(generated,
"
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
# ## ##   # # #  ### #  #   # #   # ## #   # ##
".trim().to_owned());
    }

    #[test]
    fn itf_as_ascii() {
        let itf = TF::interleaved("12345".to_owned()).unwrap();
        let ascii = ASCII::new();
        let generated = ascii.generate(&itf.encode()[..]).unwrap();

        assert_eq!(generated,
"
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
# # ### #   # # ###   ### ### #   # #   ### # ### #   #   ## #
".trim().to_owned());
    }
}
