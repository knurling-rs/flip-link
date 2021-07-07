#![no_main]
#![no_std]

use core::ptr;

use cortex_m_rt::entry;
#[cfg(feature = "lm3s6965")]
use lm3s6965 as _;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    unsafe {
        // read an address outside of the RAM region; this causes a HardFault exception
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }

    loop {}
}
