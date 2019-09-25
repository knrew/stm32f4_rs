use crate::gpio;

#[allow(dead_code)]
pub struct Led<'a> {
    gpio: &'a mut gpio::GPIO_TypeDef,
    pin: u16,
    is_on: bool,
}

impl Led<'_> {
    #[allow(dead_code)]
    pub fn new(setting: gpio::Gpio) -> Led<'_> {
        Led { gpio: setting.gpio, pin: setting.pin, is_on: false }
    }

    #[allow(dead_code)]
    pub fn on(&mut self) {
        unsafe {
            gpio::HAL_GPIO_WritePin(self.gpio, self.pin, gpio::GPIO_PinState::GPIO_PIN_SET)
        }
    }

    #[allow(dead_code)]
    pub fn off(&mut self) {
        unsafe {
            gpio::HAL_GPIO_WritePin(self.gpio, self.pin, gpio::GPIO_PinState::GPIO_PIN_RESET)
        }
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self) {
        unsafe {
            gpio::HAL_GPIO_TogglePin(self.gpio, self.pin);
        }
        self.is_on = !self.is_on;
    }

    #[allow(dead_code)]
    pub fn is_on(&self) -> bool {
        self.is_on
    }
}