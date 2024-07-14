pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Byte representation of a VGA color, where the 4 first bits represent a background color
/// and the rest 4 bits signifies foreground (character color)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// 2-byte representation of a VGA character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii: u8,
    color: ColorCode,
}

/// A matrix of size 80x25 that wraps the VGA buffer.
/// Each element is a 2-byte representation of one screen character (1 byte for data, 1 byte for color)
#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Keep track of the current state of the VGA buffer.
/// Provides automatic screen scrolling and formatting
pub struct Writer {
    current_col: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

mod buffer;
pub mod writer;