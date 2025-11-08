#![no_std]

use crate::font::*;

use core::{ mem, slice };

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub rsvd: u8,
}

pub struct Color {
    pub background: Pixel,
    pub foreground: Pixel,
}

pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

// color: (background, foreground)
pub struct FrameBuffer {
    pub buffer: &'static mut [Pixel],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub color: Color,
    pub cursor: Cursor,
}

impl FrameBuffer {
    pub fn new(buffer_ptr: *mut Pixel, width: usize, height: usize, stride: usize) -> Self {
        let buffer_len = height * stride / mem::size_of::<Pixel>();
        let buffer = unsafe { slice::from_raw_parts_mut(buffer_ptr, buffer_len)};

        return FrameBuffer {
            buffer,
            width,
            height,
            stride,
            color: {Color { background:{ Pixel {r: 0, g: 0, b: 0, rsvd: 0}}, foreground:{ Pixel {r: 255, g: 255, b: 255, rsvd: 0} } }},
            cursor:{ Cursor { x: 0, y: 0 } },
        };
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Pixel) {
        if x < self.width && y < self.height {
            let index = y * self.stride / mem::size_of::<Pixel>() + x;
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }

    pub fn write_bitmap(&mut self, bitmap: &[u8; BITMAP_HEIGHT], color: Option<(Pixel, Pixel)>, cursor: Option<(usize, usize)>) -> () {
        let (background, foreground) = match color {
            Some((back_color, fore_color)) => (back_color, fore_color),
            None => (self.color.background, self.color.foreground),
        };

        let (cursor_x, cursor_y) = match cursor {
            Some((x, y)) => (x, y),
            None => (self.cursor.x, self.cursor.y),
        };

        for y in 0..BITMAP_HEIGHT {
            for x in (0..BITMAP_WIDTH).rev() {
                if ((bitmap[y] >> x) & 1) == 1 {
                    self.set_pixel(x + cursor_x, y + cursor_y, foreground);
                } else {
                    self.set_pixel(x + cursor_x, y + cursor_y, background);
                }
            }
        }

        self.cursor.x += BITMAP_WIDTH;
        // Add logic to handle rows
        // self.cursor.y += BITMAP_LENGTH;
    }

    pub fn clear(&mut self, color: Pixel) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }
}