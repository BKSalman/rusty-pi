#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rusty_pi::uart::Uart;

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    Uart::init();
    Uart::send_string("Rasperry PI Bare Metal OS Initializing...\n");

    Uart::send_string("\n\nDone\n");

    // GPIO::set_func(21, GPFuncSel::Output);
    loop {
        // GPIO::set(21);

        // for _ in 0..50000 {
        //     unsafe { asm!("nop") }
        // }

        // GPIO::clear(21);

        // for _ in 0..50000 {
        //     unsafe { asm!("nop") }
        // }
        Uart::send(Uart::recv());
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
