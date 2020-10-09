# esp32-logger

This is a simple logger to make logging easy on the ESP32.

## Example
```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp32_hal::{
    clock_control::{ClockControl, XTAL_FREQUENCY_AUTO},
    dport::Split,
    prelude::*,
    target,
    timer::Timer,
};

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

  let gpios = dp.GPIO.split();

  esp32_logger::init(dp.UART0, gpios.gpio1, gpios.gpio3, clock_control_config);
  esp32_logger::log!("This is a valid log!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    esp32_logger::error!("\r\n{:?}", info);
    loop {}
}
```

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
