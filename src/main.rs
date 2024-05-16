#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::debug;
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use fugit::Duration;
use panic_probe as _;

use defmt_rtt as _;
use stm32f4xx_hal::{
    adc::{
        config::{AdcConfig, SampleTime},
        Adc,
    },
    gpio::GpioExt,
    pac::{self, ADC1},
    prelude::*,
    rcc::RccExt,
    timer::{Counter, SysTimerExt},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);
    let mut timer = dp.TIM2.counter_ms(&clocks);

    let mut led = gpioa.pa5.into_push_pull_output();
    let mut button = gpioc.pc13.into_pull_down_input();

    let mut adc: Adc<ADC1> = Adc::adc1(dp.ADC1, false, AdcConfig::default());
    let analog_pin = gpioa.pa0.into_analog();

    loop {
        // let signal = read_signal(&mut button, &mut timer);
        let signal = generate_signal(&mut adc, &analog_pin);
        blink_signal(&mut led, &mut delay, signal);
    }
}

enum Signal {
    Short,
    Long,
}

fn blink_signal<Pin: OutputPin>(led: &mut Pin, delay: &mut impl DelayNs, signal: Signal) {
    led.set_high().unwrap();
    match signal {
        Signal::Short => delay.delay_ms(250),
        Signal::Long => delay.delay_ms(1000),
    };
    led.set_low().unwrap();
    // So player can distinguish signals
    delay.delay_ms(250);
}

fn read_signal<Button: InputPin>(
    button: &mut Button,
    timer: &mut Counter<pac::TIM2, 1000>,
) -> Signal {
    while button.is_high().unwrap() {}
    timer.start(1.hours()).unwrap();
    while button.is_low().unwrap() {}
    let duration = timer.now().duration_since_epoch();
    debug!("Measured: {}", duration);
    timer.cancel().unwrap();
    if duration < Duration::<u32, 1, 1000>::millis(1000) {
        Signal::Short
    } else {
        Signal::Long
    }
}
type AnalogPin = stm32f4xx_hal::gpio::Pin<'A', 0, stm32f4xx_hal::gpio::Analog>;

fn generate_signal(adc: &mut Adc<ADC1>, pin: &AnalogPin) -> Signal {
    if adc.convert(pin, SampleTime::Cycles_3) % 2 == 0 {
        Signal::Long
    } else {
        Signal::Short
    }
}
