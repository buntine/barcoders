# Barcoders

Barcoders is a barcode-encoding library for the Rust programming language.

The goal of this project is to support all major symbologies (EAN-13/8, Bookland, UPC-A, Code128, Code39, Code25, etc) and provide several output formats (ASCII, SVG, GIF, PNG, HTML, etc).

**Barcoders is under active development. Initial release expected late November, 2015**

## Usage

### ASCII generation
```rust
extern crate barcoders;

use barcoders::sym::ean13::*;
use barcoders::generators::ascii::*;

let barcode = EAN13::new("750103131130".to_string()).unwrap();
let encoded: Vec<u8> = ean13.encode();
let ascii = ASCII::new().generate(&encoded);

assert_eq!(ascii.unwrap(),
"
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
".trim().to_string());
```

### Image generation
```rust
extern crate barcoders;

use barcoders::sym::code39::*;
use barcoders::generators::image::*;

let barcode = EAN13::new("750103131130".to_string()).unwrap();
let encoded: Vec<u8> = ean13.encode();
let png = Image::PNG{height: 80, xdim: 1};

// Image generators save the file to the given path and return a u32 indicating the 
// number of bytes written to disk.
let mut path = File::create(&Path::new("my_barcode.png)).unwrap();
let bytes = png.generate(&encoded, &mut path).unwrap();
```
