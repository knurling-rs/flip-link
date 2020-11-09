//! Changing the panicking behavior
//!
//! The easiest way to change the panicking behavior is to use a different [panic handler crate][0].
//!
//! [0]: https://crates.io/keywords/panic-impl

#![no_main]
#![no_std]

// Pick one of these panic handlers:

// `panic!` halts execution; the panic message is ignored
use panic_semihosting as _;

use cortex_m_rt::entry;
use lm3s6965 as _;

#[entry]
fn main() -> ! {
    panic!("Oops")
}
