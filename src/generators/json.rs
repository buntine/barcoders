//! Functionality for generating JSON representations of barcodes.
//!
//! This is useful for passing encoded data to third-party systems in a conventional format.
//!
//! Output will be of the format:
//! ```javascript
//! {
//!   "height": 10,
//!   "xdim": 1,
//!   "encoding": [1, 0, 0, 1, 1, 0, ...],
//! }
//! ```

use error::Result;

/// The JSON  barcode generator type.
#[derive(Copy, Clone, Debug)]
pub struct JSON {
    /// The height of the barcode.
    pub height: usize,
    /// The X dimension. Specifies the width of the "narrow" bars. 
    pub xdim: usize,
}

impl JSON {
    /// Returns a new JSON with default values.
    pub fn new() -> JSON {
        JSON {
            height: 10,
            xdim: 1,
        }
    }

    /// Generates the given barcode. Returns a `Result<String, Error>` indicating success.
    pub fn generate<T: AsRef<[u8]>>(&self, barcode: T) -> Result<String> {
        let mut bits = barcode
            .as_ref()
            .iter()
            .fold(String::new(), |acc, &b| {
                let n = match b {
                    0 => "0",
                    _ => "1",
                };

                acc + n + ","
            });

        // Kill trailing comma.
        bits.pop();

        let output = format!("{{\"height\":{},\"xdim\":{},\"encoding\":[{}]}}",
                             self.height, self.xdim, bits);

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::sym::ean8::*;
    use ::sym::ean_supp::*;
    use ::sym::code39::*;
    use ::sym::code128::*;
    use ::sym::tf::*;
    use ::sym::codabar::*;
    use ::generators::json::*;

    #[test]
    fn ean_13_as_json() {
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let json = JSON::new();
        let generated = json.generate(&ean13.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":10,\"xdim\":1,\"encoding\":[1,0,1,0,1,1,0,0,0,1,0,1,0,0,1,1,1,0,0,1,1,0,0,1,0,1,0,0,1,1,1,0,1,1,1,1,0,1,0,1,1,0,0,1,1,0,1,0,1,0,1,0,0,0,0,1,0,1,1,0,0,1,1,0,1,1,0,0,1,1,0,1,0,0,0,0,1,0,1,1,1,0,0,1,0,1,1,1,0,1,0,0,1,0,1]}".trim().to_owned());
    }

    #[test]
    fn ean_13_as_json_small_height_double_width() {
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let json = JSON{height: 6, xdim: 2};
        let generated = json.generate(&ean13.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":6,\"xdim\":2,\"encoding\":[1,0,1,0,1,1,0,0,0,1,0,1,0,0,1,1,1,0,0,1,1,0,0,1,0,1,0,0,1,1,1,0,1,1,1,1,0,1,0,1,1,0,0,1,1,0,1,0,1,0,1,0,0,0,0,1,0,1,1,0,0,1,1,0,1,1,0,0,1,1,0,1,0,0,0,0,1,0,1,1,1,0,0,1,0,1,1,1,0,1,0,0,1,0,1]}".trim().to_owned());
    }

    #[test]
    fn ean_8_as_json() {
        let ean8 = EAN8::new("1234567".to_owned()).unwrap();
        let json = JSON::new();
        let generated = json.generate(&ean8.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":10,\"xdim\":1,\"encoding\":[1,0,1,0,0,1,1,0,0,1,0,0,1,0,0,1,1,0,1,1,1,1,0,1,0,1,0,0,0,1,1,0,1,0,1,0,1,0,0,1,1,1,0,1,0,1,0,0,0,0,1,0,0,0,1,0,0,1,1,1,0,0,1,0,1,0,1]}".trim().to_owned());
    }

    #[test]
    fn ean_8_as_json_small_height_double_width() {
        let ean8 = EAN8::new("1234567".to_owned()).unwrap();
        let json = JSON{height: 5, xdim: 2};
        let generated = json.generate(&ean8.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":5,\"xdim\":2,\"encoding\":[1,0,1,0,0,1,1,0,0,1,0,0,1,0,0,1,1,0,1,1,1,1,0,1,0,1,0,0,0,1,1,0,1,0,1,0,1,0,0,1,1,1,0,1,0,1,0,0,0,0,1,0,0,0,1,0,0,1,1,1,0,0,1,0,1,0,1]}".trim().to_owned());
    }

    #[test]
    fn code_39_as_json() {
        let code39 = Code39::new("TEST8052".to_owned()).unwrap();
        let json = JSON::new();
        let generated = json.generate(&code39.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":10,\"xdim\":1,\"encoding\":[1,0,0,1,0,1,1,0,1,1,0,1,0,1,0,1,0,1,1,0,1,1,0,0,1,0,1,1,0,1,0,1,1,0,0,1,0,1,0,1,0,1,1,0,1,0,1,1,0,0,1,0,1,0,1,0,1,1,0,1,1,0,0,1,0,1,1,0,1,0,0,1,0,1,1,0,1,0,1,0,1,0,0,1,1,0,1,1,0,1,0,1,1,0,1,0,0,1,1,0,1,0,1,0,1,0,1,1,0,0,1,0,1,0,1,1,0,1,0,0,1,0,1,1,0,1,1,0,1]}".trim().to_owned());
    }

    #[test]
    fn code_39_as_json_small_height_double_weight() {
        let code39 = Code39::new("1234".to_owned()).unwrap();
        let json = JSON{height: 7, xdim: 2};
        let generated = json.generate(&code39.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":7,\"xdim\":2,\"encoding\":[1,0,0,1,0,1,1,0,1,1,0,1,0,1,1,0,1,0,0,1,0,1,0,1,1,0,1,0,1,1,0,0,1,0,1,0,1,1,0,1,1,0,1,1,0,0,1,0,1,0,1,0,1,0,1,0,0,1,1,0,1,0,1,1,0,1,0,0,1,0,1,1,0,1,1,0,1]}".trim().to_owned());
    }

    #[test]
    fn codabar_as_json() {
        let codabar = Codabar::new("A98B".to_owned()).unwrap();
        let json = JSON::new();
        let generated = json.generate(&codabar.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":10,\"xdim\":1,\"encoding\":[1,0,1,1,0,0,1,0,0,1,0,1,1,0,1,0,0,1,0,1,0,1,0,0,1,1,0,1,0,1,0,1,0,1,0,0,1,0,0,1,1]}".trim().to_owned());
    }

    #[test]
    fn codabar_as_json_small_height_double_weight() {
        let codabar = Codabar::new("A40156B".to_owned()).unwrap();
        let json = JSON{height: 7, xdim: 2};
        let generated = json.generate(&codabar.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":7,\"xdim\":2,\"encoding\":[1,0,1,1,0,0,1,0,0,1,0,1,0,1,1,0,1,0,0,1,0,1,0,1,0,1,0,0,1,1,0,1,0,1,0,1,1,0,0,1,0,1,1,0,1,0,1,0,0,1,0,1,0,0,1,0,1,0,1,1,0,1,0,1,0,0,1,0,0,1,1]}".trim().to_owned());
    }

    #[test]
    fn code_128_as_json() {
        let code128 = Code128::new("ÀHELLO".to_owned()).unwrap();
        let json = JSON::new();
        let generated = json.generate(&code128.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":10,\"xdim\":1,\"encoding\":[1,1,0,1,0,0,0,0,1,0,0,1,1,0,0,0,1,0,1,0,0,0,1,0,0,0,1,1,0,1,0,0,0,1,0,0,0,1,1,0,1,1,1,0,1,0,0,0,1,1,0,1,1,1,0,1,0,0,0,1,1,1,0,1,1,0,1,1,0,1,0,0,0,1,0,0,0,1,1,0,0,0,1,1,1,0,1,0,1,1]}".trim().to_owned());
    }

    #[test]
    fn code_128_as_json_small_height_double_weight() {
        let code128 = Code128::new("ÀHELLO".to_owned()).unwrap();
        let json = JSON{height: 7, xdim: 2};
        let generated = json.generate(&code128.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":7,\"xdim\":2,\"encoding\":[1,1,0,1,0,0,0,0,1,0,0,1,1,0,0,0,1,0,1,0,0,0,1,0,0,0,1,1,0,1,0,0,0,1,0,0,0,1,1,0,1,1,1,0,1,0,0,0,1,1,0,1,1,1,0,1,0,0,0,1,1,1,0,1,1,0,1,1,0,1,0,0,0,1,0,0,0,1,1,0,0,0,1,1,1,0,1,0,1,1]}".trim().to_owned());
    }

    #[test]
    fn ean2_as_json() {
        let ean2 = EANSUPP::new("34".to_owned()).unwrap();
        let json = JSON::new();
        let generated = json.generate(&ean2.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":10,\"xdim\":1,\"encoding\":[1,0,1,1,0,1,0,0,0,0,1,0,1,0,1,0,0,0,1,1]}".trim().to_owned());
    }

    #[test]
    fn ean5_as_json() {
        let ean5 = EANSUPP::new("50799".to_owned()).unwrap();
        let json = JSON::new();
        let generated = json.generate(&ean5.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":10,\"xdim\":1,\"encoding\":[1,0,1,1,0,1,1,0,0,0,1,0,1,0,1,0,0,1,1,1,0,1,0,0,1,0,0,0,1,0,1,0,0,0,1,0,1,1,0,1,0,0,0,1,0,1,1]}".trim().to_owned());
    }

    #[test]
    fn itf_as_json() {
        let itf = TF::interleaved("12345".to_owned()).unwrap();
        let json = JSON::new();
        let generated = json.generate(&itf.encode()[..]).unwrap();

        assert_eq!(generated, "{\"height\":10,\"xdim\":1,\"encoding\":[1,0,1,0,1,1,1,0,1,0,0,0,1,0,1,0,1,1,1,0,0,0,1,1,1,0,1,1,1,0,1,0,0,0,1,0,1,0,0,0,1,1,1,0,1,0,1,1,1,0,1,0,0,0,1,0,0,0,1,1,0,1]}".trim().to_owned());
    }
}
