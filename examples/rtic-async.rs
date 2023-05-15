#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;
use rtic_monotonics::systick::*;
use fugit::ExtU64;
use stm32f4xx_hal::{
    gpio::{Output, PC13},
    pac,
    prelude::*,
};
use rtic2 as rtic;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Initialize the systick interrupt & obtain the token to prove that we did
        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 48_000_000, systick_mono_token);

        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();
        defmt::info!("Start");

        tick::spawn().ok();
        (Shared {}, Local { led })
    }


    #[task(local = [led])]
    async fn tick(cx: blink::Context) {
        loop {
            ctx.local.led.toggle();
            defmt::info!("Tick");
            Systick::delay(1000.millis()).await;
        }
    }
}
