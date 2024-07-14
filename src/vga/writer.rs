use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use super::*;

impl Writer {
    pub fn new(default_color: ColorCode, buffer: &'static mut Buffer) -> Self {
        Writer {
            current_col: 0,
            color: default_color,
            buffer,
        }
    }

    /// Write an ASCII byte to the screen
    pub fn write_byte(&mut self, char: u8) {
        match char {
            b'\n' => self.new_line(),
            byte => {
                let row = BUFFER_HEIGHT - 1;
                let char = ScreenChar {
                    ascii: byte,
                    color: self.color,
                };

                self.buffer.putc(row, self.current_col, char);
                self.advance_carriage();
            }
        }
    }

    /// Write an ASCII string to the screen. Any non-printable ASCII character except \n would
    /// be displayed as 0xFE
    pub fn write_string(&mut self, string: &str) {
        for char in string.bytes() {
            match char {
                0x20..0x7e | b'\n' => self.write_byte(char),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn return_carriage(&mut self) {
        self.current_col = 0;
    }

    fn advance_carriage(&mut self) {
        self.current_col += 1;

        if self.current_col >= BUFFER_WIDTH {
            self.new_line();
        }
    }

    fn new_line(&mut self) {
        for row in 0..BUFFER_HEIGHT - 1 {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.getc(row + 1, col);
                self.buffer.putc(row, col, char);
            }
        }

        self.clear_last_row();
    }

    fn clear_last_row(&mut self) {
        let blank = ScreenChar {
            ascii: b' ',
            color: self.color
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.putc(BUFFER_HEIGHT - 1, col, blank);  
        }

        self.return_carriage();
    }
}

lazy_static! {
    /// Global vga::Writer instance shared across the kernel
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        ColorCode::new(Color::Green, Color::Black),
        unsafe { &mut *(0xb8000 as *mut Buffer) }
    ));
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
