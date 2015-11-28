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

    /// Generates the given barcode. Returns a `Result<String, &str>` of the SVG data or an
    /// error message.
    pub fn generate(&self, barcode: &[u8], path: &mut File) -> Result<String, &str> {
        let rects = "<rect x=\"0\" y=\"0\" width=\"60\" height=\"80\" fill=\"blue\" />
                     <rect x=\"60\" y=\"0\" width=\"40\" height=\"80\" fill=\"green\" />";
        let svg = format!("<svg version=\"1.1\" viewBox=\"0 0 {w} {h}\">
                             {r}
                           </svg>", w=100, h=80, r=rects);
        Ok(svg)
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::generators::svg::*;
    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::fs::File;
    use std::path::Path;

    const TEST_DATA_BASE: &'static str = "./target/debug";
    const WRITE_TO_FILE: bool = true;

    fn write_file(data: &str, file: &'static str) {
        let path = open_file(file);
        let mut writer = BufWriter::new(path);
        writer.write(data.as_bytes()).unwrap();
    }

    fn open_file(name: &'static str) -> File {
        File::create(&Path::new(&format!("{}/{}", TEST_DATA_BASE, name)[..])).unwrap()
    }

    #[test]
    fn ean_13_as_svg() {
        let mut path = open_file("ean13.svg");

        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let svg = SVG::new();
        let generated = svg.generate(&ean13.encode()[..], &mut path).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean13.svg"); }

//        assert_eq!(generated, "swag".to_owned());
    }
}
