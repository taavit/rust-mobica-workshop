#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::debug;
use embedded_hal::delay::DelayNs;
use panic_probe as _;

use defmt_rtt as _;
use stm32f4xx_hal::{gpio::GpioExt, pac, prelude::*, rcc::RccExt, timer::SysTimerExt};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        led.set_high();
        debug!("Set high");
        delay.delay_ms(250);
        led.set_low();
        debug!("Set low");
        delay.delay_ms(250);
    }
}
