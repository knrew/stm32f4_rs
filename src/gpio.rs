#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

const GPIO_PIN_5: u16 = 0x0020u16;
pub const LD2_PIN: u16 = GPIO_PIN_5;

const PERIPH_BASE: u32 = 0x40000000u32;
const AHB1PERIPH_BASE: u32 = (PERIPH_BASE + 0x00020000u32);
const GPIOA_BASE: u32 = (AHB1PERIPH_BASE + 0x0000u32);

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
enum GPIOPinState {
    GPIO_PIN_RESET = 0,
    GPIO_PIN_SET = 1,
}

#[allow(dead_code)]
extern "C" {
    pub fn HAL_GPIO_ReadPin(GPIOx: &mut GpioTypeDef, GPIO_Pin: u16) -> GPIOPinState;
    pub fn HAL_GPIO_WritePin(GPIOx: &mut GpioTypeDef, GPIO_Pin: u16, PinState: GPIOPinState);
    pub fn HAL_GPIO_TogglePin(GPIOx: &mut GpioTypeDef, GPIO_Pin: u16);
}

#[allow(dead_code)]
pub fn GPIOA() -> &'static mut GpioTypeDef {
    unsafe { &mut *(GPIOA_BASE as *mut GpioTypeDef) }
}

#[allow(dead_code)]
pub struct Led<'a> {
    gpio: &'a mut GpioTypeDef,
    pin: u16,
}

impl Led<'_> {
    #[allow(dead_code)]
    pub fn new(gpio: &mut GpioTypeDef, pin: u16) -> Led<'_> {
        Led { gpio: gpio, pin: pin }
    }

    #[allow(dead_code)]
    pub fn on(&mut self) {
        unsafe {
            HAL_GPIO_WritePin(self.gpio, self.pin, GPIOPinState::GPIO_PIN_SET)
        }
    }

    #[allow(dead_code)]
    pub fn off(&mut self) {
        unsafe {
            HAL_GPIO_WritePin(self.gpio, self.pin, GPIOPinState::GPIO_PIN_RESET)
        }
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self) {
        unsafe {
            HAL_GPIO_TogglePin(self.gpio, self.pin);
        }
    }

    #[allow(dead_code)]
    pub fn is_on(&mut self) -> bool {
        let state = unsafe {
            HAL_GPIO_ReadPin(self.gpio, self.pin)
        };
        state as u8 == GPIOPinState::GPIO_PIN_RESET as u8
    }
}
