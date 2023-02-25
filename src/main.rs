#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rusty_pi::{
    console,
    drivers::{self, driver_manager},
    println,
    uart::Uart,
};

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        if let Err(e) = drivers::init() {
            panic!("Error initializing BSP driver subsystem: {}", e);
        }

        // Initialize all device drivers.
        driver_manager::driver_manager().init_drivers();
        // println! is usable from here on.
    }

    println!(
        "[0] {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("[1] Booting on: {}", bsp::board_name());

    println!("[2] Drivers loaded:");
    driver_manager::driver_manager().enumerate();

    println!("[3] Chars written: {}", console().chars_written());
    println!("[4] Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console::console().clear_rx();

    loop {
        let c = console().read_char();
        console().write_char(c);
    }
}
