[![Build Status](https://travis-ci.org/buntine/barcoders.svg?branch=master)](https://travis-ci.org/buntine/barcoders)
[![Coverage Status](https://coveralls.io/repos/buntine/barcoders/badge.svg?branch=master&service=github)](https://coveralls.io/github/buntine/barcoders?branch=master)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](http://meritbadge.herokuapp.com/barcoders)](https://crates.io/crates/barcoders)

![BARCODERS](/media/logo.jpg?raw=true "BARCODERS")

**Barcoders** is a barcode-encoding library for the Rust programming language.

Barcoders allows you to encode valid data for a chosen barcode symbology into a ```Vec<u8>``` representation of the underlying binary structure. From here, you can take advantage of one of the optional builtin generators (for exporting to SVG, GIF, PNG, etc) or build your own.

## Installation

For encode-only functionality (e.g if you just want to translate a `String` into a `Vec<u8>` of binary digits):

```toml
[dependencies]
barcoders = "0.3.5"
```

If you want to generate barcodes into a particular format, turn on the appropriate features:

```toml
[dependencies]
barcoders = {version = "0.3.5", features = ["image", "svg"]}
```

Each generator is an optional feature so you only need to compile what you want to use.
See below for the feature associated to the generation functionality you desire.

## Documentation

[Documentation and examples are available here](http://buntine.github.io/barcoders/barcoders/index.html).

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
* Code39
* Two-Of-Five
  * Interleaved (ITF)
  * Standard (STF)
* More coming!

### Generators

* ASCII (feature: `ascii`)
* SVG (feature: `svg`)
* PNG (feature: `image`)
* GIF (feature: `image`)
* JPEG (feature: `image`)
* More coming! (PostScript, etc)

## Examples

### Encoding
```rust
extern crate barcoders;

use barcoders::sym::ean13::*;

// Each encoder accepts a String to be encoded. Valid data is barcode-specific
// and thus constructors return an Result<T, barcoders::error::Error>.
let barcode = EAN13::new("593456661897".to_owned()).unwrap();

// The `encode` method returns a Vec<u8> of the binary representation of the
// generated barcode. This is useful if you want to add your own generator.
let encoded: Vec<u8> = barcode.encode();
```

### Image generation
```rust
extern crate barcoders;

use barcoders::sym::code39::*;
use barcoders::generators::image::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

let barcode = Code39::new("1ISTHELONELIESTNUMBER".to_owned()).unwrap();
let png = Image::PNG{height: 80, xdim: 1, rotation: Rotation::Zero};
let encoded = barcode.encode();

// Image generators return a Result<Vec<u8>, &str) of encoded bytes.
let bytes = png.generate(&encoded[..]).unwrap();

// Which you can then save to disk.
let file = File::create(&Path::new("my_barcode.png")).unwrap();
let mut writer = BufWriter::new(file);
writer.write(&bytes[..]).unwrap();

// Generated file ↓ ↓ ↓
```
![Code 39: 1ISTHELONELIESTNUMBER](/media/code39_1istheloneliestnumber.png?raw=true "Code 39: 1ISTHELONELIESTNUMBER")


### ASCII generation
```rust
extern crate barcoders;

use barcoders::sym::ean13::*;
use barcoders::generators::ascii::*;

let barcode = EAN13::new("750103131130".to_owned()).unwrap();
let encoded = barcode.encode();

// The ASCII generator is useful for testing purposes.
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
".trim().to_owned());
```

## Tests

Note, some of the image tests (intentionally) leave behind image files in ./target/debug that should be visually
inspected for correctness.

Full suite:
```
$ cargo test --features="image svg ascii"
```

Encoding only:
```
$ cargo test
```
