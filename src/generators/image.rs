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
//! let png = Image::PNG{height: 80,
//!                      xdim: 1,
//!                      rotation: Rotation::Zero,
//!                      foreground: Color::new([0, 0, 0, 255]),
//!                      background: Color::new([255, 255, 255, 255])};
//!
//! // Or use the constructor for defaults (you must specify the height).
//! let png = Image::png(100);
//! ```
//!
//! See the README for more examples.

use std::io::Cursor;
use crate::error::{Error, Result};
use image::{
    DynamicImage::{self, ImageRgba8},
    ImageBuffer, ImageFormat, Rgba,
};

macro_rules! image_variants {
    ( $( #[$attr:meta] $v:ident ),* ) => {
        /// The image generator type.
        #[derive(Copy, Clone, Debug)]
        pub enum Image {
        $(
            #[$attr]
            $v {
                /// The height of the barcode in pixels.
                height: u32,
                /// The X dimension. Specifies the width of the "narrow" bars, each
                /// of which will be ```self.xdim``` pixels wide.
                xdim: u32,
                /// The rotation to apply to the generated barcode.
                rotation: Rotation,
                /// The RGBA color for the foreground.
                foreground: Color,
                /// The RGBA color for the background.
                background: Color,
            },
        )*
        }
    };
}

macro_rules! image_defaults {
    ($v:ident, $h:expr) => {
        Image::$v {
            height: $h,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        }
    };
}

macro_rules! expand_image_variants {
    ($s:expr, $b:tt => $e:tt, $($v:ident),+) => (
        match $s {
            $(
                Image::$v$b => $e
            ),+
        }
    );
}

/// Represents a RGBA color for the barcode foreground and background.
#[derive(Copy, Clone, Debug)]
pub struct Color {
    /// Reg, Green, Blue, Alpha value.
    rgba: [u8; 4],
}

impl Color {
    /// Constructor.
    pub fn new(rgba: [u8; 4]) -> Color {
        Color { rgba }
    }

    /// Constructor for black (#000000).
    pub fn black() -> Color {
        Color::new([0, 0, 0, 255])
    }

    /// Constructor for white (#FFFFFF).
    pub fn white() -> Color {
        Color::new([255, 255, 255, 255])
    }

    fn to_rgba(self) -> Rgba<u8> {
        Rgba(self.rgba)
    }
}

/// Possible rotation values for images.
#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    /// No rotation. This is the default.
    Zero,
    /// Rotated 90 degrees.
    Ninety,
    /// Rotated 180 degrees.
    OneEighty,
    /// Rotated 270 degrees.
    TwoSeventy,
}

image_variants![
    /// GIF image generator type.
    GIF,
    /// PNG image generator type.
    PNG,
    /// WEBP image generator type.
    WEBP,
    /// Image Buffer generator type.
    ImageBuffer
];

impl Image {
    /// Returns a new GIF with default values.
    pub fn gif(height: u32) -> Image {
        image_defaults!(GIF, height)
    }

    /// Returns a new PNG with default values.
    pub fn png(height: u32) -> Image {
        image_defaults!(PNG, height)
    }

    /// Returns a new WEBP with default values.
    pub fn webp(height: u32) -> Image {
        image_defaults!(WEBP, height)
    }

    /// Returns a new ImageBuffer with default values.
    pub fn image_buffer(height: u32) -> Image {
        image_defaults!(ImageBuffer, height)
    }

    /// Generates the given barcode. Returns a `Result<Vec<u8>, Error>` of the encoded bytes or
    /// an error message.
    pub fn generate<T: AsRef<[u8]>>(&self, barcode: T) -> Result<Vec<u8>> {
        let format = match *self {
            Image::GIF { .. } => ImageFormat::Gif,
            Image::PNG { .. } => ImageFormat::Png,
            Image::WEBP { .. } => ImageFormat::WebP,
            _ => return Err(Error::Generate),
        };

        let mut bytes: Vec<u8> = vec![];
        let img = self.place_pixels(&barcode);

        match img.write_to(&mut Cursor::new(&mut bytes), format) {
            Ok(_) => Ok(bytes),
            _ => Err(Error::Generate),
        }
    }

