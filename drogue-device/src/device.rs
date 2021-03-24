use drogue_device::{
    api::{uart::*},
    driver::{
        uart::{serial_rx::*, serial_tx::*},
    },
    platform::cortex_m::nrf::{
        uarte::{UarteRx, UarteTx},
    },
    prelude::*,
};
use nrf52833_hal as hal;
use hal::pac::UARTE0;

pub type AppTx = SerialTx<UarteTx<UARTE0>>;
pub type AppRx = SerialRx<App<AppTx>, UarteRx<UARTE0>>;

pub struct MyDevice {
    pub tx: ActorContext<AppTx>,
    pub rx: InterruptContext<AppRx>,
    pub app: ActorContext<App<AppTx>>,
}

impl Device for MyDevice {
    fn mount(&'static self, _: DeviceConfiguration<Self>, supervisor: &mut Supervisor) {
        let uart = self.tx.mount((), supervisor);
        let app = self.app.mount(
            AppConfig {
                uart,
            },
            supervisor,
        );

        self.rx.mount(app, supervisor);
    }
}

pub struct AppConfig<U>
where
    U: UartWriter + 'static,
{
    uart: Address<U>,
}

pub struct App<U>
where
    U: UartWriter + 'static,
{
    uart: Option<Address<U>>,
}

impl<U> App<U>
where
    U: UartWriter + 'static,
{
    pub fn new() -> Self {
        Self {
            uart: None,
        }
    }
}
impl<U> Actor for App<U>
where
    U: UartWriter + 'static,
{
    type Configuration = AppConfig<U>;
    fn on_mount(&mut self, _: Address<Self>, config: Self::Configuration) {
        self.uart.replace(config.uart);
    }

    fn on_start(self) -> Completion<Self> {
        Completion::defer(async move {
            self
        })
    }
}

impl<U> NotifyHandler<SerialData> for App<U>
where
    U: UartWriter + 'static,
{
    fn on_notify(self, event: SerialData) -> Completion<Self> {
        Completion::defer(async move {
            let mut buf = [0; 1];
            buf[0] = event.0;
            self.uart
                .as_ref()
                .unwrap()
                .write(&buf[..])
                .await
                .expect("error writing data");
            self
        })
    }
}
