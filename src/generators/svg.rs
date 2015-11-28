//! Functionality for generating SVG representations of barcodes.

/// The SVG barcode generator type.
#[derive(Copy, Clone, Debug)]
pub struct SVG {
    /// The height of the barcode (```self.height``` pixels high for SVG).
    pub height: u32,
    /// The X dimension. Specifies the width of the "narrow" bars. 
    /// For SVG, each will be ```self.xdim``` pixels wide.
    pub xdim: u32,
}

impl SVG {
    /// Returns a new SVG with default values.
    pub fn new() -> SVG {
        SVG {
            height: 80,
            xdim: 1,
        }
    }

    fn rect(&self, style: u8, offset: usize) -> String {
        let fill = match style {
            1 => "black",
            _ => "white",
        };

        format!("<rect x=\"{}\" y=\"0\" width=\"1\" height=\"{}\" fill=\"{}\" />",
                offset, self.height, fill)
    }

    /// Generates the given barcode. Returns a `Result<String, &str>` of the SVG data or an
    /// error message.
    pub fn generate(&self, barcode: &[u8]) -> Result<String, &str> {
        let width = (barcode.len() as u32) * self.xdim;
        let rects: String = barcode.iter()
                           .enumerate()
                           .map(|(i, &n)| self.rect(n, i))
                           .collect();
        let svg = format!("<svg version=\"1.1\" viewBox=\"0 0 {w} {h}\">
                             {r}
                           </svg>", w=width, h=self.height, r=rects);
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
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let svg = SVG::new();
        let generated = svg.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean13.svg"); }

        assert_eq!(generated.len(), 5413);
    }
}
