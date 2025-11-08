// Glyphs

pub const BITMAP_WIDTH: usize = 8;
pub const BITMAP_HEIGHT: usize = 16;
pub const BITMAP_LENGTH: usize = 128;

use crate::framebuffer::Pixel;

pub const BITMAP_CHAR_A: [u8; BITMAP_HEIGHT] = [
    0b00000000,
    0b00111100,
    0b01111110,
    0b01111110,
    0b01100110,
    0b01100110,
    0b01100110,
    0b01100110,
    0b01111110,
    0b01111110,
    0b01100110,
    0b01100110,
    0b01100110,
    0b01100110,
    0b01100110,
    0b00000000,
];

pub const BITMAP_CHAR_B: [u8; BITMAP_HEIGHT] = [
    0b00000000,
    0b00111110,
    0b01111110,
    0b01100110,
    0b01000110,
    0b01000110,
    0b01100110,
    0b00111110,
    0b00111110,
    0b01100110,
    0b01000110,
    0b01000110,
    0b01100110,
    0b01111110,
    0b00111110,
    0b00000000,
];

pub const BITMAP_CHAR_C: [u8; BITMAP_HEIGHT] = [
    0b00000000,
    0b00111100,
    0b01111110,
    0b01100110,
    0b01100110,
    0b01100110,
    0b00000110,
    0b00000110,
    0b00000110,
    0b00000110,
    0b01100110,
    0b01100110,
    0b01100110,
    0b01111110,
    0b00111100,
    0b00000000,
];

#[repr(C)]
pub struct Character<'a> {
    pub bitmap: &'a [u8; BITMAP_HEIGHT],
}

impl Character<'_> {
    pub fn new<'a>(char_bitmap: &'a [u8;16]) -> Character<'a> {
        return Character {
            bitmap: char_bitmap,
        };
    }
}
