#![no_std]
#![no_main]

mod framebuffer;
mod font;

use core::arch::asm;
use core::ffi::c_void;

use uefi_raw::table::system::SystemTable;
use uefi_raw::table::runtime::{RuntimeServices, ResetType};
use uefi_raw::protocol::console::{SimpleTextOutputProtocol};
use uefi_raw::Handle;
use uefi_raw::Status;

use uefi_handoff::BootInfo;

use framebuffer::FrameBuffer;
use framebuffer::Pixel;
use font::*;

use uefi_raw::protocol::console::GraphicsOutputModeInformation;

const SERIAL_PORT: u16 = 0x3F8;

/// Writes a single byte to the serial port
pub fn serial_write_byte(byte: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") SERIAL_PORT, in("al") byte);
    }
}

/// Writes a string slice to the serial port
pub fn serial_write_string(s: &str) {
    for b in s.bytes() {
        serial_write_byte(b);
    }
}

pub fn print_u32(mut num: u32) {
    let mut buf = [0u8; 10];  // Maximum length of u32 decimal digits is 10
    let mut i = 10;

    if num == 0 {
        serial_write_string("0");
        return;
    }

    while num > 0 {
        i -= 1;
        buf[i] = b'0' + (num % 10) as u8;
        num /= 10;
    }

    // Convert the relevant slice to &str safely
    let s = core::str::from_utf8(&buf[i..]).unwrap_or("ERR");
    serial_write_string(s);
}

pub fn print_u64(mut num: u64) {
    let mut buf = [0u8; 20];  // Maximum length of u64 decimal digits is 20
    let mut i = 20;

    if num == 0 {
        serial_write_string("0");
        return;
    }

    while num > 0 {
        i -= 1;
        buf[i] = b'0' + (num % 10) as u8;
        num /= 10;
    }

    // Convert the relevant slice to &str safely
    let s = core::str::from_utf8(&buf[i..]).unwrap_or("ERR");
    serial_write_string(s);
}


#[unsafe(no_mangle)]
pub extern "C" fn _start(bootinfo_addr: *mut c_void) -> ! {
    let bootinfo: *mut BootInfo = bootinfo_addr as *mut BootInfo;

    let runtime_services: *mut RuntimeServices = unsafe{
        (*bootinfo).runtime_services as *mut RuntimeServices
        // 0xbf5ecb98 as *mut RuntimeServices
    };

    let frame_buffer_base: u64 = unsafe{(*bootinfo).frame_buffer_base};

    // unsafe {
    //     ((*runtime_services).reset_system)(ResetType::COLD, Status::SUCCESS, 0, core::ptr::null())
    // }

    let info: GraphicsOutputModeInformation = unsafe{(*bootinfo).info};

    let mut frame_buffer: FrameBuffer = FrameBuffer::new(
        frame_buffer_base as *mut Pixel,
        info.horizontal_resolution as usize,
        info.vertical_resolution as usize,
        (info.pixels_per_scan_line * 4) as usize,
    );

    // try using new code
    frame_buffer.clear(Pixel {r:0, g: 0, b: 0, rsvd: 0});
    frame_buffer.write_bitmap(&BITMAP_CHAR_A, None, None);
    frame_buffer.write_bitmap(&BITMAP_CHAR_A, None, None);
    frame_buffer.write_bitmap(&BITMAP_CHAR_B, None, None);
    frame_buffer.write_bitmap(&BITMAP_CHAR_B, None, None);

    hlt_loop()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_write_string("PANICED!\n");
    loop {

    }
}

fn hlt_loop() -> ! {
    loop {

    }
}