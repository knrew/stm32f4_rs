#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[repr(C)]
#[allow(dead_code)]
pub enum HAL_StatusTypeDef {
    HAL_OK = 0x00isize,
    HAL_ERROR = 0x01isize,
    HAL_BUSY = 0x02isize,
    HAL_TIMEOUT = 0x03isize,
}

extern "C" {
     fn uart_send(s: &str);
}

pub fn print(s: &str) {
    unsafe {
        uart_send(s);
    }
}