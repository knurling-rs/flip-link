#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::{peripheral::syst::SystClkSource, Peripherals};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprint;
#[cfg(feature = "lm3s6965")]
use lm3s6965 as _;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    loop {}
}

#[exception]
fn SysTick() {
    hprint!(".").unwrap();
}
