#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]

use panic_halt as _;
use cortex_m_rt::entry;
use embassy::{executor::{task, Executor}, util::Steal};
use embassy::traits::uart::{Read, Write};
use embassy::util::Forever;
use embassy_nrf::{Peripherals, gpio::NoPin, interrupt,  uarte};
use futures::pin_mut;

#[task]
async fn run() {
    let p = unsafe { Peripherals::steal() };

    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    let irq = interrupt::take!(UARTE0_UART0);
    let uart =
        unsafe { uarte::Uarte::new(p.uarte0, irq, p.p0_08, p.p0_06, NoPin, NoPin, config) };
    pin_mut!(uart);

    let mut buf = [0; 1];

    loop {
        let _ = uart.as_mut().read(&mut buf).await;
        let _ = uart.as_mut().write(&buf).await;
    }
}

static EXECUTOR: Forever<Executor> = Forever::new();

#[entry]
fn main() -> ! {
    let p = embassy_nrf::pac::Peripherals::take().unwrap();

    let executor = EXECUTOR.put(Executor::new());

    executor.run(|spawner| {
        spawner.spawn(run()).unwrap();
    });
}
