use super::{Buffer, ScreenChar};

impl Buffer {
    pub fn putc(&mut self, row: usize, col: usize, char: ScreenChar) {
        self.chars[row][col].write(char);
    }

    pub fn getc(&self, row: usize, col: usize) -> ScreenChar {
        self.chars[row][col].read()
    }
}