# `flip-link`

> adds zero-cost stack overflow protection to your embedded programs

## The problem

Bare metal Rust programs may *not* be memory safe in presence of stack overflows.
For example, this is the case for Rust programs based on v0.6.x of the `cortex-m-rt` crate.

The following program, which contains no `unsafe` code block, can run into *undefined behavior* if it reaches a stack overflow condition.

``` rust
// static variables placed in the .bss / .data sections
static FLAG1: AtomicBool = AtomicU32::new(false); // .bss
static FLAG2: AtomicBool = AtomicU32::new(true);  // .data

fn main() {
    let _x = fib(100);
}

#[inline(never)]
fn fib(n: u32) -> u32 {
    // allocate and initialize 4 kilobytes of stack memory
    let _use_stack = [0xAA; 1024];

    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2) // recursion
    }
}

#[interrupt]
fn interrupt_handler() {
    // does some operation with `FLAG1` and `FLAG2`
}
```

The default memory layout of ARM Cortex-M programs in RAM is shown below.

<p align="center">
  <img src="assets/overflow.svg" alt="left: default memory layout of ARM Cortex-M programs; right: stack overflow condition">
</p>

The function call stack, also known as the "stack", grows downwards on function calls and when local variables (e.g. `let x`) are created (these variables are also placed on the stack).

If the stack grows too large it collides with the `.bss + .data` region, which contains all the program's static variables. The collision results in the static variables being overwritten with unrelated data. This can result in the program observing the static variables in an invalid state: for example an `AtomicBool` may hold the value `3` -- this is undefined behavior because the Rust ABI expects this single-byte variable to be either `0` or `1`.

## The solution

One potential solution is to change the memory layout of the program and place the stack *below* the `.bss+.data` region.

With this flipped memory layout (pictured below) the stack cannot collide with the static variables. Instead it will collide with the boundary of the physical RAM memory region. In the ARM Cortex-M architecture, trying to read or write past the boundaries of the RAM region produces a "hardware exception". The `cortex-m-rt` crate provides an API to handle this condition: a `HardFault` exception handler can be defined; this "handler" (function) will be executed when the invalid memory operation is attempted.

<p align="center">
  <img src="assets/flipped.svg" alt="left: flipped memory layout; right: stack overflow condition">
</p>

`flip-link` implements this stack overflow solution. Linking your program with `flip-link` produces the flipped memory layout, which is memory safe in presence of stack overflows.

## Architecture support

`flip-link` is known to work with ARM Cortex-M programs that link to version `0.6.x` of the [`cortex-m-rt`] crate and are linked using the linker shipped with the Rust toolchain (LLD).
At this time, it hasn't been tested with other architectures or runtime crates.

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

## Installation

`flip-link` is available on [crates.io]. To install it, run

[crates.io]: https://crates.io/crates/flip-link

```console
$ cargo install flip-link
```

## Usage

Change the linker from `rust-lld` (the default) to `flip-link` in `.cargo/config.toml`

``` toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# (..)
linker = "flip-link"
```

In versions of Cargo < 1.74, use `rustflags` to change the linker

``` toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# (..)
rustflags = [
  "-C", "linker=flip-link", # <- add this
  # (..)
]
```

NOTE that if you were using GNU `ld` or GNU `gcc` to link your program then this won't work. Support for other linkers is being tracked in [issue #1].

[issue #1]: https://github.com/knurling-rs/flip-link/issues/1

## Testing

Our CI enforces various checks. You can run them locally to make sure your PR will pass the CI:

* `cargo fmt --all -- --check`
* `cargo clippy -- --deny warnings`
* `cargo xtest`
  * This installs the current revision of `flip-link` and runs `cargo test`.

## Logging

If you want to see what `flip-link` is up to, you can set these environment variables:

```bash
export RUSTC_LOG=rustc_codegen_ssa::back::link=info
export RUST_LOG=info
```

This will produce something like:

```console
$ cargo build
...
 INFO rustc_codegen_ssa::back::link linker stderr:
 [INFO  flip_link] found MemoryEntry(line=3, origin=0x20000000, length=0x10000) in ./target/thumbv7em-none-eabi/debug/build/lm3s6965-3b7087c63b161e04/out/memory.x
 [INFO  flip_link] used RAM spans: origin=0x20000000, length=12, align=4
 [INFO  flip_link] new RAM region: ORIGIN=0x2000fff0, LENGTH=16
 INFO rustc_codegen_ssa::back::link linker stdout:
 INFO rustc_codegen_ssa::back::link linker stdout:
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
```

You can see even more detail about how we parse expressions using `RUST_LOG=debug`.

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

[Knurling]: https://knurling.ferrous-systems.com
[Ferrous Systems]: https://ferrous-systems.com/
[GitHub Sponsors]: https://github.com/sponsors/knurling-rs
