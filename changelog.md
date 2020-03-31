# Changelog

All notable changes to this project will be documented in this file.

This format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [Unreleased]

## [0.1.0] (2018-10-15)

The initial release of current iteration

## [0.0.6] (2018-10-14)

* remove feature flag `external-doc`
  - use the field `use_external_doc` in `doubter!()` instead
* add initial support for build script
* add support for files outside of `CARGO_MANIFEST_DIR`
* add mode to extract code blocks in Markdown

## [0.0.5] (2018-10-13)

* add glob pattern support ([#4](https://github.com/ubnt-intrepid/doubter/pull/4))
* save the original directory tree as module structure ([#6](https://github.com/ubnt-intrepid/doubter/pull/6))

### Compatibility Notes
* rename the field key `file` to `include`

## [0.0.4] (2018-10-12)

* fix sanitization scheme of file path

## [0.0.3] (2018-10-11)

* revert: extract code blocks from Markdown files
* add feature flag whether to use the unstable `#[doc(include = "...")]`

## [0.0.2] (2018-10-11)

* extract code blocks from Markdown files

## [0.0.1] (2018-10-11)

First release

<!-- links -->

[Unreleased]: https://github.com/ubnt-intrepid/doubter/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ubnt-intrepid/doubter/compare/v0.0.6...v0.1.0
[0.0.6]: https://github.com/ubnt-intrepid/doubter/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/ubnt-intrepid/doubter/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/ubnt-intrepid/doubter/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/ubnt-intrepid/doubter/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/ubnt-intrepid/doubter/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/ubnt-intrepid/doubter/tree/v0.0.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
