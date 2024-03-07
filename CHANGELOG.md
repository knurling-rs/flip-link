# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- [#90] Configure release-plz
- [#88] Setup release-plz

[#90]: https://github.com/knurling-rs/flip-link/pull/86
[#88]: https://github.com/knurling-rs/flip-link/pull/86

## [v0.1.8] - 2024-03-06

- [#86]: Release v0.1.8
- [#85]: Setup cargo-dist
- [#84]: Fix for comments in linker script

[#86]: https://github.com/knurling-rs/flip-link/pull/86
[#85]: https://github.com/knurling-rs/flip-link/pull/85
[#84]: https://github.com/knurling-rs/flip-link/pull/84

## [v0.1.7] - 2023-07-20

- [#79]: Summer cleanup
- [#77]: CI: Switch from bors to github merge queue
- [#75]: End of year refactoring
- [#74]: CI: Simplify
- [#72]: CI: Install Rust manually
- [#71]: CI: Add changelog enforcer
- [#70]: Support addition in ORIGIN

[#79]: https://github.com/knurling-rs/flip-link/pull/79
[#77]: https://github.com/knurling-rs/flip-link/pull/77
[#75]: https://github.com/knurling-rs/flip-link/pull/75
[#74]: https://github.com/knurling-rs/flip-link/pull/74
[#72]: https://github.com/knurling-rs/flip-link/pull/72
[#71]: https://github.com/knurling-rs/flip-link/pull/71
[#70]: https://github.com/knurling-rs/flip-link/pull/70

## [v0.1.6] - 2022-03-23

### Fixed

- [#63]: Handles `memory.x` overrides by searching in the current working directory first.

[#63]: https://github.com/knurling-rs/flip-link/pull/63

### Changed

- [#60]: Update to Rust 2021 🎉. Requires Rust 1.56+ to build

[#60]: https://github.com/knurling-rs/flip-link/pull/60

### Added

- [#58] Print a message when linking normally fails. This makes it clearer that the failure is not due to `flip-link`.

[#58]: https://github.com/knurling-rs/flip-link/pull/58

## [v0.1.5] - 2021-08-27

- [#55] Drop `anyhow`
- [#54] update & upgrade
- [#53] `xtest`: Clear `test-flip-link-app`s `target/`-dir before each run
- [#52] `xtest`: Pass `--force` to `cargo install`
- [#51] Avoid the `tempfile` dependency
- [#49] Fix Clippy warnings
- [#45] `Cargo.toml`: Disable default features of `env_logger`
- [#44] Transfer non-static CI steps into `cargo xtest` command
- [#42] Add tests to check problem with `memory.x` in project root
- [#41] Verify initial stack-pointer to be inside static RAM
- [#40] Do linking test as part of cargo test; cleanup

[#55]: https://github.com/knurling-rs/flip-link/pull/55
[#54]: https://github.com/knurling-rs/flip-link/pull/54
[#53]: https://github.com/knurling-rs/flip-link/pull/53
[#52]: https://github.com/knurling-rs/flip-link/pull/52
[#51]: https://github.com/knurling-rs/flip-link/pull/51
[#49]: https://github.com/knurling-rs/flip-link/pull/49
[#46]: https://github.com/knurling-rs/flip-link/pull/46
[#42]: https://github.com/knurling-rs/flip-link/pull/42
[#45]: https://github.com/knurling-rs/flip-link/pull/45
[#44]: https://github.com/knurling-rs/flip-link/pull/44
[#41]: https://github.com/knurling-rs/flip-link/pull/41
[#40]: https://github.com/knurling-rs/flip-link/pull/40

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

[Unreleased]: https://github.com/knurling-rs/flip-link/compare/v0.1.8...main
[v0.1.7]: https://github.com/knurling-rs/flip-link/compare/v0.1.7...v0.1.8
[v0.1.7]: https://github.com/knurling-rs/flip-link/compare/v0.1.6...v0.1.7
[v0.1.6]: https://github.com/knurling-rs/flip-link/compare/v0.1.5...v0.1.g
[v0.1.5]: https://github.com/knurling-rs/flip-link/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/knurling-rs/flip-link/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/knurling-rs/flip-link/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/knurling-rs/flip-link/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/knurling-rs/flip-link/compare/v0.1.0...v0.1.1
