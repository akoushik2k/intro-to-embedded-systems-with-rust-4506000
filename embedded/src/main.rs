#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::{asm, peripheral};
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//CPU Frequency (12.5Mhz default)
const CPU_FREQ: u32 = 12_500_000;

#[entry]
fn main() -> ! {
    hprintln!("Starting program!");
    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.enable_interrupt();
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(CPU_FREQ);
    systick.clear_current();
    systick.enable_counter();
    loop {
      asm::wfi()
    }
}

#[exception]
fn SysTick() {
    hprintln!("SysTick");
} 
