//! Functionality for generating image representations of barcodes.
//!
//! Each enum variant can be constructed via the standard constructor pattern
//! or via a constructor method if you want default values.
//!
//! For example:
//!
//! ```rust
//! use barcoders::generators::image::*;
//!
//! // Specify your own struct fields.
//! let png = Image::PNG{height: 80, xdim: 1};
//!
//! // Or use the constructor for defaults.
//! let png = Image::png();
//! ```
//!
//! See the README for more examples.

extern crate image;

use image::GenericImage;
use image::ImageBuffer;
use std::fs::File;

/// The image generator type.
#[derive(Copy, Clone, Debug)]
pub enum Image {
    /// GIF image generator type.
    GIF {
        /// The height of the barcode in pixels.
        height: u32,
        /// The X dimension. Specifies the width of the "narrow" bars. 
        /// For GIF, each will be ```self.xdim``` pixels wide.
        xdim: u32,
    },
    /// PNG image generator type.
    PNG {
        /// The height of the barcode in pixels.
        height: u32,
        /// The X dimension. Specifies the width of the "narrow" bars. 
        /// For PNG, each will be ```self.xdim``` pixels wide.
        xdim: u32,
    },
    /// JPEG image generator type.
    JPEG {
        /// The height of the barcode in pixels.
        height: u32,
        /// The X dimension. Specifies the width of the "narrow" bars. 
        /// For JPEG, each will be ```self.xdim``` pixels wide.
        xdim: u32,
    },
}

impl Image {
    /// Returns a new GIF with default values.
    pub fn gif() -> Image {
        Image::GIF {
            height: 80,
            xdim: 1,
        }
    }

    /// Returns a new PNG with default values.
    pub fn png() -> Image {
        Image::PNG {
            height: 80,
            xdim: 1,
        }
    }

    /// Returns a new PNG with default values.
    pub fn jpeg() -> Image {
        Image::JPEG {
            height: 80,
            xdim: 1,
        }
    }

