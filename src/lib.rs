#![no_std]

use esp32_hal::{
    clock_control::ClockControlConfig,
    gpio::{Gpio1, Gpio3, Unknown},
    prelude::*,
    serial::{config::Config, Pins, Rx, Serial, Tx},
    target::UART0,
};
use spin::Mutex;

pub static STORED_TX: Mutex<Option<Tx<UART0>>> = Mutex::new(None);
pub static STORED_RX: Mutex<Option<Rx<UART0>>> = Mutex::new(None);

/// Setup the logger on UART0
pub fn setup_logger(
    uart0: UART0,
    gpio1: Gpio1<Unknown>,
    gpio3: Gpio3<Unknown>,
    clock_control_config: ClockControlConfig,
) {
    use core::fmt::Write;

    let serial: Serial<_, _, _> = Serial::new(
        uart0,
        Pins {
            tx: gpio1,
            rx: gpio3,
            cts: None,
            rts: None,
        },
        Config::default().baudrate(115200.Hz()),
        clock_control_config,
    )
    .unwrap();

    let (tx, rx) = serial.split();

    *STORED_TX.lock() = Some(tx);
    *STORED_RX.lock() = Some(rx);

    let mut tx = STORED_TX.lock();
    let tx = tx.as_mut().unwrap();
    write!(tx, "\r\n[ INIT ] Initialized logger.\r\n").unwrap();
}

/// Log message
/// # Example
/// ```
/// use esp32_logger::*;
///
/// let value = 42;
/// log!("The current value is {}", value);
/// ```
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        if let Some(tx) = unsafe { STORED_TX.lock().as_mut() } {
            use core::fmt::Write;

            write!(tx, "[ LOG ] ").unwrap();
            write!(tx, $($arg)*).unwrap();
            write!(tx, "\r\n").unwrap();
        }
    );
}

/// Log message as a warning
/// # Example
/// ```
/// use esp32_logger::*;
///
/// let value = 42;
/// warn!("Something doesn't seem right... {}", value);
/// ```
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        if let Some(tx) = unsafe { STORED_TX.lock().as_mut() } {
            use core::fmt::Write;
            write!(tx, "[ WARN ] ").unwrap();
            write!(tx, $($arg)*).unwrap();
            write!(tx, "\r\n").unwrap();
        }
    );
}

/// Log message as an error
/// # Example
/// ```
/// use esp32_logger::*;
///
/// let value = 42;
/// error!("Okay something broke: {}", value);
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        if let Some(tx) = unsafe { STORED_TX.lock().as_mut() } {
            use core::fmt::Write;

            write!(tx, "[ ERROR ] ").unwrap();
            write!(tx, $($arg)*).unwrap();
            write!(tx, "\r\n").unwrap();
        }
    );
}
