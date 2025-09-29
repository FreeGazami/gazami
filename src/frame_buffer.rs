use core::{ mem, slice }

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

struct FrameBuffer {
    buffer: &'static mut [Pixel],
    width: usize,
    height: usize,
    stride: usize,
}

impl FrameBuffer {
    fn new(buffer_ptr: *mut Pixel, width: usize, height: usize, stride: usize) -> Self {
        let buffer_len = height * stride / mem::size_of::<Pixel>();
        let buffer = unsafe { slice::from_raw_parts_mut(buffer_ptr, buffer_len)};

        return FrameBuffer {
            buffer,
            width,
            height,
            stride,
        };
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: Pixel) {
        if x < self.width && y < self.height {
            let index = y * self.strid / mem::size_of::<Pixel>() + x;
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }

    fn clear(&mut self, color: Pixel) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }
}