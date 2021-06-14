#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
#[cfg(feature = "default")]
use lm3s6965 as _;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
