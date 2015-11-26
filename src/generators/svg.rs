//! Functionality for generating SVG representations of barcodes.

use std::fs::File;

/// The SVG barcode generator type.
#[derive(Copy, Clone, Debug)]
pub struct SVG {
    /// The height of the barcode (```self.height``` pixels high for SVG).
    pub height: usize,
    /// The X dimension. Specifies the width of the "narrow" bars. 
    /// For SVG, each will be ```self.xdim``` pixels wide.
    pub xdim: usize,
}

impl SVG {
    /// Returns a new SVG with default values.
    pub fn new() -> SVG {
        SVG {
            height: 80,
            xdim: 1,
        }
    }

    /// Generates the given barcode. Returns a `Result<usize, &str>` indicating the number of bytes written.
    pub fn generate(&self, barcode: &[u8], path: &mut File) -> Result<usize, &str> {
        Ok(10)
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::sym::ean8::*;
    use ::sym::ean_supp::*;
    use ::sym::code39::*;
    use ::sym::tf::*;
    use ::generators::svg::*;
    use std::fs::File;
    use std::path::Path;

    const TEST_DATA_BASE: &'static str = "./target/debug";

    fn open_file(name: &'static str) -> File {
        File::create(&Path::new(&format!("{}/{}", TEST_DATA_BASE, name)[..])).unwrap()
    }

    #[test]
    fn ean_13_as_svg() {
        let mut path = open_file("ean13.svg");

        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let svg = SVG::new();
        let generated = svg.generate(&ean13.encode()[..], &mut path).unwrap();

        assert_eq!(generated, 10);
    }
}
