//! This module provides types for generating GIF representations of barcodes. 

extern crate image;

use ::sym::EncodedBarcode;
use image::GenericImage;
use image::ImageBuffer;
use std::fs::File;

/// The GIF barcode generator type.
pub enum Image {
    GIF {
        /// The height of the barcode in pixels.
        height: u32,
        /// The X dimension. Specifies the width of the "narrow" bars. 
        /// For GIF, each will be ```self.xdim * 3``` pixels wide.
        xdim: u32,
    },
    PNG {
        /// The height of the barcode in pixels.
        height: u32,
        /// The X dimension. Specifies the width of the "narrow" bars. 
        /// For GIF, each will be ```self.xdim * 3``` pixels wide.
        xdim: u32,
    }
}

impl Image {
    /// Returns a new GIF with default values.
    pub fn gif() -> Image {
        Image::GIF{height: 80, xdim: 1}
    }

    /// Returns a new PNG with default values.
    pub fn png() -> Image {
        Image::PNG{height: 80, xdim: 1}
    }

    /// Generates the given EncodedBarcode. Returns a usize indicating the number of bytes written.
    pub fn generate(&self, barcode: &EncodedBarcode, path: &mut File) -> Result<usize, &str> {
        let (xdim, height, format) = match *self {
            Image::GIF{height: h, xdim: x} => (x, h, image::GIF),
            Image::PNG{height: h, xdim: x} => (x, h, image::PNG),
        };

        let width = (barcode.len() as u32) * (xdim * 3);
        let mut buffer = ImageBuffer::new(width, height);
        
        // TODO: Implement.
        for x in 0..(width - 1) {
            buffer.put_pixel(x, 0, image::Luma([255 as u8]));
            buffer.put_pixel(x, 1, image::Luma([255 as u8]));
            buffer.put_pixel(x, 2, image::Luma([255 as u8]));
            buffer.put_pixel(x, 3, image::Luma([255 as u8]));
        }

        let buflen = buffer.len();

        match image::ImageLuma8(buffer).save(path, format) {
            Ok(_) => Ok(buflen),
            _ => Err("Could not encode image."),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate image;

    use ::sym::ean13::*;
    use ::generators::gif::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn ean_13_as_gif() {
        let ean13 = EAN13::new("750103131130".to_string()).unwrap();
        let gif = Image::gif();
        let mut path = File::create(&Path::new("./barcode.gif")).unwrap();
        let generated = gif.generate(&ean13.encode(), &mut path).unwrap();

        assert_eq!(generated, 22800);
    }

    #[test]
    fn ean_13_as_png() {
        let ean13 = EAN13::new("750103131130".to_string()).unwrap();
        let png = Image::PNG{height: 100, xdim: 1};
        let mut path = File::create(&Path::new("./barcode.png")).unwrap();
        let generated = png.generate(&ean13.encode(), &mut path).unwrap();

        assert_eq!(generated, 28500);
    }
}
