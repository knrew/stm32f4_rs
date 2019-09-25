#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::gpio;

pub const LD2_PIN: u16 = gpio::GPIO_PIN_5;
pub const B1_PIN: u16 = gpio::GPIO_PIN_13;

pub fn GpioLed2() -> gpio::Gpio<'static> {
    gpio::Gpio { gpio: gpio::GPIOA(), pin: LD2_PIN }
}

pub fn GpioButton1() -> gpio::Gpio<'static> {
    gpio::Gpio { gpio: gpio::GPIOC(), pin: B1_PIN }
}