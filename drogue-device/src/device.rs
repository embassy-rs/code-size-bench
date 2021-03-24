use drogue_device::{
    api::{delayer::*, uart::*},
    domain::time::duration::Milliseconds,
    driver::{
        led::*,
        timer::*,
        uart::{serial_rx::*, serial_tx::*},
    },
    platform::cortex_m::nrf::{
        timer::Timer as HalTimer,
        uarte::{UarteRx, UarteTx},
    },
    prelude::*,
};
use hal::gpio::{Output, Pin, PushPull};
use hal::pac::{TIMER0, UARTE0};
use heapless::consts;
use nrf52833_hal as hal;

pub type AppTimer = Timer<HalTimer<TIMER0>>;
pub type AppTx = SerialTx<UarteTx<UARTE0>>;
pub type AppRx = SerialRx<App<AppTx, <AppTimer as Package>::Primary>, UarteRx<UARTE0>>;

pub struct MyDevice {
    pub timer: AppTimer,
    pub tx: ActorContext<AppTx>,
    pub rx: InterruptContext<AppRx>,
    pub app: ActorContext<App<AppTx, <AppTimer as Package>::Primary>>,
}

impl Device for MyDevice {
    fn mount(&'static self, _: DeviceConfiguration<Self>, supervisor: &mut Supervisor) {
        let timer = self.timer.mount((), supervisor);
        let uart = self.tx.mount((), supervisor);
        let app = self.app.mount(
            AppConfig {
                uart,
                timer,
            },
            supervisor,
        );

        self.rx.mount(app, supervisor);
    }
}

pub struct AppConfig<U, D>
where
    U: UartWriter + 'static,
    D: Delayer + 'static,
{
    uart: Address<U>,
    timer: Address<D>,
}

pub struct App<U, D>
where
    U: UartWriter + 'static,
    D: Delayer + 'static,
{
    uart: Option<Address<U>>,
    timer: Option<Address<D>>,
}

impl<U, D> App<U, D>
where
    U: UartWriter + 'static,
    D: Delayer,
{
    pub fn new() -> Self {
        Self {
            uart: None,
            timer: None,
        }
    }
}
impl<U, D> Actor for App<U, D>
where
    U: UartWriter + 'static,
    D: Delayer,
{
    type Configuration = AppConfig<U, D>;
    fn on_mount(&mut self, _: Address<Self>, config: Self::Configuration) {
        self.uart.replace(config.uart);
        self.timer.replace(config.timer);
    }

    fn on_start(self) -> Completion<Self> {
        Completion::defer(async move {
            self
        })
    }
}

impl<U, D> NotifyHandler<SerialData> for App<U, D>
where
    U: UartWriter + 'static,
    D: Delayer + 'static,
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
