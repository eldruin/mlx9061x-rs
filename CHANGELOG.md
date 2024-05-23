# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- [breaking-change] Transitioned `embedded-hal` to version 1.0
- Add feature `defmt-03` to derive "`defmt::Format`" from `defmt = "0.3"` for public types.

## [0.2.1] - 2023-06-02

### Added

- Methods for reading temperature as a integer (`u16`).
  Users are advised to ONLY use these methods if running into to trouble using
  the standard temperature reading methods returning `f32` values as it's a more accurate read.

## [0.2.0] - 2021-05-22

### Added

- Add support for device sleep and wake.

### Changed

- Removed delays after final EEPROM writes before exiting a method.
  Users are advised to wait enough time before interacting with the device again.
  Thanks to @David-OConnor for the suggestion.

## [0.1.0] - 2020-07-29

Initial release to crates.io.

[Unreleased]: https://github.com/eldruin/mlx9061x-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/eldruin/mlx9061x-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/mlx9061x-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eldruin/mlx9061x-rs/releases/tag/v0.1.0
