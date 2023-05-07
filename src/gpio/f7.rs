// auto-generated using codegen
// STM32CubeMX DB release: DB.6.0.50
pub use super::*;

pub use super::Input as DefaultMode;

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 10]),
    PA1: (pa1, 1, [1, 2, 7, 8, 9, 10]),
    PA2: (pa2, 2, [1, 2, 3, 7, 8]),
    PA3: (pa3, 3, [1, 2, 3, 7, 10]),
    PA4: (pa4, 4, [5, 6, 7, 12]),
    PA5: (pa5, 5, [1, 3, 5, 10]),
    PA6: (pa6, 6, [1, 2, 3, 5, 9]),
    PA7: (pa7, 7, [1, 2, 3, 5, 9, 12]),
    PA8: (pa8, 8, [0, 1, 3, 4, 7, 10]),
    PA9: (pa9, 9, [1, 4, 5, 7]),
    PA10: (pa10, 10, [1, 7, 10]),
    PA11: (pa11, 11, [1, 7, 9, 10]),
    PA12: (pa12, 12, [1, 7, 8, 9, 10]),
    PA13: (pa13, 13, [0], super::Debugger),
    PA14: (pa14, 14, [0], super::Debugger),
    PA15: (pa15, 15, [0, 1, 5, 6, 8], super::Debugger),
]);

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 8, 10]),
    PB1: (pb1, 1, [1, 2, 3, 10]),
    PB2: (pb2, 2, [6, 7, 9]),
    PB3: (pb3, 3, [0, 1, 5, 6, 10], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7, 10], super::Debugger),
    PB5: (pb5, 5, [2, 4, 5, 6, 10, 12]),
    PB6: (pb6, 6, [2, 4, 7, 10, 12]),
    PB7: (pb7, 7, [2, 4, 7, 12]),
    PB8: (pb8, 8, [2, 3, 4, 9, 10, 12]),
    PB9: (pb9, 9, [2, 3, 4, 5, 9, 10, 12]),
    PB10: (pb10, 10, [1, 4, 5, 7, 10]),
    PB11: (pb11, 11, [1, 4, 7, 10]),
    PB12: (pb12, 12, [1, 4, 5, 7, 10, 12]),
    PB13: (pb13, 13, [1, 5, 7, 10]),
    PB14: (pb14, 14, [1, 3, 5, 7, 9, 10, 12]),
    PB15: (pb15, 15, [0, 1, 3, 5, 9, 10, 12]),
]);

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [8, 10, 12]),
    PC1: (pc1, 1, [0, 5, 6]),
    PC2: (pc2, 2, [5, 10, 12]),
    PC3: (pc3, 3, [5, 10, 12]),
    PC4: (pc4, 4, [5, 12]),
    PC5: (pc5, 5, [12]),
    PC6: (pc6, 6, [2, 3, 5, 8, 10, 12]),
    PC7: (pc7, 7, [2, 3, 6, 8, 10, 12]),
    PC8: (pc8, 8, [0, 2, 3, 7, 8, 12]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 7, 9, 12]),
    PC10: (pc10, 10, [6, 7, 8, 9, 12]),
    PC11: (pc11, 11, [6, 7, 8, 9, 12]),
    PC12: (pc12, 12, [0, 6, 7, 8, 12]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [9, 12]),
    PD1: (pd1, 1, [9, 12]),
    PD2: (pd2, 2, [0, 2, 8, 12]),
    PD3: (pd3, 3, [5, 7, 12]),
    PD4: (pd4, 4, [7, 12]),
    PD5: (pd5, 5, [7, 12]),
    PD6: (pd6, 6, [5, 6, 7, 11, 12]),
    PD7: (pd7, 7, [7, 11, 12]),
    PD8: (pd8, 8, [7, 12]),
    PD9: (pd9, 9, [7, 12]),
    PD10: (pd10, 10, [7, 12]),
    PD11: (pd11, 11, [7, 9, 10, 12]),
    PD12: (pd12, 12, [2, 3, 7, 9, 10, 12]),
    PD13: (pd13, 13, [2, 3, 9, 10, 12]),
    PD14: (pd14, 14, [2, 8, 12]),
    PD15: (pd15, 15, [2, 8, 12]),
]);

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 3, 8, 10, 12]),
    PE1: (pe1, 1, [3, 8, 12]),
    PE2: (pe2, 2, [0, 5, 6, 9, 12]),
    PE3: (pe3, 3, [0, 6, 12]),
    PE4: (pe4, 4, [0, 5, 6, 12]),
    PE5: (pe5, 5, [0, 3, 5, 6, 12]),
    PE6: (pe6, 6, [0, 1, 3, 5, 6, 10, 12]),
    PE7: (pe7, 7, [1, 8, 10, 12]),
    PE8: (pe8, 8, [1, 8, 10, 12]),
    PE9: (pe9, 9, [1, 8, 10, 12]),
    PE10: (pe10, 10, [1, 8, 10, 12]),
    PE11: (pe11, 11, [1, 5, 10, 12]),
    PE12: (pe12, 12, [1, 5, 10, 12]),
    PE13: (pe13, 13, [1, 5, 10, 12]),
    PE14: (pe14, 14, [1, 5, 10, 12]),
    PE15: (pe15, 15, [1, 12]),
]);

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12]),
    PF1: (pf1, 1, [4, 12]),
    PF2: (pf2, 2, [4, 12]),
    PF3: (pf3, 3, [12]),
    PF4: (pf4, 4, [12]),
    PF5: (pf5, 5, [12]),
    PF6: (pf6, 6, [3, 5, 6, 8, 9]),
    PF7: (pf7, 7, [3, 5, 6, 8, 9]),
    PF8: (pf8, 8, [5, 6, 8, 9, 10]),
    PF9: (pf9, 9, [5, 6, 8, 9, 10]),
    PF10: (pf10, 10, []),
    PF11: (pf11, 11, [5, 10, 12]),
    PF12: (pf12, 12, [12]),
    PF13: (pf13, 13, [12]),
    PF14: (pf14, 14, [12]),
    PF15: (pf15, 15, [12]),
]);

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [12]),
    PG1: (pg1, 1, [12]),
    PG2: (pg2, 2, [12]),
    PG3: (pg3, 3, [12]),
    PG4: (pg4, 4, [12]),
    PG5: (pg5, 5, [12]),
    PG6: (pg6, 6, []),
    PG7: (pg7, 7, [8, 12]),
    PG8: (pg8, 8, [8, 12]),
    PG9: (pg9, 9, [8, 9, 10, 11, 12]),
    PG10: (pg10, 10, [10, 11, 12]),
    PG11: (pg11, 11, [10, 12]),
    PG12: (pg12, 12, [3, 8, 11, 12]),
    PG13: (pg13, 13, [0, 3, 8, 12]),
    PG14: (pg14, 14, [0, 3, 8, 9, 12]),
    PG15: (pg15, 15, [8, 12]),
]);

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
    PH2: (ph2, 2, [3, 9, 10, 12]),
    PH3: (ph3, 3, [9, 10, 12]),
    PH4: (ph4, 4, [4, 10]),
    PH5: (ph5, 5, [4, 5, 12]),
    PH6: (ph6, 6, [4, 5, 9, 12]),
    PH7: (ph7, 7, [4, 5, 12]),
    PH8: (ph8, 8, [4, 12]),
    PH9: (ph9, 9, [4, 9, 12]),
    PH10: (ph10, 10, [2, 12]),
    PH11: (ph11, 11, [2, 12]),
    PH12: (ph12, 12, [2, 12]),
    PH13: (ph13, 13, [3, 8, 9, 12]),
    PH14: (ph14, 14, [3, 8, 9, 12]),
    PH15: (ph15, 15, [3, 12]),
]);

