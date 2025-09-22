#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// System tick - control and status register
const SYST_CSR: u32 = 0xE000E010;

// System tick - reload value register
const SYST_RVR: u32 = 0xE000E014;

// System tick - current value register
const SYST_CVR: u32 = 0xE000E018;

// CPU frequency (12.5 MHz by default)
const CPU_FREQ: u32 = 12_500_000;

#[entry]
fn main() -> ! {
    hprintln!("Starting program!");

    let sleep_dur = CPU_FREQ; // 1 second

    unsafe {
        *(SYST_RVR as *mut u32) = sleep_dur; // Set reload register

        //clear the current value register
        *(SYST_CVR as *mut u32) = 0;

        //Enable the times
        *(SYST_CSR as *mut u32) = 0b111; // Enable SysTick, its interrupt, and select processor clock
    }

    loop {
        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    hprintln!("ugh, woke up :(")
}
