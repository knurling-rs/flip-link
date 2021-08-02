# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- [#49] Fix Clippy warnings
- [#46] Link Knurling User Survey in `README`
- [#42] Add tests to check generation of `memory.x` in project root
- [#45] `Cargo.toml`: Disable default features of `env_logger`
- [#44] Add helper crate `xtest`
- [#41] Verify initial stack-pointer to be inside static RAM

[#49]: https://github.com/knurling-rs/flip-link/pull/49
[#46]: https://github.com/knurling-rs/flip-link/pull/46
[#42]: https://github.com/knurling-rs/flip-link/pull/42
[#45]: https://github.com/knurling-rs/flip-link/pull/45
[#44]: https://github.com/knurling-rs/flip-link/pull/44
[#41]: https://github.com/knurling-rs/flip-link/pull/41

## [v0.1.4] - 2021-05-21

- [#38] Handle no units in linker script parser

[#38]: https://github.com/knurling-rs/flip-link/pull/38

## [v0.1.3] - 2021-04-26

### Improvements
- [#33] Add clippy to CI
- [#32] Minimize deps
- [#24] Add bors (https://bors.tech/)
- [#22] Give `current_dir` precedence over `tempdir`
- [#18] Explain stack overflow problem and solution

### Fixes
- [#21] Fix typos in `README.md`

### Internal Improvements
- [#30] Refactoring II
- [#28] Refactoring I
- [#20] Add white background to SVG images

[#33]: https://github.com/knurling-rs/flip-link/pull/33
[#32]: https://github.com/knurling-rs/flip-link/pull/32
[#30]: https://github.com/knurling-rs/flip-link/pull/30
[#28]: https://github.com/knurling-rs/flip-link/pull/28
[#24]: https://github.com/knurling-rs/flip-link/pull/24
[#22]: https://github.com/knurling-rs/flip-link/pull/22
[#21]: https://github.com/knurling-rs/flip-link/pull/21
[#20]: https://github.com/knurling-rs/flip-link/pull/20
[#18]: https://github.com/knurling-rs/flip-link/pull/18

## [v0.1.2] - 2020-11-26

### Added
- Add README link to `Cargo.toml`

## [v0.1.1] - 2020-11-26

### Fixed
- [#17] attributes in linker scripts do not cause parse errors anymore (they are ignored)

[#17]: https://github.com/knurling-rs/flip-link/pull/17

## v0.1.0 - 2020-10-16

Initial release

[Unreleased]: https://github.com/knurling-rs/flip-link/compare/v0.1.4...main
[v0.1.4]: https://github.com/knurling-rs/flip-link/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/knurling-rs/flip-link/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/knurling-rs/flip-link/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/knurling-rs/flip-link/compare/v0.1.0...v0.1.1
