#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Run - mode clock configuration
const RCC: u32 = 0x400FE060;

//prescaler value
const SYSCTL_SYSDIV_12: u32 = 0xB;
const SYSCTL_SYSDIV_16: u32 = 0xF;

// CPU frequency (12.5 MHz by default)
const CPU_FREQ: u32 = 12_500_000;

#[entry]
fn main() -> ! {
    hprintln!("Starting program!");

    //set the prescaler value
    unsafe {
        let sysdiv: u32 = SYSCTL_SYSDIV_16 << 23;
        let orig: u32 = *(RCC as *const u32);
        let mask: u32 = !0b1111 << 23;
        let rcc: u32 = (orig & mask) | sysdiv;
        *(RCC as *mut u32) = rcc;
    }

    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.enable_interrupt();
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(CPU_FREQ);
    systick.clear_current();
    systick.enable_counter();

    loop {
        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    hprintln!("ugh, woke up :(")
}
