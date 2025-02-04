# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
+# Changelog
+All notable changes to this project will be documented in this file.
+
+The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
+and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
+
+## [Unreleased]

## [0.22.2](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.22.1...rasn-derive-impl-v0.22.2) - 2025-01-05

### Other

- Better error messages for derives ([#400](https://github.com/librasn/rasn/pull/400))

## [0.22.1](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.22.0...rasn-derive-impl-v0.22.1) - 2025-01-03

### Fixed

- *(derive)* use `const` blocks and add generic bounds when generating an enum's `AsnType` impl (#398)

### Other

- OER: remove nom usage, improve errors (#384)

## [0.22.0](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.21.0...rasn-derive-impl-v0.22.0) - 2024-11-26

### Other

- [**breaking**] Add lifetime for `encoder` trait and add allocation improvements based on that (OER) ([#370](https://github.com/librasn/rasn/pull/370))
- OER: improve decoding presence tracking ([#375](https://github.com/librasn/rasn/pull/375))
- Make constraints explicitly constant and evaluated in compile time & move some computation there (OER/PER) ([#318](https://github.com/librasn/rasn/pull/318))
