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

#[allow(dead_code)]
extern "C" {
    fn uart_send(s: *const u8, length: usize)->u8;
}

#[allow(dead_code)]
pub fn print(s: &str) {
    unsafe {
        uart_send(s.as_ptr(), s.len());
    }
}