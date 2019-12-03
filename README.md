[![Build Status](https://travis-ci.org/buntine/barcoders.svg?branch=master)](https://travis-ci.org/buntine/barcoders)
[![Coverage Status](https://coveralls.io/repos/buntine/barcoders/badge.svg?branch=master&service=github)](https://coveralls.io/github/buntine/barcoders?branch=master)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](http://meritbadge.herokuapp.com/barcoders)](https://crates.io/crates/barcoders)
[![Algorithmia](https://algorithmia.com/algorithms/anowell/barcoders/badge)](https://algorithmia.com/algorithms/anowell/barcoders)

![BARCODERS](/media/logo.jpg?raw=true "BARCODERS")

**Barcoders** is a barcode-encoding library for the Rust programming language.

Barcoders allows you to encode valid data for a chosen barcode symbology into a ```Vec<u8>``` representation of the underlying binary structure. From here, you can take advantage of one of the optional builtin generators (for exporting to SVG, GIF, PNG, etc) or build your own.

## Installation

For encode-only functionality (e.g if you just want to translate a `String` into a `Vec<u8>` of binary digits):

```toml
[dependencies]
barcoders = "1.0.1"
```

If you want to generate barcodes into a particular format, turn on the appropriate feature(s):

```toml
[dependencies]
barcoders = {version = "1.0.1", features = ["image", "ascii", "svg", "json"]}
```

Each generator is an optional feature so you only need to compile what you want to use.
See below for the feature associated to the generation functionality you desire.

## Documentation

[Documentation and examples are available here](https://docs.rs/barcoders).

## Current Support

The ultimate goal of Barcoders is to provide encoding support for all major (and many not-so-major) symbologies.

### Symbologies

* EAN-13
  * UPC-A
  * JAN
  * Bookland
* EAN-8
* EAN Supplementals
  * EAN-2
  * EAN-5
* Code11
  * USD-8
* Code39
* Code93
* Code128 (A, B, C)
* Two-Of-Five
  * Interleaved (ITF)
  * Standard (STF)
* Codabar
* More coming!

### Generators

* ASCII (feature: `ascii`)
* JSON (feature: `json`)
* SVG (feature: `svg`)
* PNG (feature: `image`)
* GIF (feature: `image`)
* JPEG (feature: `image`)
* Image Buffer (feature: `image`)
* Or add your own

## Examples

### Encoding
```rust
extern crate barcoders;

use barcoders::sym::ean13::*;

// Each encoder accepts a String to be encoded. Valid data is barcode-specific
// and thus constructors return an Result<T, barcoders::error::Error>.
let barcode = EAN13::new("593456661897").unwrap();

// The `encode` method returns a Vec<u8> of the binary representation of the
// generated barcode. This is useful if you want to add your own generator.
let encoded: Vec<u8> = barcode.encode();
```

### Image (GIF, JPEG, PNG) generation
```rust
extern crate barcoders;

use barcoders::sym::code39::*;
use barcoders::generators::image::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

let barcode = Code39::new("1ISTHELONELIESTNUMBER").unwrap();
let png = Image::png(80); // You must specify the height in pixels.
let encoded = barcode.encode();

// Image generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes.
let bytes = png.generate(&encoded[..]).unwrap();

// Which you can then save to disk.
let file = File::create(&Path::new("my_barcode.png")).unwrap();
let mut writer = BufWriter::new(file);
writer.write(&bytes[..]).unwrap();

// Generated file ↓ ↓ ↓
```
![Code 39: 1ISTHELONELIESTNUMBER](/media/code39_1istheloneliestnumber.png?raw=true "Code 39: 1ISTHELONELIESTNUMBER")

You can also request an [image::RgbaImage](http://www.piston.rs/image/image/type.RgbaImage.html), which you can manipulate yourself:
```rust
let barcode = Code39::new("BEELZEBUB").unwrap();
let buffer = Image::image_buffer(100);
let encoded = barcode.encode();
let img = buffer.generate_buffer(&encoded[..]).unwrap();

// Manipulate and save the image here...
```

You may also specify the barcode x-dimension, rotation, background/foreground colors and opacity by specifying the struct fields:
```rust
let gif = Image::GIF{height: 80,
                     xdim: 1,
                     rotation: Rotation::Zero,
                     // Using non black/white colors is generally not recommended by most vendors, but barcoders makes it possible.
                     foreground: Color::new([255, 0, 0, 255]),
                     background: Color::new([0, 255, 20, 255])};
```

### SVG generation

SVG is similar to the other image types, but I've supplied it as a separate feature as it doesn't require third-party dependencies.

```rust
extern crate barcoders;

use barcoders::sym::code39::*;
use barcoders::generators::svg::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

let barcode = Code39::new("56DFU4A777H").unwrap();
let svg = SVG::new(200); // You must specify the height in pixels.
let encoded = barcode.encode();
let data: String = svg.generate(&encoded).unwrap();

let file = File::create(&Path::new("my_barcode.svg")).unwrap();
let mut writer = BufWriter::new(file);
writer.write(data.as_bytes()).unwrap();
```

You may also specify the barcode x-dimension, background/foreground colors and opacity by specifying the struct fields:
```rust
let gif = SVG{height: 80,
              xdim: 1,
              // Using non black/white colors is generally not recommended by most vendors, but barcoders makes it possible.
              foreground: Color::black(),
              background: Color::new([0, 255, 20, 255])};
```

### ASCII generation

The ASCII generator is useful for testing purposes.

```rust
extern crate barcoders;

use barcoders::sym::ean13::*;
use barcoders::generators::ascii::*;

let barcode = EAN13::new("750103131130").unwrap();
let encoded = barcode.encode();

let ascii = ASCII::new();
ascii.generate(&encoded[..]);

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
".trim());
```


### JSON generation

This may be useful for passing encoded data to third-party systems in a conventional format.

```rust
extern crate barcoders;

use barcoders::sym::codabar::*;
use barcoders::generators::json::*;

let codabar = Codabar::new("A98B").unwrap();
let json = JSON::new();
let generated = json.generate(&codabar.encode()[..]);

assert_eq!(generated.unwrap(),
"
{
 \"height\": 10,
 \"xdim\": 1,
 \"encoding\": [1,0,1,1,0,0,1,0,0,1,0,1,1,0,1,0,0,1,0,1,0,1,0,0,1,1,0,1,0,1,0,1,0,1,0,0,1,0,0,1,1]
}
"
```

## Tests

Note, if you want to output actual image/svg files to the filesystem for visual confirmation, set
the `WRITE_TO_FILE` variable in the appropriate test modules.

Full suite:
```
$ cargo test --features="image svg ascii json"
```

Encoding only:
```
$ cargo test
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
