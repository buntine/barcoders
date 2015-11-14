//! This module provides types for generating GIF representations of barcodes. 

extern crate image;

use ::sym::EncodedBarcode;

use image::GenericImage;
use image::ImageBuffer;

use std::fs::File;
use std::path::Path;

/// The GIF barcode generator type.
pub struct GIF {
    /// The height of the barcode in pixels.
    pub height: u32,
    /// The X dimension. Specifies the width of the "narrow" bars. 
    /// For GIF, each will be ```self.xdim * 3``` pixels wide.
    pub xdim: u32,
}

impl GIF {
    /// Returns a new GIF with default height and xdim values.
    pub fn new() -> GIF {
        GIF{height: 80, xdim: 1}
    }

    /// Sets the height of the barcode and returns self.
    pub fn height(mut self, h: u32) -> GIF {
        self.height = h;
        self
    }

    /// Sets the X dimension of the barcode and returns self.
    pub fn xdim(mut self, x: u32) -> GIF {
        self.xdim = x;
        self
    }

    /// Generates the given EncodedBarcode. Returns an ImageBuffer.
    pub fn generate(&self, barcode: &EncodedBarcode) -> Result<u32, String> {
        let width = (barcode.len() as u32) * (self.xdim * 3);
        let mut buffer = ImageBuffer::new(width, self.height);
        
        for x in 0..(width - 1) {
            buffer.put_pixel(x, 0, image::Luma([255 as u8]));
            buffer.put_pixel(x, 1, image::Luma([255 as u8]));
            buffer.put_pixel(x, 2, image::Luma([255 as u8]));
            buffer.put_pixel(x, 3, image::Luma([255 as u8]));
        }

        let ref mut fout = File::create(&Path::new("gay.gif")).unwrap();
        let img = image::ImageLuma8(buffer).save(fout, image::GIF);
        Ok(10)
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
        let gif = GIF::new();
        let generated = gif.generate(&ean13.encode()).unwrap();
    }
}
