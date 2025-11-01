#![no_std]

use core::{ mem, slice };

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub reserved: u8,
}

pub struct FrameBuffer {
    pub buffer: &'static mut [Pixel],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
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
        };
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Pixel) {
        if x < self.width && y < self.height {
            let index = y * self.stride / mem::size_of::<Pixel>() + x;
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }

    pub fn clear(&mut self, color: Pixel) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }
}