//! This module provides types for generating GIF representations of barcodes. 

extern crate image;

use ::sym::EncodedBarcode;

use image::GenericImage;
use image::ImageBuffer;

use std::fs::File;
use std::path::Path;

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
    /// Returns a new GIF with default height and xdim values.
    //pub fn new() -> Image {
   //     match Image {
   //         Gif => Gif{height: 80, xdim: 1},
   //         Png => Gif{height: 80, xdim: 1},
  //      }
 //   }

    /// Generates the given EncodedBarcode. Returns a usize indicating the number of bytes written.
    pub fn generate(&self, barcode: &EncodedBarcode, path: &str) -> Result<usize, &str> {
        let (xdim, height, format) = match *self {
            Image::GIF{height: h, xdim: x} => (x, h, image::GIF),
            Image::PNG{height: h, xdim: x} => (x, h, image::PNG),
        };

        let width = (barcode.len() as u32) * (xdim * 3);
        let mut buffer = ImageBuffer::new(width, height);
        
        for x in 0..(width - 1) {
            buffer.put_pixel(x, 0, image::Luma([255 as u8]));
            buffer.put_pixel(x, 1, image::Luma([255 as u8]));
            buffer.put_pixel(x, 2, image::Luma([255 as u8]));
            buffer.put_pixel(x, 3, image::Luma([255 as u8]));
        }

        let ref mut fout = match File::create(&Path::new(path)) {
            Ok(f) => f,
            _ => return Err("Could not open file for writing."),
        };
        let buflen = buffer.len();

        match image::ImageLuma8(buffer).save(fout, format) {
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

    #[test]
    fn ean_13_as_gif() {
        let ean13 = EAN13::new("750103131130".to_string()).unwrap();
        let gif = Image::GIF{height: 80, xdim: 1};
        let generated = gif.generate(&ean13.encode(), "./barcode.gif").unwrap();

        assert_eq!(generated, 22800);
    }

    #[test]
    fn ean_13_as_png() {
        let ean13 = EAN13::new("750103131130".to_string()).unwrap();
        let png = Image::PNG{height: 80, xdim: 1};
        let generated = png.generate(&ean13.encode(), "./barcode.png").unwrap();

        assert_eq!(generated, 22800);
    }
}
