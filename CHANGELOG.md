# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### v1.0.2 (2020-09-09)

- [fixed] Typo in Code128 binary mappings for char: FS, |, 92


### v1.0.1 (2019-12-03)

- [fixed] Several linting issues.

### v1.0.0 (2019-12-03)

- [changed] Updated image dependency from 0.18.0 to 0.22.0.

### v0.10.0 (2018-02-28)

- [added] Support for FNC1, FNC2, FNC3, FNC4 and SHIFT meta-characters in Code128 symbology.
- [changed] Updated image dependency from 0.16.0 to 0.18.0.

### v0.9.0 (2017-10-30)

- [added] Code11 barcode encoder.
- [removed] Public checksum functions from public API's of all encoders (breaking change!).
- [changed] Updated image dependency from 0.15.0 to 0.16.0.
- [changed] Updated clippy dependency from 0.0.134 to 0.0.166.
- [changed] Several internal refactorings.

### v0.8.2 (2017-09-03)

- [added] Code93 barcode encoder.
- [changed] Removed gratuitous qualifiers from some constants.
- [changed] Updated image dependency from 0.13.0 to 0.15.0

### v0.8.1 (2017-06-20)

- [changed] SVG constructor now requires `height` argument.

### v0.8.0 (2017-06-12)

- [changed] Image enum variant constructors now require `height` argument.
- [changed] Image generator now accepts `background` and `foreground` fields to specify RGBA attributes.
- [added] Added 'generate_buffer' method to 'generators::image::\*', which returns 'image::ImageBuffer<Rgba<u8>, Vec<u8>>'.
- [changed] Updated clippy dependency from 0.0.83 to 0.0.134
- [changed] Updated image dependency from 0.10.3 to 0.13.0
- [removed] Removed static lifetime indicators in consts (as implemented in Rust 1.17).
- [changed] Saving 88 bytes in SVG generation.
- [changed] Refactor pattern matching statements to use 'and_then' combinator.

### v0.7.0 (2017-02-11)

- [added] Added JSON generator. Very simple, but useful for sending encoded data to third parties.
- [fixed] Removed obsolete imports in ASCII generator.
- [changed] Usage documentation in README.

### v0.6.0 (2016-12-09)

- [changed] Swapped try!() macros for ? operator that was stableized in Rust 1.13
- [changed] Usage documentation in README.

### v0.5.1 (2016-08-18)

- [changed] Avoid use of owned String in parsing.
- [changed] Updated dependencies.
- [fixed] Usage documentation in README.

### v0.5.0 (2016-02-04)

- [added] Codabar symbology encoder.
- [removed] 'raw_data' method from all encoders.

### v0.4.0 (2016-01-30)

- [added] Code128 symbology encoder.

### v0.3.6 (2016-01-04)

- [changed] Relicensed to dual MIT/APACHE.
- [changed] Updated dependencies to latest stable sub-versions.

### v0.3.5 (2015-12-03)

- [added] Rotation support for PNG, GIF, JPEG image generators.
- [added] Error type for generators.

### v0.3.4 (2015-11-30)

- [added] Error types for all encoders.

### v0.3.3 (2015-11-28)

- [added] SVG generator.
