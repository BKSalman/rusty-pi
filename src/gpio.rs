use core::{arch::asm, panic};

pub struct GPIO;

pub enum GPFuncSel {
    Input = 0b000,
    Output = 0b001,
    AltFn0 = 0b100,
    AltFn1 = 0b101,
    AltFn2 = 0b110,
    AltFn3 = 0b111,
    AltFn4 = 0b011,
    AltFn5 = 0b010,
}

impl GPIO {
    pub fn set_func(pin: u32, func: GPFuncSel) {
        let fsel = pin / 10;
        let fsel_reg = match fsel {
            0 => GPFSEL0,
            1 => GPFSEL1,
            2 => GPFSEL2,
            _ => panic!("Pin doesn't exist"),
        };

        let mut val: u32;

        unsafe {
            val = core::ptr::read_volatile(fsel_reg as *mut u32);
        }

        let mut mask = 0b111;

        let pinnum = pin % 10;

        mask <<= pinnum * 3;

        val &= !(mask);

        val |= (func as u32) << (pinnum * 3);

        unsafe {
            core::ptr::write_volatile(fsel_reg as *mut u32, val);
        }
    }

    pub fn set(pin: u32) {
        let bitpos = pin;

        let mut val: u32;

        unsafe {
            val = core::ptr::read_volatile(GPIO_SET0 as *mut u32);
        }

        val |= 1 << bitpos;

        unsafe {
            core::ptr::write_volatile(GPIO_SET0 as *mut u32, val);
        }
    }

    pub fn clear(pin: u32) {
        let bitpos = pin;

        let mut val: u32;

        unsafe {
            val = core::ptr::read_volatile(GPIO_CLR0 as *mut u32);
        }

        val |= 1 << bitpos;

        unsafe {
            core::ptr::write_volatile(GPIO_CLR0 as *mut u32, val);
        }
    }

    pub fn enable(pin: u32) {
        unsafe {
            // disable pupd
            core::ptr::write_volatile(GPPUD_ENABLE as *mut u32, 0);

            // wait for 150 cycles
            for _ in 0..150 {
                asm!("nop")
            }

            // set clock to pin
            let index = pin / 32;
            core::ptr::write_volatile(GPPUD_CLK[index as usize] as *mut u32, 1 << (pin % 32));

            // wait for 150 cycles
            for _ in 0..150 {
                asm!("nop")
            }

            core::ptr::write_volatile(GPPUD_ENABLE as *mut u32, 0);
            core::ptr::write_volatile(GPPUD_CLK[index as usize] as *mut u32, 0);
        }
    }
}

// pin 20 -> which GPIO FSEL address I should modify, then
// what is the value at which bits we need to write in that FSEL to tell it if it's output or input
const GPFSEL0: u32 = 0x3F20_0000;
const GPFSEL1: u32 = 0x3F20_0004;
const GPFSEL2: u32 = 0x3F20_0008;

const GPIO_SET0: u32 = 0x3f20_001c;
const GPIO_CLR0: u32 = 0x3f20_0028;

// - write to GPPUD_ENABLE either
// 00 = off, 01 = enable pull down control, or 10 enable pull up control
//
// - wait 150 cycles
//
// - write to GPPUD_CLK0/1
//
// - wait 150 cycles
//
// - write to GPPUD_ENABLE to remove the control signal
//
// - write to GPPUD_CLK0/1 to remove the clock
const GPPUD_ENABLE: u32 = 0x3F20_0094;
const GPPUD_CLK: [u32; 2] = [0x3F20_0098, 0x3F20_009c];
