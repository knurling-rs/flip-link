#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use lm3s6965 as _;

#[entry]
fn main() -> ! {
    panic!("Oops")
}
