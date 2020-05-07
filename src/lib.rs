#![no_std]

pub use esp32::{APB_CTRL, DPORT, RTCCNTL, UART0};
pub use esp32_hal::clock_control::{ClockControl, XTAL_FREQUENCY_AUTO};
pub use esp32_hal::dport::Split;
pub use esp32_hal::prelude::*;
pub use esp32_hal::serial::config::Config;
pub use esp32_hal::serial::{NoRx, NoTx, Rx, Serial, Tx};

pub static STORED_TX: spin::Mutex<Option<Tx<UART0>>> = spin::Mutex::new(None);
pub static STORED_RX: spin::Mutex<Option<Rx<UART0>>> = spin::Mutex::new(None);

#[macro_export]
macro_rules! setup_logger {
    ($dp:expr) => {
        let (mut dport, dport_clock_control) = $dp.DPORT.split();
        let clock_control =
            ClockControl::new($dp.RTCCNTL, $dp.APB_CTRL, dport_clock_control, XTAL_FREQUENCY_AUTO).unwrap();

        let (clock_control_config, mut watchdog) = clock_control.freeze().unwrap();
        watchdog.disable();

        let serial = Serial::uart0(
            $dp.UART0,
            (NoTx, NoRx),
            Config::default().baudrate(115200.Hz()),
            clock_control_config,
            &mut dport,
        )
        .unwrap();

        let (tx, rx) = serial.split();

        unsafe {
            *STORED_TX.lock() = Some(tx);
            *STORED_RX.lock() = Some(rx);
        }
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        if let Some(tx) = unsafe { STORED_TX.lock().as_mut() } {
            write!(tx, "[ LOG ] ").unwrap();
            write!(tx, $($arg)*).unwrap();
            write!(tx, "\r\n").unwrap();
        }
    );
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        if let Some(tx) = unsafe { STORED_TX.lock().as_mut() } {
            write!(tx, "[ WARN ] ").unwrap();
            write!(tx, $($arg)*).unwrap();
            write!(tx, "\r\n").unwrap();
        }
    );
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        if let Some(tx) = unsafe { STORED_TX.lock().as_mut() } {
            write!(tx, "[ ERROR ] ").unwrap();
            write!(tx, $($arg)*).unwrap();
            write!(tx, "\r\n").unwrap();
        }
    );
}
