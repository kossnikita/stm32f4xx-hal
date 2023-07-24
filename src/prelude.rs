//! Convenience re-export of multiple traits.
//!
//! This allows a HAL user to conveniently import this module and have all the
//! helper traits already imported.
//! Otherwise the use of peripherals would require the import of the
//! corresponding module and the import of the trait, which connects this HAL
//! to the autogenerated svd2rust API in [crate::pac].
//!
//! # Example
//!
//! Consider the following code.
//!
//! ```
//! #[entry]
//! fn main() -> ! {
//!     let dp = pac::Peripherals::take().unwrap();
//!     let gpiog = dp.GPIOG.split();
//!     let mut led1 = gpiog.pg13.into_push_pull_output();
//!     led1.set_high().unwrap();
//! }
//! ```
//!
//! Without the prelude we would have to import the following traits:
//!
//! ```
//! use stm32f4xx_hal::gpio::GpioExt; // for the split method.
//! use embedded_hal::digital::v2::OutputPin; // for the set_high() function.
//! // And more use-statements with more complex code.
//! ```
//!
//! These imports are a bit unintuitive, because we can create the objects
//! without the import. But we need these traits to access most of their
//! functions.
//!
//! The prelude module keeps the import section cleaner:
//! ```
//! use stm32f4xx_hal::prelude::*;
//! ```
pub use embedded_hal::adc::OneShot as _embedded_hal_adc_OneShot;
pub use embedded_hal::blocking::delay::DelayMs as _embedded_hal_blocking_delay_DelayMs;
pub use embedded_hal::blocking::delay::DelayUs as _embedded_hal_blocking_delay_DelayUs;
pub use embedded_hal::blocking::serial::Write as _embedded_hal_blocking_serial_Write;
pub use embedded_hal::serial::Read as _embedded_hal_serial_Read;
pub use embedded_hal::serial::Write as _embedded_hal_serial_Write;
pub use embedded_hal::Capture as _embedded_hal_Capture;
pub use embedded_hal::Pwm as _embedded_hal_Pwm;
pub use embedded_hal::Qei as _embedded_hal_Qei;
pub use fugit::ExtU32 as _fugit_ExtU32;
pub use fugit::RateExtU32 as _fugit_RateExtU32;

#[cfg(all(
    feature = "device-selected",
    feature = "can",
    any(feature = "can1", feature = "can2",)
))]
pub use crate::can::CanExt as _stm32f4xx_hal_can_CanExt;
#[cfg(all(feature = "device-selected", feature = "dac"))]
pub use crate::dac::DacExt as _stm32f4xx_hal_dac_DacExt;
pub use crate::dma::traits::DmaCommonInterruptsExt as _;
pub use crate::dma::traits::DmaFlagExt as _;
pub use crate::dma::traits::Stream as _;
pub use crate::dma::traits::StreamISR as _;
pub use crate::gpio::outport::OutPort as _;
pub use crate::gpio::ExtiPin as _stm32f4xx_hal_gpio_ExtiPin;
pub use crate::gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;
pub use crate::i2c::dma::I2CMasterHandleIT as _stm32f4xx_hal_i2c_dma_I2CMasterHandleIT;
pub use crate::i2c::dma::I2CMasterReadDMA as _stm32f4xx_hal_i2c_dma_I2CMasterReadDMA;
pub use crate::i2c::dma::I2CMasterWriteDMA as _stm32f4xx_hal_i2c_dma_I2CMasterWriteDMA;
pub use crate::i2c::dma::I2CMasterWriteReadDMA as _stm32f4xx_hal_i2c_dma_I2CMasterWriteReadDMA;
pub use crate::i2c::I2cExt as _stm32f4xx_hal_i2c_I2cExt;
pub use crate::i2s::I2sExt as _stm32f4xx_hal_i2s_I2sExt;
pub use crate::qei::QeiExt as _stm32f4xx_hal_QeiExt;
pub use crate::rcc::RccExt as _stm32f4xx_hal_rcc_RccExt;
#[cfg(all(feature = "device-selected", feature = "rng"))]
pub use crate::rng::RngExt as _stm32f4xx_hal_rng_RngExt;
pub use crate::serial::Listen as _stm32f4xx_hal_serial_Listen;
pub use crate::serial::RxISR as _stm32f4xx_hal_serial_RxISR;
pub use crate::serial::RxListen as _stm32f4xx_hal_serial_RxListen;
pub use crate::serial::SerialExt as _stm32f4xx_hal_serial_SerialExt;
pub use crate::serial::TxISR as _stm32f4xx_hal_serial_TxISR;
pub use crate::serial::TxListen as _stm32f4xx_hal_serial_TxListen;
pub use crate::spi::SpiExt as _stm32f4xx_hal_spi_SpiExt;
pub use crate::syscfg::SysCfgExt as _stm32f4xx_hal_syscfg_SysCfgExt;
pub use crate::time::U32Ext as _stm32f4xx_hal_time_U32Ext;
#[cfg(feature = "rtic")]
pub use crate::timer::MonoTimer64Ext as _;
#[cfg(feature = "rtic")]
pub use crate::timer::MonoTimerExt as _;
pub use crate::timer::PwmExt as _stm32f4xx_hal_timer_PwmExt;
#[cfg(feature = "rtic")]
pub use crate::timer::SysMonoTimerExt as _stm32f4xx_hal_timer_SysMonoTimerExt;
pub use crate::timer::SysTimerExt as _stm32f4xx_hal_timer_SysCounterExt;
pub use crate::timer::TimerExt as _stm32f4xx_hal_timer_TimerExt;
