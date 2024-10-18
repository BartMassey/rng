#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs};
use nrf52833_hal::{self as hal, pac};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let peripherals = pac::Peripherals::take().unwrap();
    let mut rng = hal::Rng::new(peripherals.RNG);
    let mut timer0 = hal::Timer::new(peripherals.TIMER0);

    loop {
        rprintln!("{}", rng.random_u32());
        timer0.delay_ms(1000);
    }
}
