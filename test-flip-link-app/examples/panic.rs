#![no_main]
#![no_std]

use cortex_m_rt::entry;
#[cfg(feature = "lm3s6965")]
use lm3s6965 as _;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    panic!("Oops")
}
