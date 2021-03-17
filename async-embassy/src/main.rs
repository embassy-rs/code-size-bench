#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]

use panic_halt as _;
use cortex_m_rt::entry;
use embassy::executor::{task, Executor};
use embassy::time::{Duration, Timer};
use embassy::traits::uart::Uart;
use embassy::util::Forever;
use embassy_nrf::{interrupt, pac, rtc, uarte};
use futures::future::{select, Either};
use nrf52840_hal::clocks;
use nrf52840_hal::gpio;

#[task]
async fn run(uart: pac::UARTE0, port: pac::P0) {
    // Init UART
    let port0 = gpio::p0::Parts::new(port);

    let pins = uarte::Pins {
        rxd: port0.p0_08.into_floating_input().degrade(),
        txd: port0
            .p0_06
            .into_push_pull_output(gpio::Level::Low)
            .degrade(),
        cts: None,
        rts: None,
    };

    // NOTE(unsafe): Safe becasue we do not use `mem::forget` anywhere.
    let mut uart = unsafe {
        uarte::Uarte::new(
            uart,
            interrupt::take!(UARTE0_UART0),
            pins,
            uarte::Parity::EXCLUDED,
            uarte::Baudrate::BAUD115200,
        )
    };

    loop {
        let mut buf = [0u8; 1];
        uart.receive(&mut buf).await.unwrap();
        uart.send(&buf).await.unwrap();
    }
}

static EXECUTOR: Forever<Executor> = Forever::new();

#[entry]
fn main() -> ! {
    let p = embassy_nrf::pac::Peripherals::take().unwrap();

    let executor = EXECUTOR.put(Executor::new());

    let uarte0 = p.UARTE0;
    let p0 = p.P0;
    executor.run(|spawner| {
        spawner.spawn(run(uarte0, p0)).unwrap();
    });
}
