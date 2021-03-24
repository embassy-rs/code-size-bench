#![no_main]
#![no_std]

mod device;

use panic_halt as _;

use cortex_m_rt::{entry, exception};
use drogue_device::{
    driver::uart::{serial_rx::*, serial_tx::*},
    platform::cortex_m::nrf::{
        uarte::{Baudrate, Parity, Pins, Uarte},
    },
    prelude::*,
};
use hal::gpio::Level;

use nrf52833_hal as hal;

use crate::device::*;

fn configure() -> MyDevice {
    let device = hal::pac::Peripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(device.P0);

    let clocks = hal::clocks::Clocks::new(device.CLOCK).enable_ext_hfosc();
    let _clocks = clocks.start_lfclk();

    // Uart
    static mut RX_BUF: [u8; 1] = [0; 1];
    let (tx, rx) = Uarte::new(
        device.UARTE0,
        Pins {
            txd: port0.p0_06.into_push_pull_output(Level::High).degrade(),
            rxd: port0.p0_08.into_floating_input().degrade(),
            cts: None,
            rts: None,
        },
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    )
    .split(unsafe { &mut RX_BUF });
    let tx = SerialTx::new(tx);
    let rx = SerialRx::new(rx);

    MyDevice {
        tx: ActorContext::new(tx).with_name("uart_tx"),
        rx: InterruptContext::new(rx, hal::pac::Interrupt::UARTE0_UART0).with_name("uart_rx"),
        app: ActorContext::new(App::new()),
    }
}

#[entry]
fn main() -> ! {
    device!(MyDevice = configure; 2048);
}
