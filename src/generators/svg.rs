//! Functionality for generating SVG representations of barcodes.

use error::Result;

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

    fn rect(&self, style: u8, offset: u32, width: u32) -> String {
        let fill = match style {
            1 => "black",
            _ => "white",
        };

        format!("<rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />",
                offset, width, self.height, fill)
    }

    /// Generates the given barcode. Returns a `Result<String, Error>` of the SVG data or an
    /// error message.
    pub fn generate<T: AsRef<[u8]>>(&self, barcode: T) -> Result<String> {
        let barcode = barcode.as_ref();
        let width = (barcode.len() as u32) * self.xdim;
        let rects: String = barcode.iter()
                           .enumerate()
                           .filter(|&(_, &n)| n == 1)
                           .map(|(i, &n)| self.rect(n, (i as u32 * self.xdim), self.xdim))
                           .collect();
        let svg = format!("<svg version=\"1.1\" viewBox=\"0 0 {w} {h}\">
                             {s}
                             {r}
                           </svg>", w=width, h=self.height, s=self.rect(0, 0, width), r=rects);

        Ok(svg)
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::sym::ean8::*;
    use ::sym::code39::*;
    use ::sym::code128::*;
    use ::sym::ean_supp::*;
    use ::sym::tf::*;
    use ::sym::codabar::*;
    use ::generators::svg::*;
    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::fs::File;
    use std::path::Path;

    const TEST_DATA_BASE: &'static str = "./target/debug";
    const WRITE_TO_FILE: bool = false;

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

        assert_eq!(generated.len(), 2928);
    }

    #[test]
    fn ean_8_as_svg() {
        let ean8 = EAN8::new("9998823".to_owned()).unwrap();
        let svg = SVG::new();
        let generated = svg.generate(&ean8.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean8.svg"); }

        assert_eq!(generated.len(), 1976);
    }

    #[test]
    fn code39_as_svg() {
        let code39 = Code39::new("IGOT99PROBLEMS".to_owned()).unwrap();
        let svg = SVG::new();
        let generated = svg.generate(&code39.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "code39.svg"); }

        assert_eq!(generated.len(), 6514);
    }

    #[test]
    fn codabar_as_svg() {
        let codabar = Codabar::new("A12----34A".to_owned()).unwrap();
        let svg = SVG::new();
        let generated = svg.generate(&codabar.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "codabar.svg"); }

        assert_eq!(generated.len(), 2987);
    }

    #[test]
    fn code128_as_svg() {
        let code128 = Code128::new("ÀHIĆ345678".to_owned()).unwrap();
        let svg = SVG::new();
        let generated = svg.generate(&code128.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "code128.svg"); }

        assert_eq!(generated.len(), 2764);
    }

    #[test]
    fn ean_2_as_svg() {
        let ean2 = EANSUPP::new("78".to_owned()).unwrap();
        let svg = SVG::new();
        let generated = svg.generate(&ean2.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean2.svg"); }

        assert_eq!(generated.len(), 801);
    }

    #[test]
    fn itf_as_svg() {
        let itf = TF::interleaved("1234123488993344556677118".to_owned()).unwrap();
        let svg = SVG{height: 200, xdim:3};
        let generated = svg.generate(&itf.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "itf.svg"); }

        assert_eq!(generated.len(), 7249);
    }
}
