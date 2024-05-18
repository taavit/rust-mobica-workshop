#![no_std]
#![no_main]

use bsp::hal::clocks::init_clocks_and_plls;
use bsp::hal::Sio;
use bsp::hal::Watchdog;
use defmt::debug;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

use bsp::hal as rp2040_hal;
use defmt_rtt as _;
use rp_pico as bsp;

use bsp::entry;
use bsp::pac;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut delay = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.gpio16.into_push_pull_output();
    // let button = pins.gpio18.into_pull_down_input();

    loop {
        led.set_high().unwrap();
        debug!("Set high");
        delay.delay_ms(250);
        led.set_low().unwrap();
        debug!("Set low");
        delay.delay_ms(250);
    }
}
