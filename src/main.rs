#![no_std]
#![no_main]

use rusty_pi::{
    console,
    drivers::{self, driver_manager},
    println,
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
    println!("[1] Booting on: Raspberry Pi 3");

    println!("[2] Drivers loaded:");
    driver_manager::driver_manager().enumerate();

    println!("[3] Chars written: {}", console::console().chars_written());
    println!("[4] Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console::console().clear_rx();

    loop {
        let c = console::console().read_char();
        console::console().write_char(c);
    }
}
