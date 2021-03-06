# Changelog
All notable changes (and upcoming changes) to this crate will be documented in this file. This format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [Unreleased]

## [1.1.0] - 2019-01-04
### Added
- CHANGELOG.md
- Travis CI configuration.
- Badges for documentation and build status on Travis.
- `Signs` trait which provides methods for checking the sign bit of a primitive and for sign-extending primitive values.

## 1.0.0 - 2018-12-23
### Added
- `BitSize` trait which provides a constant for the number of bits on implementers on the integer primitives.
- `BitMask` trait which provides methods on the integer primitive types for generating masks and on instances to truncate the value with a mask.
- `BitIndex` trait which provides methods on the integer primitives for checking bits and getting the value in a bit range.

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/

[Unreleased]: https://github.com/ryanq/quark/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/ryanq/quark/compare/v1.0.0...v1.1.0
