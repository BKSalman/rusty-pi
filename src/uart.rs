use crate::gpio::{GPFuncSel, GPIO};

const TXD: u32 = 14;
const RXD: u32 = 15;

const AUX_IRQ: u32 = 0x3F21_5000;
const AUX_ENABLE: u32 = 0x3F21_5004;

const MU_IO: u32 = 0x3F21_5040;
const MU_IER: u32 = 0x3F21_5044;
const MU_IIR: u32 = 0x3F21_5048;
const MU_LCR: u32 = 0x3F21_504C;
const MU_MCR: u32 = 0x3F21_5050;
const MU_LSR: u32 = 0x3F21_5054;
const MU_MSR: u32 = 0x3F21_5058;
const MU_SCRATCH: u32 = 0x3F21_505C;
const MU_CONTROL: u32 = 0x3F21_5060;
const MU_STAT: u32 = 0x3F21_5064;
const MU_BAUD_RATE: u32 = 0x3F21_5068;

pub struct Uart;

impl Uart {
    pub fn init() {
        GPIO::set_func(TXD, GPFuncSel::AltFn5);
        GPIO::set_func(RXD, GPFuncSel::AltFn5);

        GPIO::enable(TXD);
        GPIO::enable(RXD);

        unsafe {
            // AUX_ENABLE = 0 -> mini UART enable
            core::ptr::write_volatile(AUX_ENABLE as *mut u32, 1);

            core::ptr::write_volatile(MU_CONTROL as *mut u32, 0);

            core::ptr::write_volatile(MU_IER as *mut u32, 0);

            core::ptr::write_volatile(MU_LCR as *mut u32, 3);

            core::ptr::write_volatile(MU_MCR as *mut u32, 0);

            core::ptr::write_volatile(MU_BAUD_RATE as *mut u32, 270); // = 115200 @ 250 Mhz

            core::ptr::write_volatile(MU_CONTROL as *mut u32, 3);
        }

        Uart::send('\r');
        Uart::send('\n');
        Uart::send('\n');
    }

    pub fn send(c: char) {
        unsafe {
            loop {
                let val = core::ptr::read_volatile(MU_LSR as *mut u32);
                if val & (1 << 5) != 0 {
                    break;
                }
            }

            core::ptr::write_volatile(MU_IO as *mut u32, c as u32);
        }
    }

    pub fn send_string(text: &str) {
        if text == "\n" {
            Uart::send('\r');
        }

        for c in text.chars() {
            Uart::send(c);
        }
    }

    pub fn recv() -> char {
        unsafe {
            loop {
                let val = core::ptr::read_volatile(MU_LSR as *mut u32);
                if val & 1 != 0 {
                    break;
                }
            }

            let c = core::ptr::read_volatile(MU_IO as *mut u32);
            char::from_u32(c & 0xFF).unwrap()
        }
    }
}
