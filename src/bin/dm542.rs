#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, OutputType, Pull, Speed};

use embassy_stm32::time::{khz, mhz};
use embassy_stm32::timer::low_level::CaptureCompare16bitInstance;
use embassy_stm32::timer::{Channel, CountingMode};
use embassy_time::Timer;
use embassy_stm32::{bind_interrupts, peripherals, timer, Config};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blinky(led: peripherals::PC13) {
    let mut led = Output::new(led, Level::High, Speed::Low);
    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(300).await;
        info!("low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}

// bind_interrupts!(struct Irqs {
//     TIM2 => timer::CaptureCompareInterruptHandler<peripherals::TIM2>;
// });

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello DM542 and 57 Stepper");

    unwrap!(spawner.spawn(blinky(p.PC13)));
 
    let step = PwmPin::new_ch1(p.PA6, OutputType::PushPull);
    let mut pwm1 = SimplePwm::new(p.TIM3, Some(step), None, None, None, khz(20), CountingMode::EdgeAlignedUp);
    let mut dir1 = Output::new(p.PA7, Level::High, Speed::Low);
    let max_duty = pwm1.get_max_duty();
    loop {
        pwm1.set_duty(timer::Channel::Ch1, max_duty * 7 / 10);
        pwm1.enable(Channel::Ch1);
        dir1.set_high();
        Timer::after_millis(2000).await;
    }
}