#![no_std]
#![no_main]

// mod FrameBuffer;
use core::arch::asm;

use uefi_raw::table::system::SystemTable;
use uefi_raw::protocol::console::{SimpleTextOutputProtocol};
use uefi_raw::Handle;

use uefi_handoff::BootInfo;


#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: *mut u8) -> ! {
    let handoff: *mut BootInfo = boot_info as *mut BootInfo;

    let system_table: *mut SystemTable = unsafe { (*handoff).system_table as *mut SystemTable };

    // the comment out of shame
    // let stdout: *mut SimpleTextOutputProtocol = unsafe {(*system_table).stdout};
    // let u16bytes: [u16; 6] = ['H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16, 0u16 ];

    // unsafe {
    //     let _ = ((*stdout).output_string)(stdout, u16bytes.as_ptr());
    // }

    hlt_loop();
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