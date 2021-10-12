#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy::traits::uart::{Read, Write};
use embassy::util::Steal;
use embassy_nrf::gpio::NoPin;
use embassy_nrf::{interrupt, uarte, Peripherals};
use futures::pin_mut;
use panic_halt as _;

mod block_on;

async fn run() -> ! {
    let p = unsafe { Peripherals::steal() };

    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    let irq = interrupt::take!(UARTE0_UART0);
    let uart = unsafe { uarte::Uarte::new(p.UARTE0, irq, p.P0_08, p.P0_06, NoPin, NoPin, config) };
    pin_mut!(uart);

    let mut buf = [0; 1];

    loop {
        let _ = uart.as_mut().read(&mut buf).await;
        let _ = uart.as_mut().write(&buf).await;
    }
}

#[entry]
fn main() -> ! {
    let _p = embassy_nrf::init(Default::default());
    block_on::block_on(run())
}
