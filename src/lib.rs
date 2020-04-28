#![no_std]

use core::fmt::Write;
use esp32::{APB_CTRL, DPORT, RTCCNTL, UART0};
use esp32_hal::clock_control::{ClockControl, XTAL_FREQUENCY_AUTO};
use esp32_hal::dport::Split;
use esp32_hal::prelude::*;
use esp32_hal::serial::config::Config;
use esp32_hal::serial::{NoRx, NoTx, Rx, Serial, Tx};

static mut STORED_TX: Option<Tx<UART0>> = None;
static mut STORED_RX: Option<Rx<UART0>> = None;

pub fn setup(uart0: UART0, rtccntl: RTCCNTL, apb_ctrl: APB_CTRL, dport: DPORT) {
    let (mut dport, dport_clock_control) = dport.split();
    let clock_control =
        ClockControl::new(rtccntl, apb_ctrl, dport_clock_control, XTAL_FREQUENCY_AUTO).unwrap();

    let (clock_control_config, mut watchdog) = clock_control.freeze().unwrap();
    watchdog.disable();

    let serial = Serial::uart0(
        uart0,
        (NoTx, NoRx),
        Config::default().baudrate(115200.Hz()),
        clock_control_config,
        &mut dport,
    )
    .unwrap();

    let (tx, rx) = serial.split();

    unsafe {
        STORED_TX = Some(tx);
        STORED_RX = Some(rx);
    }
}

pub unsafe fn log(msg: &str) {
    if let Some(tx) = &mut STORED_TX {
        write!(tx, "{}\r\n", msg).unwrap();
    }
}
