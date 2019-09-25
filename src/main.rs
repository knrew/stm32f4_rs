#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]


mod gpio;
mod hal_time;

use hal_time as time;

#[no_mangle]
pub extern fn main_rs() {
    let mut led = gpio::Led::new(gpio::GPIOA(), gpio::LD2_PIN);

    loop {
        led.on();
        time::delay(100);
        led.off();
        time::delay(100);
    }
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[lang = "eh_personality"]
extern fn eh_personality() {}