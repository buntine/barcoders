//! This module provides types for generating GIF representations of barcodes. 

use ::sym::EncodedBarcode;

/// The GIF barcode generator type.
pub struct GIF {
    /// The height of the barcode in pixels.
    pub height: usize,
    /// The X dimension. Specifies the width of the "narrow" bars. 
    /// For GIF, each will be ```self.xdim * 5``` pixels wide.
    pub xdim: usize,
}

impl GIF {
    /// Returns a new GIF with default height and xdim values.
    pub fn new() -> GIF {
        GIF{height: 80, xdim: 1}
    }

    /// Sets the height of the barcode and returns self.
    pub fn height(mut self, h: usize) -> GIF {
        self.height = h;
        self
    }

    /// Sets the X dimension of the barcode and returns self.
    pub fn xdim(mut self, x: usize) -> GIF {
        self.xdim = x;
        self
    }

    /// Generates the given EncodedBarcode. Returns an ImageBuffer.
    pub fn generate(&self, barcode: &EncodedBarcode) -> Result<String, String> {
        Ok("#".to_string())
    }
}

#[cfg(test)]
mod tests {
}
