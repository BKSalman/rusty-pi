pub mod common;
pub mod driver_manager;
pub mod gpio;
pub mod uart;

use self::driver_manager::DeviceDriverDescriptor;

use super::memory::map::mmio;
use crate::console;
use core::sync::atomic::{AtomicBool, Ordering};

static PL011_UART: uart::PL011Uart = unsafe { uart::PL011Uart::new(mmio::PL011_UART_START) };
static GPIO: gpio::GPIO = unsafe { gpio::GPIO::new(mmio::GPIO_START) };

/// This must be called only after successful init of the UART driver.
fn post_init_uart() -> Result<(), &'static str> {
    console::register_console(&PL011_UART);

    Ok(())
}

/// This must be called only after successful init of the GPIO driver.
fn post_init_gpio() -> Result<(), &'static str> {
    GPIO.map_pl011_uart();
    Ok(())
}

fn driver_uart() -> Result<(), &'static str> {
    let uart_descriptor = DeviceDriverDescriptor::new(&PL011_UART, Some(post_init_uart));
    driver_manager::driver_manager().register_driver(uart_descriptor);

    Ok(())
}

fn driver_gpio() -> Result<(), &'static str> {
    let gpio_descriptor = DeviceDriverDescriptor::new(&GPIO, Some(post_init_gpio));
    driver_manager::driver_manager().register_driver(gpio_descriptor);

    Ok(())
}

/// Initialize the driver subsystem.
///
/// # Safety
///
/// See child function calls.
pub unsafe fn init() -> Result<(), &'static str> {
    static INIT_DONE: AtomicBool = AtomicBool::new(false);
    if INIT_DONE.load(Ordering::Relaxed) {
        return Err("Init already done");
    }

    driver_uart()?;
    driver_gpio()?;

    INIT_DONE.store(true, Ordering::Relaxed);
    Ok(())
}
