#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[repr(C)]
pub struct GPIO_TypeDef {
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

#[allow(dead_code)]
#[repr(C)]
pub enum GPIO_PinState {
    GPIO_PIN_RESET = 0,
    GPIO_PIN_SET = 1,
}

#[allow(dead_code)]
extern "C" {
    pub fn HAL_GPIO_ReadPin(GPIOx: &GPIO_TypeDef, GPIO_Pin: u16) -> GPIO_PinState;
    pub fn HAL_GPIO_WritePin(GPIOx: &mut GPIO_TypeDef, GPIO_Pin: u16, PinState: GPIO_PinState);
    pub fn HAL_GPIO_TogglePin(GPIOx: &mut GPIO_TypeDef, GPIO_Pin: u16);
}

#[allow(dead_code)]
pub struct Gpio<'a> {
    pub gpio: &'a mut GPIO_TypeDef,
    pub pin: u16,
}

pub const GPIO_PIN_5: u16 = 0x0020u16;
pub const GPIO_PIN_13: u16 = 0x2000u16;

pub const PERIPH_BASE: u32 = 0x40000000u32;
pub const AHB1PERIPH_BASE: u32 = (PERIPH_BASE + 0x00020000u32);

pub const GPIOA_BASE: u32 = (AHB1PERIPH_BASE + 0x0000u32);
pub const GPIOC_BASE: u32 = (AHB1PERIPH_BASE + 0x0800u32);

#[allow(dead_code)]
pub fn GPIOA() -> &'static mut GPIO_TypeDef {
    unsafe { &mut *(GPIOA_BASE as *mut GPIO_TypeDef) }
}

#[allow(dead_code)]
pub fn GPIOC() -> &'static mut GPIO_TypeDef {
    unsafe { &mut *(GPIOC_BASE as *mut GPIO_TypeDef) }
}