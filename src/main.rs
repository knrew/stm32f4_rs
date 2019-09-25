#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

mod hal_time;
mod gpio;
mod led;
mod button;
mod uart;
mod nucleo_f411re;

use hal_time as time;


#[no_mangle]
pub extern fn main_rs() {
    let mut led = led::Led::new(nucleo_f411re::GpioLed2());
    let button = button::Button::new(nucleo_f411re::GpioButton1());

    let mut start = time::get_tick();
    loop {
        if button.is_pushed() {
            uart::print("yes.\r\n");
        } else {
            uart::print("no.\r\n");
        }

        if time::get_tick() - start > 500 {
            led.toggle();
            start = time::get_tick();
        }

        time::delay(500);
    }
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[lang = "eh_personality"]
extern fn eh_personality() {}