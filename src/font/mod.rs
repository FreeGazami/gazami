// Glyphs

pub const FIXED_WIDTH: usize = 8;
pub const FIXED_HEIGHT: usize = 16;
pub const FIXED_LENGTH: usize = 128;

use crate::framebuffer::Pixel;

pub const BITMAP_CHAR_A: [u8; 16] = [
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

// #[repr(C)]
// pub struct Character<'a> {
//     pub bitmap: &'a [u8; FIXED_HEIGHT],
//     pub foreground_color: Pixel,
//     pub background_color: Pixel,
// }
// 
// impl Character<'_> {
//     pub fn new(char_bitmap: &[u8;16], foreground: Pixel, background: Pixel) -> Self {
//         return Character {
//             bitmap: char_bitmap,
//             foreground_color: foreground,
//             background_color: background
//         };
//     }
// }

