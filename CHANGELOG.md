# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- [#60]: Update to Rust 2021 🎉
- [#58] Print a message when linking normally fails

[#60]: https://github.com/knurling-rs/flip-link/pull/60
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

[Unreleased]: https://github.com/knurling-rs/flip-link/compare/v0.1.5...main
[v0.1.5]: https://github.com/knurling-rs/flip-link/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/knurling-rs/flip-link/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/knurling-rs/flip-link/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/knurling-rs/flip-link/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/knurling-rs/flip-link/compare/v0.1.0...v0.1.1
