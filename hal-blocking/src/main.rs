#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_hal::{gpio, uarte};
use nrf52840_pac as pac;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    // Init UART
    let port0 = gpio::p0::Parts::new(p.P0);

    let pins = uarte::Pins {
        rxd: port0.p0_08.into_floating_input().degrade(),
        txd: port0
            .p0_06
            .into_push_pull_output(gpio::Level::Low)
            .degrade(),
        cts: None,
        rts: None,
    };

    let mut uart = uarte::Uarte::new(
        p.UARTE0,
        pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    loop {
        let mut buf = [0u8; 1];
        let _ = uart.read(&mut buf);
        let _ = uart.write(&buf);
    }
}
