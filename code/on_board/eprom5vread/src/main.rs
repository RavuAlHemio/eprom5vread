#![no_main]
#![no_std]


mod i2c;
mod i2c_ioext;
mod setup;
mod uart;


use core::panic::PanicInfo;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use stm32g0b0::Peripherals;

use crate::i2c_ioext::{I2C_BASE_ADDRESS, IoExtHandler};


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

    // turn on the LED
    // enable clock for GPIOA
    peripherals.RCC.iopenr().modify(|_, w| w
        .gpioaen().set_bit()
    );
    // start out with LED off
    peripherals.GPIOA.bsrr().write(|w| w
        .br2().set_bit()
    );
    // set to output
    peripherals.GPIOA.moder().modify(|_, w| w
        .moder2().output()
    );
    // turn LED on
    peripherals.GPIOA.bsrr().write(|w| w
        .bs2().set_bit()
    );

    crate::uart::setup(&mut peripherals);
    crate::uart::write(&mut peripherals, b"Well hello there!\r\n");

    crate::i2c::setup(&mut peripherals);

    // reset the port expanders via PB1
    // enable clock for GPIOB
    peripherals.RCC.iopenr().modify(|_, w| w
        .gpioben().set_bit()
    );
    // start out with reset down
    peripherals.GPIOB.bsrr().write(|w| w
        .br1().set_bit()
    );
    // set to output
    peripherals.GPIOB.moder().modify(|_, w| w
        .moder1().output()
    );
    // spin a bit
    for _ in 0..1024 {
        nop();
    }
    // pull reset back up
    peripherals.GPIOB.bsrr().write(|w| w
        .bs1().set_bit()
    );

    // prepare the pins
    let mut ext0 = IoExtHandler::new(&mut peripherals, I2C_BASE_ADDRESS | 0b000);
    let mut ext1 = IoExtHandler::new(&mut peripherals, I2C_BASE_ADDRESS | 0b001);
    let mut ext2 = IoExtHandler::new(&mut peripherals, I2C_BASE_ADDRESS | 0b010);
    let mut ext3 = IoExtHandler::new(&mut peripherals, I2C_BASE_ADDRESS | 0b011);
    let mut ext4 = IoExtHandler::new(&mut peripherals, I2C_BASE_ADDRESS | 0b100);

    // we're reading an M27C800

    // ext1 pin4: ~{G} is ~{output enable} (keep it high while we faff about)
    // ext1 pin5: ~{E} is ~{chip select} (keep it high while we faff about)
    // ext3 pin3: ~{BYTE}V_{PP} reads words when high (keep it high the whole time)
    ext1.enable_pins(&mut peripherals, 0b0011_0000);
    ext3.enable_pins(&mut peripherals, 0b0000_1000);

    // set pin directions: Q are inputs, everything else is an output
    ext0.set_pin_direction(&mut peripherals, 0b00000000);
    ext1.set_pin_direction(&mut peripherals, 0b00001111);
    ext2.set_pin_direction(&mut peripherals, 0b11111111);
    ext3.set_pin_direction(&mut peripherals, 0b11110000);
    ext4.set_pin_direction(&mut peripherals, 0b00000000);

    // this is where the fun starts
    // address has up to 21 bits => store in u32

    // on 1Mbit 16-bit ROMs, the highest address is 0xFFFF
    for addr in 0u32..=0xFFFF {
        // set the address pins
        let pins_ext0 = (
            ((addr >> 2) & 0b111111) << 0
            | ((addr >> 17) & 0b11) << 6
        ) as u8;
        let pins_ext1 = (
            // keep ~{chip select} and ~{output enable} high
            0b0011_0000
            | ((addr >> 0) & 0b11) << 6
        ) as u8;
        // ext2 is all data pins
        let pins_ext3 = (
            // keep ~{byte} high
            0b0000_1000
            | ((addr >> 14) & 0b111) << 0
        ) as u8;
        let pins_ext4 = (
            ((addr >> 19) & 0b1) << 1
            | ((addr >> 8) & 0b111111) << 2
        ) as u8;

        ext0.set_pin_values(&mut peripherals, pins_ext0);
        ext1.set_pin_values(&mut peripherals, pins_ext1);
        ext3.set_pin_values(&mut peripherals, pins_ext3);
        ext4.set_pin_values(&mut peripherals, pins_ext4);

        // pull Chip Select down
        ext1.disable_pins(&mut peripherals, 0b0010_0000);

        // pull Output Enable down
        ext1.disable_pins(&mut peripherals, 0b0001_0000);

        // read data
        // ext0 is all address pins
        let ext1_values = ext1.read_pin_values(&mut peripherals);
        let ext2_values = ext2.read_pin_values(&mut peripherals);
        let ext3_values = ext3.read_pin_values(&mut peripherals);
        // ext4 is all address pins

        let upper_byte =
            ((ext1_values >> 2) & 0b1) << 0 // 8
            | ((ext1_values >> 0) & 0b1) << 1 // 9
            | ((ext2_values >> 6) & 0b1) << 2 // 10
            | ((ext2_values >> 4) & 0b1) << 3 // 11
            | ((ext2_values >> 2) & 0b1) << 4 // 12
            | ((ext2_values >> 0) & 0b1) << 5 // 13
            | ((ext3_values >> 6) & 0b1) << 6 // 14
            | ((ext3_values >> 4) & 0b1) << 7 // 15
        ;
        let lower_byte =
            ((ext1_values >> 3) & 0b1) << 0
            | ((ext1_values >> 1) & 0b1) << 1
            | ((ext2_values >> 7) & 0b1) << 2
            | ((ext2_values >> 5) & 0b1) << 3
            | ((ext2_values >> 3) & 0b1) << 4
            | ((ext2_values >> 1) & 0b1) << 5
            | ((ext3_values >> 7) & 0b1) << 6
            | ((ext3_values >> 5) & 0b1) << 7
        ;

        // transmit as big-endian via the UART
        crate::uart::write(&mut peripherals, &[upper_byte, lower_byte]);

        // next address!
    }

    loop {
    }
}
