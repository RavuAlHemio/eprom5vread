#![no_main]
#![no_std]


mod setup;
mod uart;


use core::panic::PanicInfo;

use cortex_m_rt::entry;
use stm32g0b0::Peripherals;


#[panic_handler]
fn oh_no(_info: &PanicInfo) -> ! {
    loop {
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take()
        .expect("peripherals taken?!");

    crate::setup::setup_clocks(&mut peripherals);
    crate::uart::setup(&mut peripherals);
    crate::uart::write(&mut peripherals, b"Well hello there!\r\n");

    loop {
    }
}
