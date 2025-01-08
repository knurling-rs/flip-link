#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
#[cfg(feature = "lm3s6965")]
use lm3s6965 as _;
use panic_semihosting as _;

static X: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

#[entry]
fn main() -> ! {
    hprintln!(
        "Hello, world!, X = {}",
        X.load(core::sync::atomic::Ordering::Relaxed)
    );

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
