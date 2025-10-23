#![no_std]
#![no_main]

// mod FrameBuffer;
use core::arch::asm;
use core::ffi::c_void;

use uefi_raw::table::system::SystemTable;
use uefi_raw::table::runtime::{RuntimeServices, ResetType};
use uefi_raw::protocol::console::{SimpleTextOutputProtocol};
use uefi_raw::Handle;
use uefi_raw::Status;

use uefi_handoff::BootInfo;

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

    unsafe {
        ((*runtime_services).reset_system)(ResetType::COLD, Status::SUCCESS, 0, core::ptr::null())
    }

    hlt_loop()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {

    }
}

fn hlt_loop() -> ! {
    loop {

    }
}