    /// Generates the given barcode to an image::ImageBuffer. Returns a `Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Error>`
    /// of the encoded bytes or an error message.
    pub fn generate_buffer<T: AsRef<[u8]>>(
        self,
        barcode: T,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let img = self.place_pixels(&barcode);

        Ok(img.to_rgba8())
    }

    fn place_pixels<T: AsRef<[u8]>>(&self, barcode: T) -> DynamicImage {
        let barcode = barcode.as_ref();
        let (xdim, height, rotation, bg, fg) = expand_image_variants!(
            *self,
            {height: h, xdim: x, rotation: r, background: b, foreground: f} => (x, h, r, b.to_rgba(), f.to_rgba()),
            GIF, PNG, WEBP, ImageBuffer
        );
        let width = (barcode.len() as u32) * xdim;
        let mut buffer = ImageBuffer::new(width, height);

        for y in 0..height {
            for (i, &b) in barcode.iter().enumerate() {
                let c = if b == 0 { bg } else { fg };

                for p in 0..xdim {
                    buffer.put_pixel((i as u32 * xdim) + p, y, c);
                }
            }
        }

        let img = ImageRgba8(buffer);

        match rotation {
            Rotation::Ninety => img.rotate90(),
            Rotation::OneEighty => img.rotate180(),
            Rotation::TwoSeventy => img.rotate270(),
            _ => img,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate image;

    use crate::generators::image::*;
    use crate::sym::codabar::*;
    use crate::sym::code11::*;
    use crate::sym::code128::*;
    use crate::sym::code39::*;
    use crate::sym::code93::*;
    use crate::sym::ean13::*;
    use crate::sym::ean8::*;
    use crate::sym::ean_supp::*;
    use crate::sym::tf::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::path::Path;

    const TEST_DATA_BASE: &str = "./target/debug";
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
        let ean13 = EAN13::new(b"750103131130").unwrap();
        let gif = Image::gif(80);
        let generated = gif.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean13.gif");
        }

        assert_eq!(generated.len(), 918);
    }

    #[test]
    fn ean_13_as_png() {
        let ean13 = EAN13::new(b"750103131130").unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean13.png");
        }

