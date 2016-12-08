# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

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
