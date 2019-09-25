#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

const GPIO_PIN_5: u16 = 0x0020u16;
const GPIO_PIN_13: u16 = 0x2000u16;

const PERIPH_BASE: u32 = 0x40000000u32;
const AHB1PERIPH_BASE: u32 = (PERIPH_BASE + 0x00020000u32);

const GPIOA_BASE: u32 = (AHB1PERIPH_BASE + 0x0000u32);
const GPIOC_BASE: u32 = (AHB1PERIPH_BASE + 0x0800u32);

#[repr(C)]
pub struct GpioTypeDef {
    MODER: u32,
    OTYPER: u32,
    OSPEEDR: u32,
    PUPDR: u32,
    IDR: u32,
    ODR: u32,
    BSRR: u32,
    LCKR: u32,
    AFR: [u32; 2],
}

#[repr(C)]
pub enum GPIOPinState {
    GPIO_PIN_RESET = 0,
    GPIO_PIN_SET = 1,
}

#[allow(dead_code)]
extern "C" {
    pub fn HAL_GPIO_ReadPin(GPIOx: &GpioTypeDef, GPIO_Pin: u16) -> GPIOPinState;
    pub fn HAL_GPIO_WritePin(GPIOx: &mut GpioTypeDef, GPIO_Pin: u16, PinState: GPIOPinState);
    pub fn HAL_GPIO_TogglePin(GPIOx: &mut GpioTypeDef, GPIO_Pin: u16);
}

#[allow(dead_code)]
pub fn GPIOA() -> &'static mut GpioTypeDef {
    unsafe { &mut *(GPIOA_BASE as *mut GpioTypeDef) }
}

#[allow(dead_code)]
pub fn GPIOC() -> &'static mut GpioTypeDef {
    unsafe { &mut *(GPIOC_BASE as *mut GpioTypeDef) }
}


#[allow(dead_code)]
pub struct Gpio<'a> {
    pub gpio: &'a mut GpioTypeDef,
    pub pin: u16,
}

pub const LD2_PIN: u16 = GPIO_PIN_5;
pub const B1_PIN: u16 = GPIO_PIN_13;

pub fn GpioLed2() -> Gpio<'static> {
    Gpio { gpio: GPIOA(), pin: LD2_PIN }
}

pub fn GpioButton1() -> Gpio<'static> {
    Gpio { gpio: GPIOC(), pin: B1_PIN }
}