use crate::gpio;

#[allow(dead_code)]
pub struct Button<'a> {
    gpio: &'a mut gpio::GPIO_TypeDef,
    pin: u16,
}

impl Button<'_> {
    #[allow(dead_code)]
    pub fn new(setting: gpio::Gpio) -> Button<'_> {
        Button { gpio: setting.gpio, pin: setting.pin }
    }

    #[allow(dead_code)]
    pub fn is_pushed(&self) -> bool {
        let state = unsafe {
            gpio::HAL_GPIO_ReadPin(self.gpio, self.pin)
        };
        state as u8 == gpio::GPIO_PinState::GPIO_PIN_RESET as u8
    }
}