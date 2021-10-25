#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use cortex_m_rt::entry;
use embassy::traits::uart::{Read, Write};
use embassy::util::Forever;
use embassy::{executor::Executor};
use embassy_nrf::gpio::NoPin;
use embassy_nrf::{interrupt, uarte, peripherals::UARTE0};
use core::future::Future;
use drogue_device::*;
use panic_halt as _;

static EXECUTOR: Forever<Executor> = Forever::new();
static SERVER: Forever<ActorContext<'static, EchoServer>> = Forever::new();

#[entry]
fn main() -> ! {
    let executor = EXECUTOR.put(Executor::new());
    let p = embassy_nrf::init(Default::default());

    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    let irq = interrupt::take!(UARTE0_UART0);
    let uart = unsafe { uarte::Uarte::new(p.UARTE0, irq, p.P0_08, p.P0_06, NoPin, NoPin, config) };

    executor.run(|spawner| {
        let server = SERVER.put(ActorContext::new(EchoServer { uart }));
        server.mount((), spawner);
    })
}

pub struct EchoServer {
    uart: uarte::Uarte<'static, UARTE0>,
}

impl Actor for EchoServer {
    #[rustfmt::skip]
    type OnMountFuture<'m, M> where M: 'm, = impl Future<Output = ()> + 'm;

    fn on_mount<'m, M>(
        &'m mut self,
        _: Self::Configuration,
        _: Address<'static, Self>,
        _: &'m mut M,
    ) -> Self::OnMountFuture<'m, M>
    where
        M: Inbox<'m, Self> + 'm,
    {
        async move {
            let mut buf = [0; 1];
            loop {
                let _ = self.uart.read(&mut buf).await;
                let _ = self.uart.write(&buf).await;
            }
        }
    }
}