        assert_eq!(generated.len(), 872);
    }

    #[test]
    fn rotated_ean_13_as_png() {
        let ean13 = EAN13::new(b"750103131130").unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 1,
            rotation: Rotation::Ninety,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean13_90.png");
        }

        assert_eq!(generated.len(), 716);
    }

    #[test]
    fn ean_13_as_webp() {
        let ean13 = EAN13::new(b"999988881234").unwrap();
        let webp = Image::WEBP {
            height: 100,
            xdim: 3,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = webp.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE {
           write_file(&generated[..], "ean13.webp");
       }

       assert_eq!(generated.len(), 150);
    }

    #[test]
    fn ean_13_as_image_buffer() {
        let ean13 = EAN13::new(b"750399599113").unwrap();
        let img = Image::ImageBuffer {
            height: 99,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = img.generate_buffer(&ean13.encode()[..]).unwrap();

        assert_eq!(generated.height(), 99);
        assert_eq!(generated.width(), 95);
    }

    #[test]
    fn colored_ean_13_as_gif() {
        let ean13 = EAN13::new(b"750103131130").unwrap();
        let gif = Image::GIF {
            height: 99,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [255, 38, 42, 255],
            },
            background: Color {
                rgba: [34, 52, 255, 255],
            },
        };

        let generated = gif.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "colored_ean13.gif");
        }

        assert_eq!(generated.len(), 1084);
    }

    #[test]
    fn colored_semi_opaque_ean_13_as_png() {
        let ean13 = EAN13::new(b"750153666132").unwrap();
        let png = Image::PNG {
            height: 99,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [255, 38, 42, 120],
            },
            background: Color {
                rgba: [34, 52, 255, 120],
            },
        };

        let generated = png.generate(&ean13.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "colored_opaque_ean13.png");
        }

        assert_eq!(generated.len(), 1027);
    }

    #[test]
    fn code39_as_png() {
        let code39 = Code39::new(b"ILOVEMEL").unwrap();
        let png = Image::PNG {
            height: 60,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&code39.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code39.png");
        }

        assert_eq!(generated.len(), 716);
    }

    #[test]
    fn code39_as_gif() {
        let code39 = Code39::new(b"WIKIPEDIA").unwrap();
        let gif = Image::GIF {
            height: 60,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&code39.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code39.gif");
        }

        assert_eq!(generated.len(), 911);
    }

    #[test]
    fn rotated_code39_as_gif() {
        let code39 = Code39::new(b"HELLOWORLD").unwrap();
        let gif = Image::GIF {
            height: 60,
            xdim: 1,
            rotation: Rotation::OneEighty,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&code39.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code39_180.gif");
        }

        assert_eq!(generated.len(), 969);
    }

    #[test]
    fn code93_as_png() {
        let code93 = Code93::new(b"ILOVEBAH").unwrap();
        let png = Image::PNG {
            height: 60,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&code93.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code93.png");
        }

        assert_eq!(generated.len(), 681);
    }

    #[test]
    fn code93_as_gif() {
        let code93 = Code93::new(b"CIVIC VIDEO").unwrap();
        let gif = Image::GIF {
            height: 60,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&code93.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code93.gif");
        }

        assert_eq!(generated.len(), 982);
    }

    #[test]
    fn rotated_code93_as_gif() {
        let code93 = Code93::new(b"TWISTIES 100").unwrap();
        let gif = Image::GIF {
            height: 60,
            xdim: 1,
            rotation: Rotation::OneEighty,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&code93.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code93_180.gif");
        }

        assert_eq!(generated.len(), 1061);
    }

    #[test]
    fn code11_as_png() {
        let code11 = Code11::new(b"9923-1111").unwrap();
        let png = Image::PNG {
            height: 60,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&code11.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code11.png");
        }

        assert_eq!(generated.len(), 602);
    }

    #[test]
    fn code11_as_gif() {
        let code11 = Code11::new(b"122333444455556666").unwrap();
        let gif = Image::GIF {
            height: 60,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&code11.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code11.gif");
        }

        assert_eq!(generated.len(), 981);
    }

    #[test]
    fn codabar_as_png() {
        let codabar = Codabar::new(b"B12354999A").unwrap();
        let png = Image::PNG {
            height: 60,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&codabar.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "codabar.png");
        }

        assert_eq!(generated.len(), 681);
    }

    #[test]
    fn codabar_as_gif() {
        let codabar = Codabar::new(b"A5675+++3$$B").unwrap();
        let gif = Image::GIF {
            height: 80,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&codabar.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "codabar.gif");
        }

        assert_eq!(generated.len(), 1653);
    }

    #[test]
    fn rotated_codabar_as_gif() {
        let codabar = Codabar::new(b"C1234D").unwrap();
        let gif = Image::GIF {
            height: 60,
            xdim: 1,
            rotation: Rotation::Ninety,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&codabar.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "codabar_180.gif");
        }

        assert_eq!(generated.len(), 174);
    }

    #[test]
    fn code128_as_png() {
        let code128 = Code128::new("ÀHIĆ345678".as_bytes()).unwrap();
        let png = Image::PNG {
            height: 60,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&code128.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code128.png");
        }

        assert_eq!(generated.len(), 659);
    }

    #[test]
    fn code128_as_gif() {
        let code128 = Code128::new("ÀHELLOWORLD".as_bytes()).unwrap();
        let gif = Image::GIF {
            height: 90,
            xdim: 3,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&code128.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code128.gif");
        }

        assert_eq!(generated.len(), 2741);
    }

    #[test]
    fn rotated_code128_as_gif() {
        let code128 = Code128::new("ÀHELLOWORLD".as_bytes()).unwrap();
        let gif = Image::GIF {
            height: 90,
            xdim: 3,
            rotation: Rotation::OneEighty,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&code128.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "code128_180.gif");
        }

        assert_eq!(generated.len(), 2752);
    }

    #[test]
    fn rotated_code128_as_image_buffer() {
        let code128 = Code128::new("ƁCLOJURE".as_bytes()).unwrap();
        let img = Image::ImageBuffer {
            height: 93,
            xdim: 2,
            rotation: Rotation::OneEighty,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = img.generate_buffer(&code128.encode()[..]).unwrap();

        assert_eq!(generated.height(), 93);
        assert_eq!(generated.width(), 224);
    }

    #[test]
    fn ean8_as_png() {
        let ean8 = EAN8::new(b"5512345").unwrap();
        let png = Image::PNG {
            height: 70,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&ean8.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean8.png");
        }

        assert_eq!(generated.len(), 692);
    }

    #[test]
    fn rotated_ean8_as_png() {
        let ean8 = EAN8::new(b"5512345").unwrap();
        let png = Image::PNG {
            height: 70,
            xdim: 2,
            rotation: Rotation::TwoSeventy,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&ean8.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean8_270.png");
        }

        assert_eq!(generated.len(), 808);
    }

    #[test]
    fn ean8_as_gif() {
        let ean8 = EAN8::new(b"9992227").unwrap();
        let gif = Image::GIF {
            height: 70,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&ean8.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean8.gif");
        }

        assert_eq!(generated.len(), 897);
    }

    #[test]
    fn ean8_as_webp() {
        let ean8 = EAN8::new(b"9992227").unwrap();
        let webp = Image::WEBP {
            height: 70,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = webp.generate(&ean8.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean8.webp");
        }

        assert_eq!(generated.len(), 114);
    }

    #[test]
    fn ean2_as_png() {
        let ean2 = EAN2::new(b"94").unwrap();
        let png = Image::PNG {
            height: 70,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&ean2.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean2.png");
        }

        assert_eq!(generated.len(), 485);
    }

    #[test]
    fn ean5_as_gif() {
        let ean5 = EAN5::new(b"51234").unwrap();
        let gif = Image::GIF {
            height: 70,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&ean5.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean5.gif");
        }

        assert_eq!(generated.len(), 655);
    }

    #[test]
    fn ean5_as_webp() {
        let ean5 = EAN5::new(b"51574").unwrap();
        let webp = Image::WEBP {
            height: 140,
            xdim: 5,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = webp.generate(&ean5.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ean5.webp");
        }

        assert_eq!(generated.len(), 124);
    }

    #[test]
    fn ean5_as_imagebuffer() {
        let ean5 = EAN5::new(b"99888").unwrap();
        let img = Image::ImageBuffer {
            height: 140,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = img.generate_buffer(&ean5.encode()[..]).unwrap();

        assert_eq!(generated.height(), 140);
        assert_eq!(generated.width(), 47);
    }

    #[test]
    fn itf_as_png() {
        let itf = ToF::interleaved(b"1234567").unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&itf.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ift.png");
        }

        assert_eq!(generated.len(), 911);
    }

    #[test]
    fn stf_as_png() {
        let stf = ToF::new(b"1234567").unwrap();
        let png = Image::PNG {
            height: 100,
            xdim: 2,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = png.generate(&stf.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "sft.png");
        }

        assert_eq!(generated.len(), 1131);
    }

    #[test]
    fn itf_as_gif() {
        let itf = ToF::interleaved(b"98766543561").unwrap();
        let gif = Image::GIF {
            height: 130,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = gif.generate(&itf.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ift.gif");
        }

        assert_eq!(generated.len(), 1410);
    }

    #[test]
    fn itf_as_webp() {
        let itf = ToF::interleaved(b"98766543561").unwrap();
        let webp = Image::WEBP {
            height: 130,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = webp.generate(&itf.encode()[..]).unwrap();

        if WRITE_TO_FILE {
            write_file(&generated[..], "ift.webp");
        }

        assert_eq!(generated.len(), 116);
    }

    #[test]
    fn itf_as_imagebuffer() {
        let itf = ToF::interleaved(b"98766543561").unwrap();
        let img = Image::ImageBuffer {
            height: 130,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };
        let generated = img.generate_buffer(&itf.encode()[..]).unwrap();

        assert_eq!(generated.height(), 130);
        assert_eq!(generated.width(), 116);
    }

    #[test]
    fn image_buffer_fails_on_generate() {
        let itf = ToF::interleaved(b"98766543561").unwrap();
        let img = Image::ImageBuffer {
            height: 130,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: Color {
                rgba: [0, 0, 0, 255],
            },
            background: Color {
                rgba: [255, 255, 255, 255],
            },
        };

        assert!(img.generate(&itf.encode()[..]).is_err());
    }
}
