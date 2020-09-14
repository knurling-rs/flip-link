# `flip-link`

> adds zero-cost stack overflow protection to your embedded programs

## Usage

Change the linker from `rust-lld` (the default) to `flip-link` in `.cargo/config.toml`

``` toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# (..)
rustflags = [
  "-C", "linker=flip-link", # <- add this
  # (..)
]
```

NOTE that if you were using GNU `ld` or GNU `gcc` to link your program then this
won't work

## Support

`flip-link` is part of the [Knurling] project, [Ferrous Systems]' effort at
improving tooling used to develop for embedded systems.

If you think that our work is useful, consider sponsoring it via [GitHub
Sponsors].

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Knurling]: https://github.com/knurling-rs/meta
[Ferrous Systems]: https://ferrous-systems.com/
[GitHub Sponsors]: https://github.com/sponsors/knurling-rs