#[cfg(feature = "gpio-f72x")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI0: (pi0, 0, [2, 5, 12]),
    PI1: (pi1, 1, [3, 5, 12]),
    PI2: (pi2, 2, [3, 5, 12]),
    PI3: (pi3, 3, [3, 5, 12]),
    PI4: (pi4, 4, [3, 10, 12]),
    PI5: (pi5, 5, [3, 10, 12]),
    PI6: (pi6, 6, [3, 10, 12]),
    PI7: (pi7, 7, [3, 10, 12]),
    PI8: (pi8, 8, []),
    PI9: (pi9, 9, [8, 9, 12]),
    PI10: (pi10, 10, [12]),
    PI11: (pi11, 11, [10]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 10, 11]),
    PA1: (pa1, 1, [1, 2, 7, 8, 9, 10, 11, 14]),
    PA2: (pa2, 2, [1, 2, 3, 7, 8, 11, 14]),
    PA3: (pa3, 3, [1, 2, 3, 7, 10, 11, 14]),
    PA4: (pa4, 4, [5, 6, 7, 12, 13, 14]),
    PA5: (pa5, 5, [1, 3, 5, 10, 14]),
    PA6: (pa6, 6, [1, 2, 3, 5, 9, 13, 14]),
    PA7: (pa7, 7, [1, 2, 3, 5, 9, 11, 12]),
    PA8: (pa8, 8, [0, 1, 3, 4, 7, 10, 14]),
    PA9: (pa9, 9, [1, 4, 5, 7, 13]),
    PA10: (pa10, 10, [1, 7, 10, 13]),
    PA11: (pa11, 11, [1, 7, 9, 10, 14]),
    PA12: (pa12, 12, [1, 7, 8, 9, 10, 14]),
    PA13: (pa13, 13, [0], super::Debugger),
    PA14: (pa14, 14, [0], super::Debugger),
    PA15: (pa15, 15, [0, 1, 4, 5, 6, 8], super::Debugger),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 8, 9, 10, 11]),
    PB1: (pb1, 1, [1, 2, 3, 9, 10, 11]),
    PB2: (pb2, 2, [6, 7, 9]),
    PB3: (pb3, 3, [0, 1, 5, 6], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7], super::Debugger),
    PB5: (pb5, 5, [2, 4, 5, 6, 9, 10, 11, 12, 13]),
    PB6: (pb6, 6, [2, 3, 4, 7, 9, 10, 12, 13]),
    PB7: (pb7, 7, [2, 4, 7, 12, 13]),
    PB8: (pb8, 8, [2, 3, 4, 9, 11, 12, 13, 14]),
    PB9: (pb9, 9, [2, 3, 4, 5, 9, 12, 13, 14]),
    PB10: (pb10, 10, [1, 4, 5, 7, 10, 11, 14]),
    PB11: (pb11, 11, [1, 4, 7, 10, 11, 14]),
    PB12: (pb12, 12, [1, 4, 5, 7, 9, 10, 11, 12]),
    PB13: (pb13, 13, [1, 5, 7, 9, 10, 11]),
    PB14: (pb14, 14, [1, 3, 5, 7, 9, 12]),
    PB15: (pb15, 15, [0, 1, 3, 5, 9, 12]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [8, 10, 12, 14]),
    PC1: (pc1, 1, [0, 5, 6, 11]),
    PC2: (pc2, 2, [5, 10, 11, 12]),
    PC3: (pc3, 3, [5, 10, 11, 12]),
    PC4: (pc4, 4, [5, 8, 11, 12]),
    PC5: (pc5, 5, [8, 11, 12]),
    PC6: (pc6, 6, [2, 3, 5, 8, 12, 13, 14]),
    PC7: (pc7, 7, [2, 3, 6, 8, 12, 13, 14]),
    PC8: (pc8, 8, [0, 2, 3, 7, 8, 12, 13]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 7, 9, 12, 13]),
    PC10: (pc10, 10, [6, 7, 8, 9, 12, 13, 14]),
    PC11: (pc11, 11, [6, 7, 8, 9, 12, 13]),
    PC12: (pc12, 12, [0, 6, 7, 8, 12, 13]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [9, 12]),
    PD1: (pd1, 1, [9, 12]),
    PD2: (pd2, 2, [0, 2, 8, 12, 13]),
    PD3: (pd3, 3, [5, 7, 12, 13, 14]),
    PD4: (pd4, 4, [7, 12]),
    PD5: (pd5, 5, [7, 12]),
    PD6: (pd6, 6, [5, 6, 7, 12, 13, 14]),
    PD7: (pd7, 7, [7, 8, 12]),
    PD8: (pd8, 8, [7, 8, 12]),
    PD9: (pd9, 9, [7, 12]),
    PD10: (pd10, 10, [7, 12, 14]),
    PD11: (pd11, 11, [4, 7, 9, 10, 12]),
    PD12: (pd12, 12, [2, 3, 4, 7, 9, 10, 12]),
    PD13: (pd13, 13, [2, 3, 4, 9, 10, 12]),
    PD14: (pd14, 14, [2, 8, 12]),
    PD15: (pd15, 15, [2, 8, 12]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 3, 8, 10, 12, 13]),
    PE1: (pe1, 1, [3, 8, 12, 13]),
    PE2: (pe2, 2, [0, 5, 6, 9, 11, 12]),
    PE3: (pe3, 3, [0, 6, 12]),
    PE4: (pe4, 4, [0, 5, 6, 12, 13, 14]),
    PE5: (pe5, 5, [0, 3, 5, 6, 12, 13, 14]),
    PE6: (pe6, 6, [0, 1, 3, 5, 6, 10, 12, 13, 14]),
    PE7: (pe7, 7, [1, 8, 10, 12]),
    PE8: (pe8, 8, [1, 8, 10, 12]),
    PE9: (pe9, 9, [1, 8, 10, 12]),
    PE10: (pe10, 10, [1, 8, 10, 12]),
    PE11: (pe11, 11, [1, 5, 10, 12, 14]),
    PE12: (pe12, 12, [1, 5, 10, 12, 14]),
    PE13: (pe13, 13, [1, 5, 10, 12, 14]),
    PE14: (pe14, 14, [1, 5, 10, 12, 14]),
    PE15: (pe15, 15, [1, 12, 14]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12]),
    PF1: (pf1, 1, [4, 12]),
    PF2: (pf2, 2, [4, 12]),
    PF3: (pf3, 3, [12]),
    PF4: (pf4, 4, [12]),
    PF5: (pf5, 5, [12]),
    PF6: (pf6, 6, [3, 5, 6, 8, 9]),
    PF7: (pf7, 7, [3, 5, 6, 8, 9]),
    PF8: (pf8, 8, [5, 6, 8, 9, 10]),
    PF9: (pf9, 9, [5, 6, 8, 9, 10]),
    PF10: (pf10, 10, [13, 14]),
    PF11: (pf11, 11, [5, 10, 12, 13]),
    PF12: (pf12, 12, [12]),
    PF13: (pf13, 13, [4, 12]),
    PF14: (pf14, 14, [4, 12]),
    PF15: (pf15, 15, [4, 12]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [12]),
    PG1: (pg1, 1, [12]),
    PG2: (pg2, 2, [12]),
    PG3: (pg3, 3, [12]),
    PG4: (pg4, 4, [12]),
    PG5: (pg5, 5, [12]),
    PG6: (pg6, 6, [13, 14]),
    PG7: (pg7, 7, [8, 12, 13, 14]),
    PG8: (pg8, 8, [5, 7, 8, 11, 12]),
    PG9: (pg9, 9, [7, 8, 9, 10, 12, 13]),
    PG10: (pg10, 10, [9, 10, 12, 13, 14]),
    PG11: (pg11, 11, [7, 11, 13, 14]),
    PG12: (pg12, 12, [3, 5, 7, 8, 9, 12, 14]),
    PG13: (pg13, 13, [0, 3, 5, 8, 11, 12, 14]),
    PG14: (pg14, 14, [0, 3, 5, 8, 9, 11, 12, 14]),
    PG15: (pg15, 15, [8, 12, 13]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
    PH2: (ph2, 2, [3, 9, 10, 11, 12, 14]),
    PH3: (ph3, 3, [9, 10, 11, 12, 14]),
    PH4: (ph4, 4, [4, 10]),
    PH5: (ph5, 5, [4, 5, 12]),
    PH6: (ph6, 6, [4, 5, 9, 11, 12, 13]),
    PH7: (ph7, 7, [4, 5, 11, 12, 13]),
    PH8: (ph8, 8, [4, 12, 13, 14]),
    PH9: (ph9, 9, [4, 9, 12, 13, 14]),
    PH10: (ph10, 10, [2, 4, 12, 13, 14]),
    PH11: (ph11, 11, [2, 4, 12, 13, 14]),
    PH12: (ph12, 12, [2, 4, 12, 13, 14]),
    PH13: (ph13, 13, [3, 9, 12, 14]),
    PH14: (ph14, 14, [3, 12, 13, 14]),
    PH15: (ph15, 15, [3, 12, 13, 14]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI0: (pi0, 0, [2, 5, 12, 13, 14]),
    PI1: (pi1, 1, [3, 5, 12, 13, 14]),
    PI2: (pi2, 2, [3, 5, 12, 13, 14]),
    PI3: (pi3, 3, [3, 5, 12, 13]),
    PI4: (pi4, 4, [3, 10, 12, 13, 14]),
    PI5: (pi5, 5, [3, 10, 12, 13, 14]),
    PI6: (pi6, 6, [3, 10, 12, 13, 14]),
    PI7: (pi7, 7, [3, 10, 12, 13, 14]),
    PI8: (pi8, 8, []),
    PI9: (pi9, 9, [9, 12, 14]),
    PI10: (pi10, 10, [11, 12, 14]),
    PI11: (pi11, 11, [10]),
    PI12: (pi12, 12, [14]),
    PI13: (pi13, 13, [14]),
    PI14: (pi14, 14, [14]),
    PI15: (pi15, 15, [14]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOJ, gpioj, PJ, 'J', PJn, [
    PJ0: (pj0, 0, [14]),
    PJ1: (pj1, 1, [14]),
    PJ2: (pj2, 2, [14]),
    PJ3: (pj3, 3, [14]),
    PJ4: (pj4, 4, [14]),
    PJ5: (pj5, 5, [14]),
    PJ6: (pj6, 6, [14]),
    PJ7: (pj7, 7, [14]),
    PJ8: (pj8, 8, [14]),
    PJ9: (pj9, 9, [14]),
    PJ10: (pj10, 10, [14]),
    PJ11: (pj11, 11, [14]),
    PJ12: (pj12, 12, [14]),
    PJ13: (pj13, 13, [14]),
    PJ14: (pj14, 14, [14]),
    PJ15: (pj15, 15, [14]),
]);

#[cfg(feature = "gpio-f746")]
gpio!(GPIOK, gpiok, PK, 'K', PKn, [
    PK0: (pk0, 0, [14]),
    PK1: (pk1, 1, [14]),
    PK2: (pk2, 2, [14]),
    PK3: (pk3, 3, [14]),
    PK4: (pk4, 4, [14]),
    PK5: (pk5, 5, [14]),
    PK6: (pk6, 6, [14]),
    PK7: (pk7, 7, [14]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 10, 11]),
    PA1: (pa1, 1, [1, 2, 7, 8, 9, 10, 11, 14]),
    PA2: (pa2, 2, [1, 2, 3, 7, 8, 11, 12, 14]),
    PA3: (pa3, 3, [1, 2, 3, 7, 9, 10, 11, 14]),
    PA4: (pa4, 4, [5, 6, 7, 8, 12, 13, 14]),
    PA5: (pa5, 5, [1, 3, 5, 8, 10, 14]),
    PA6: (pa6, 6, [1, 2, 3, 5, 8, 9, 12, 13, 14]),
    PA7: (pa7, 7, [1, 2, 3, 5, 8, 9, 11, 12]),
    PA8: (pa8, 8, [0, 1, 3, 4, 7, 10, 11, 12, 13, 14]),
    PA9: (pa9, 9, [1, 4, 5, 7, 13, 14]),
    PA10: (pa10, 10, [1, 7, 9, 10, 12, 13, 14]),
    PA11: (pa11, 11, [1, 5, 6, 7, 9, 10, 14]),
    PA12: (pa12, 12, [1, 5, 6, 7, 8, 9, 10, 14]),
    PA13: (pa13, 13, [0], super::Debugger),
    PA14: (pa14, 14, [0], super::Debugger),
    PA15: (pa15, 15, [0, 1, 4, 5, 6, 7, 8, 11, 12], super::Debugger),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 6, 8, 9, 10, 11, 14]),
    PB1: (pb1, 1, [1, 2, 3, 6, 9, 10, 11, 14]),
    PB2: (pb2, 2, [6, 7, 9, 10]),
    PB3: (pb3, 3, [0, 1, 5, 6, 8, 10, 11, 12], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7, 8, 10, 11, 12], super::Debugger),
    PB5: (pb5, 5, [1, 2, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14]),
    PB6: (pb6, 6, [1, 2, 3, 4, 6, 7, 9, 10, 11, 12, 13]),
    PB7: (pb7, 7, [2, 4, 6, 7, 11, 12, 13]),
    PB8: (pb8, 8, [1, 2, 3, 4, 6, 7, 9, 10, 11, 12, 13, 14]),
    PB9: (pb9, 9, [1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14]),
    PB10: (pb10, 10, [1, 4, 5, 6, 7, 9, 10, 11, 14]),
    PB11: (pb11, 11, [1, 4, 6, 7, 10, 11, 13, 14]),
    PB12: (pb12, 12, [1, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
    PB13: (pb13, 13, [1, 5, 6, 7, 8, 9, 10, 11]),
    PB14: (pb14, 14, [1, 3, 4, 5, 6, 7, 8, 9, 10, 12]),
    PB15: (pb15, 15, [0, 1, 3, 4, 5, 6, 8, 9, 10, 12]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [3, 6, 8, 10, 12, 14]),
    PC1: (pc1, 1, [0, 3, 5, 6, 10, 11, 12]),
    PC2: (pc2, 2, [3, 5, 6, 10, 11, 12]),
    PC3: (pc3, 3, [3, 5, 10, 11, 12]),
    PC4: (pc4, 4, [3, 5, 8, 11, 12]),
    PC5: (pc5, 5, [3, 8, 11, 12]),
    PC6: (pc6, 6, [2, 3, 5, 7, 8, 9, 10, 12, 13, 14]),
    PC7: (pc7, 7, [2, 3, 6, 7, 8, 9, 10, 12, 13, 14]),
    PC8: (pc8, 8, [0, 2, 3, 7, 8, 9, 12, 13]),
    PC9: (pc9, 9, [0, 2, 3, 4, 5, 7, 9, 10, 12, 13, 14]),
    PC10: (pc10, 10, [3, 6, 7, 8, 9, 12, 13, 14]),
    PC11: (pc11, 11, [3, 6, 7, 8, 9, 12, 13]),
    PC12: (pc12, 12, [0, 6, 7, 8, 12, 13]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [3, 6, 8, 9, 12]),
    PD1: (pd1, 1, [3, 6, 8, 9, 12]),
    PD2: (pd2, 2, [0, 2, 8, 12, 13]),
    PD3: (pd3, 3, [3, 5, 6, 7, 12, 13, 14]),
    PD4: (pd4, 4, [6, 7, 12]),
    PD5: (pd5, 5, [7, 12]),
    PD6: (pd6, 6, [3, 5, 6, 7, 10, 11, 12, 13, 14]),
    PD7: (pd7, 7, [3, 5, 6, 7, 8, 11, 12]),
    PD8: (pd8, 8, [3, 7, 8, 12]),
    PD9: (pd9, 9, [3, 7, 12]),
    PD10: (pd10, 10, [3, 7, 12, 14]),
    PD11: (pd11, 11, [4, 7, 9, 10, 12]),
    PD12: (pd12, 12, [2, 3, 4, 7, 9, 10, 12]),
    PD13: (pd13, 13, [2, 3, 4, 9, 10, 12]),
    PD14: (pd14, 14, [2, 8, 12]),
    PD15: (pd15, 15, [2, 8, 12]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 3, 8, 10, 12, 13]),
    PE1: (pe1, 1, [3, 8, 12, 13]),
    PE2: (pe2, 2, [0, 5, 6, 9, 11, 12]),
    PE3: (pe3, 3, [0, 6, 12]),
    PE4: (pe4, 4, [0, 5, 6, 10, 12, 13, 14]),
    PE5: (pe5, 5, [0, 3, 5, 6, 10, 12, 13, 14]),
    PE6: (pe6, 6, [0, 1, 3, 5, 6, 10, 12, 13, 14]),
    PE7: (pe7, 7, [1, 6, 8, 10, 12]),
    PE8: (pe8, 8, [1, 6, 8, 10, 12]),
    PE9: (pe9, 9, [1, 6, 8, 10, 12]),
    PE10: (pe10, 10, [1, 6, 8, 10, 12]),
    PE11: (pe11, 11, [1, 5, 6, 10, 12, 14]),
    PE12: (pe12, 12, [1, 5, 6, 10, 12, 14]),
    PE13: (pe13, 13, [1, 5, 6, 10, 12, 14]),
    PE14: (pe14, 14, [1, 5, 10, 12, 14]),
    PE15: (pe15, 15, [1, 12, 14]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12]),
    PF1: (pf1, 1, [4, 12]),
    PF2: (pf2, 2, [4, 12]),
    PF3: (pf3, 3, [12]),
    PF4: (pf4, 4, [12]),
    PF5: (pf5, 5, [12]),
    PF6: (pf6, 6, [3, 5, 6, 8, 9]),
    PF7: (pf7, 7, [3, 5, 6, 8, 9]),
    PF8: (pf8, 8, [5, 6, 8, 9, 10]),
    PF9: (pf9, 9, [5, 6, 8, 9, 10]),
    PF10: (pf10, 10, [9, 13, 14]),
    PF11: (pf11, 11, [5, 10, 12, 13]),
    PF12: (pf12, 12, [12]),
    PF13: (pf13, 13, [4, 6, 12]),
    PF14: (pf14, 14, [4, 6, 12]),
    PF15: (pf15, 15, [4, 12]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [12]),
    PG1: (pg1, 1, [12]),
    PG2: (pg2, 2, [12]),
    PG3: (pg3, 3, [12]),
    PG4: (pg4, 4, [12]),
    PG5: (pg5, 5, [12]),
    PG6: (pg6, 6, [12, 13, 14]),
    PG7: (pg7, 7, [6, 8, 12, 13, 14]),
    PG8: (pg8, 8, [5, 7, 8, 11, 12, 14]),
    PG9: (pg9, 9, [5, 7, 8, 9, 10, 11, 12, 13]),
    PG10: (pg10, 10, [5, 9, 10, 11, 12, 13, 14]),
    PG11: (pg11, 11, [5, 7, 10, 11, 13, 14]),
    PG12: (pg12, 12, [3, 5, 7, 8, 9, 11, 12, 14]),
    PG13: (pg13, 13, [0, 3, 5, 8, 11, 12, 14]),
    PG14: (pg14, 14, [0, 3, 5, 8, 9, 11, 12, 14]),
    PG15: (pg15, 15, [8, 12, 13]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
    PH2: (ph2, 2, [3, 9, 10, 11, 12, 14]),
    PH3: (ph3, 3, [9, 10, 11, 12, 14]),
    PH4: (ph4, 4, [4, 9, 10, 14]),
    PH5: (ph5, 5, [4, 5, 12]),
    PH6: (ph6, 6, [4, 5, 9, 11, 12, 13]),
    PH7: (ph7, 7, [4, 5, 11, 12, 13]),
    PH8: (ph8, 8, [4, 12, 13, 14]),
    PH9: (ph9, 9, [4, 9, 12, 13, 14]),
    PH10: (ph10, 10, [2, 4, 12, 13, 14]),
    PH11: (ph11, 11, [2, 4, 12, 13, 14]),
    PH12: (ph12, 12, [2, 4, 12, 13, 14]),
    PH13: (ph13, 13, [3, 8, 9, 12, 14]),
    PH14: (ph14, 14, [3, 8, 9, 12, 13, 14]),
    PH15: (ph15, 15, [3, 12, 13, 14]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI0: (pi0, 0, [2, 5, 12, 13, 14]),
    PI1: (pi1, 1, [3, 5, 12, 13, 14]),
    PI2: (pi2, 2, [3, 5, 12, 13, 14]),
    PI3: (pi3, 3, [3, 5, 12, 13]),
    PI4: (pi4, 4, [3, 10, 12, 13, 14]),
    PI5: (pi5, 5, [3, 10, 12, 13, 14]),
    PI6: (pi6, 6, [3, 10, 12, 13, 14]),
    PI7: (pi7, 7, [3, 10, 12, 13, 14]),
    PI8: (pi8, 8, []),
    PI9: (pi9, 9, [8, 9, 12, 14]),
    PI10: (pi10, 10, [11, 12, 14]),
    PI11: (pi11, 11, [9, 10]),
    PI12: (pi12, 12, [14]),
    PI13: (pi13, 13, [14]),
    PI14: (pi14, 14, [14]),
    PI15: (pi15, 15, [9, 14]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOJ, gpioj, PJ, 'J', PJn, [
    PJ0: (pj0, 0, [9, 14]),
    PJ1: (pj1, 1, [14]),
    PJ2: (pj2, 2, [13, 14]),
    PJ3: (pj3, 3, [14]),
    PJ4: (pj4, 4, [14]),
    PJ5: (pj5, 5, [14]),
    PJ6: (pj6, 6, [14]),
    PJ7: (pj7, 7, [14]),
    PJ8: (pj8, 8, [14]),
    PJ9: (pj9, 9, [14]),
    PJ10: (pj10, 10, [14]),
    PJ11: (pj11, 11, [14]),
    PJ12: (pj12, 12, [9, 14]),
    PJ13: (pj13, 13, [9, 14]),
    PJ14: (pj14, 14, [14]),
    PJ15: (pj15, 15, [14]),
]);

#[cfg(feature = "gpio-f76x")]
gpio!(GPIOK, gpiok, PK, 'K', PKn, [
    PK0: (pk0, 0, [14]),
    PK1: (pk1, 1, [14]),
    PK2: (pk2, 2, [14]),
    PK3: (pk3, 3, [14]),
    PK4: (pk4, 4, [14]),
    PK5: (pk5, 5, [14]),
    PK6: (pk6, 6, [14]),
    PK7: (pk7, 7, [14]),
]);