    /// Generates the given barcode. Returns a `Result<Vec<u8>, &str>` of the encoded bytes.
    pub fn generate(&self, barcode: &[u8]) -> Result<Vec<u8>, &str> {
        let (xdim, height, format) = match *self {
            Image::GIF{height: h, xdim: x} => (x, h, image::GIF),
            Image::PNG{height: h, xdim: x} => (x, h, image::PNG),
            Image::JPEG{height: h, xdim: x} => (x, h, image::JPEG),
        };

        let width = (barcode.len() as u32) * xdim;
        let mut buffer = ImageBuffer::new(width, height);
        let mut pos = 0;

        for y in 0..height {
            for &b in barcode {
                let size = xdim;

                if b == 0 {
                    for p in 0..size {
                        buffer.put_pixel(pos + p, y, image::Luma([255]));
                    }
                }

                pos += size;
            }

            pos = 0;
        }

        let mut bytes: Vec<u8> = vec![];

        {
            image::ImageLuma8(buffer).save(&mut bytes, format).unwrap();
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    extern crate image;

    use ::sym::ean13::*;
    use ::sym::ean8::*;
    use ::sym::code39::*;
    use ::sym::ean_supp::*;
    use ::sym::tf::*;
    use ::generators::image::*;
    use std::io;
    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::fs::File;
    use std::path::Path;

    const TEST_DATA_BASE: &'static str = "./target/debug";
    const WRITE_TO_FILE: bool = true;

    fn open_file(name: &'static str) -> File {
        File::create(&Path::new(&format!("{}/{}", TEST_DATA_BASE, name)[..])).unwrap()
    }

    fn write_file(bytes: &[u8], file: &'static str) {
        let path = open_file(file);
        let mut writer = BufWriter::new(path);
        writer.write(bytes).unwrap();
    }

    #[test]
    fn ean_13_as_gif() {
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let gif = Image::gif();
        let generated = gif.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE { write_file(&generated[..], "ean13.gif"); }

        assert_eq!(generated.len(), 7600);
    }

    #[test]
    fn ean_13_as_png() {
        let ean13 = EAN13::new("750103131130".to_owned()).unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 1,
        };
        let generated = png.generate(&ean13.encode()[..]).unwrap();

        assert_eq!(generated.len(), 9500);
    }

    #[test]
    fn ean_13_as_jpeg() {
        let ean13 = EAN13::new("999988881234".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 100,
            xdim: 3,
        };
        let generated = jpeg.generate(&ean13.encode()[..]).unwrap();

        assert_eq!(generated.len(), 28500);
    }

    #[test]
    fn code39_as_png() {
        let code39 = Code39::new("ILOVEMEL".to_owned()).unwrap();
        let png = Image::PNG {
            height: 60,
            xdim: 1,
        };
        let generated = png.generate(&code39.encode()[..]).unwrap();

        assert_eq!(generated.len(), 7740);
    }

    #[test]
    fn code39_as_gif() {
        let code39 = Code39::new("WIKIPEDIA".to_owned()).unwrap();
        let gif = Image::GIF {
            height: 60,
            xdim: 1,
        };
        let generated = gif.generate(&code39.encode()[..]).unwrap();

        assert_eq!(generated.len(), 8520);
    }

    #[test]
    fn code39_as_jpeg() {
        let code39 = Code39::new("SWAGLORDTHE3RD".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 160,
            xdim: 1,
        };
        let generated = jpeg.generate(&code39.encode()[..]).unwrap();

        assert_eq!(generated.len(), 33120);
    }

    #[test]
    fn ean8_as_png() {
        let ean8 = EAN8::new("5512345".to_owned()).unwrap();
        let png = Image::PNG {
            height: 70,
            xdim: 2,
        };
        let generated = png.generate(&ean8.encode()[..]).unwrap();

        assert_eq!(generated.len(), 9380);
    }

    #[test]
    fn ean8_as_gif() {
        let ean8 = EAN8::new("9992227".to_owned()).unwrap();
        let gif = Image::GIF {
            height: 70,
            xdim: 2,
        };
        let generated = gif.generate(&ean8.encode()[..]).unwrap();

        assert_eq!(generated.len(), 9380);
    }

    #[test]
    fn ean8_as_jpeg() {
        let ean8 = EAN8::new("9992227".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 70,
            xdim: 2,
        };
        let generated = jpeg.generate(&ean8.encode()[..]).unwrap();

        assert_eq!(generated.len(), 9380);
    }

    #[test]
    fn ean2_as_png() {
        let ean2 = EANSUPP::new("94".to_owned()).unwrap();
        let png = Image::PNG {
            height: 70,
            xdim: 2,
        };
        let generated = png.generate(&ean2.encode()[..]).unwrap();

        assert_eq!(generated.len(), 2800);
    }

    #[test]
    fn ean5_as_gif() {
        let ean5 = EANSUPP::new("51234".to_owned()).unwrap();
        let gif = Image::GIF {
            height: 70,
            xdim: 2,
        };
        let generated = gif.generate(&ean5.encode()[..]).unwrap();

        assert_eq!(generated.len(), 6580);
    }

    #[test]
    fn ean5_as_jpeg() {
        let ean5 = EANSUPP::new("51574".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 140,
            xdim: 5,
        };
        let generated = jpeg.generate(&ean5.encode()[..]).unwrap();

        assert_eq!(generated.len(), 32900);
    }

    #[test]
    fn itf_as_png() {
        let itf = TF::interleaved("1234567".to_owned()).unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 2,
        };
        let generated = png.generate(&itf.encode()[..]).unwrap();

        assert_eq!(generated.len(), 16000);
    }

    #[test]
    fn stf_as_png() {
        let stf = TF::standard("1234567".to_owned()).unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 2,
        };
        let generated = png.generate(&stf.encode()[..]).unwrap();

        assert_eq!(generated.len(), 22800);
    }

    #[test]
    fn itf_as_gif() {
        let itf = TF::interleaved("98766543561".to_owned()).unwrap();
        let gif = Image::GIF {
            height: 130,
            xdim: 1,
        };
        let generated = gif.generate(&itf.encode()[..]).unwrap();

        assert_eq!(generated.len(), 15080);
    }

    #[test]
    fn itf_as_jpeg() {
        let itf = TF::interleaved("98766543561".to_owned()).unwrap();
        let jpeg = Image::JPEG {
            height: 130,
            xdim: 1,
        };
        let generated = jpeg.generate(&itf.encode()[..]).unwrap();

        assert_eq!(generated.len(), 15080);
    }
}
