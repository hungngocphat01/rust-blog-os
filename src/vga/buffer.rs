use super::{Buffer, ScreenChar};

use core::ptr::{read_volatile, write_volatile};

impl Buffer {
    pub fn putc(&mut self, row: usize, col: usize, char: ScreenChar) {
        unsafe {
            write_volatile(
                &mut self.chars[row][col] as *mut ScreenChar, char 
            )
        }
    }

    pub fn getc(&self, row: usize, col: usize) -> ScreenChar {
        unsafe {
            read_volatile(&self.chars[row][col] as *const ScreenChar)
        }
    }
}
