#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern "C" {
    pub fn HAL_Delay(delay: u32);
}

pub fn delay(ms: u32) {
    unsafe { HAL_Delay(ms); }
}