#![no_std]
#![no_main]

// mod FrameBuffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let mut fb = FrameBuffer::new();
    loop {

    }
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