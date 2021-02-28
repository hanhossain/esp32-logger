#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp32_hal::{
    clock_control::{sleep, ClockControl, XTAL_FREQUENCY_AUTO},
    dport::Split,
    prelude::*,
    target,
    timer::Timer,
};
use log::{debug, error, info, trace, warn, LevelFilter};

#[entry]
fn main() -> ! {
    let dp = target::Peripherals::take().expect("failed to acquire peripherals");
    let (_, dport_clock_control) = dp.DPORT.split();

    let clock_control = ClockControl::new(
        dp.RTCCNTL,
        dp.APB_CTRL,
        dport_clock_control,
        XTAL_FREQUENCY_AUTO,
    )
    .unwrap();

    // disable RTC watchdog
    let (clock_control_config, mut watchdog) = clock_control.freeze().unwrap();
    watchdog.disable();

    // disable MST watchdogs
    let (.., mut watchdog0) = Timer::new(dp.TIMG0, clock_control_config);
    let (.., mut watchdog1) = Timer::new(dp.TIMG1, clock_control_config);
    watchdog0.disable();
    watchdog1.disable();

    esp32_logger::init_with_level(LevelFilter::Trace);

    loop {
        trace!("Time to trace it");
        debug!("Crush those bugs");
        info!("Some informational statement");
        warn!("Be careful!");
        error!("Uh oh, something broke");

        sleep(1.s());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    loop {}
}
