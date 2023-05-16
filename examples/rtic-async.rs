#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;
use rtic_monotonics::systick::*;
use fugit::ExtU32;
use stm32f4xx_hal::{
    gpio::{Output, PC13},
    prelude::*,
};
use rtic::app;

#[app(device = stm32f4xx_hal::pac, dispatchers = [USART1], peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PC13<Output>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let rcc = ctx.device.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Initialize the systick interrupt & obtain the token to prove that we did
        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(ctx.core.SYST, 48_000_000, systick_mono_token);

        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();
        defmt::info!("Start");

        tick::spawn().ok();
        (Shared {}, Local { led })
    }


    #[task(local = [led, count: u32 = 0])]
    async fn tick(ctx: tick::Context) {
        loop {
            ctx.local.led.toggle();
            *ctx.local.count += 1;
            defmt::info!("Tick {}", *ctx.local.count);
            Systick::delay(500.millis()).await;
        }
    }
}
