#![no_main]
#![no_std]


mod setup;
mod uart;


use core::panic::PanicInfo;

use cortex_m_rt::entry;
use paste::paste;
use stm32g0b0::Peripherals;


#[panic_handler]
fn oh_no(_info: &PanicInfo) -> ! {
    loop {
    }
}

macro_rules! set_by_address {
    ($w:expr, $pin:expr, $address:expr, $addr_bit:expr) => { paste! {
        if ($address & (1 << $addr_bit)) != 0 {
            $w.[<bs $pin>]().set_bit()
        } else {
            $w.[<br $pin>]().set_bit()
        }
    } };
}
macro_rules! get_for_data {
    ($read_reg:expr, $pin:expr, $data:expr, $data_bit:expr) => { paste! {
        if $read_reg.[<idr $pin>]().bit_is_set() {
            $data |= 1 << $data_bit;
        }
    } };
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take()
        .expect("peripherals taken?!");

    crate::setup::setup_clocks(&mut peripherals);
    crate::uart::setup(&mut peripherals);
    crate::uart::write(&mut peripherals, b"Well hello there!\r\n");

    // M27C800-100F1 in word (not byte) organisation

    // turn on clocks to all relevant GPIOs
    peripherals.RCC.iopenr().modify(|_, w| w
        .gpioaen().set_bit()
        .gpioben().set_bit()
        .gpiocen().set_bit()
        .gpioden().set_bit()
        .gpiofen().set_bit()
    );

    // ensure that level shifter output is disabled and the chip is unselected
    peripherals.GPIOA.bsrr().write(|w| w
        .br0().set_bit() // level shifter output enable to low
    );
    peripherals.GPIOB.bsrr().write(|w| w
        .bs3().set_bit() // not-output-enable to high
        .bs4().set_bit() // not-chip-select to high
    );

    // set up pin directions
    // A pins (address): output
    // Q pins (data): input (we aren't writing the ROM)
    // control pins: output
    peripherals.GPIOA.moder().modify(|_, w| w
        .moder0().output() // level shifter output enable
        .moder1().output() // A8
        .moder2().output() // A9
        .moder3().output() // A10
        .moder4().output() // A11
        .moder5().output() // A12
        .moder6().output() // A13
        .moder7().output() // A14
        .moder8().input() // Q5
        // PA9+10: USART1
        .moder11().input() // Q10
        .moder12().input() // Q3
        // PA13+14: SWD
        .moder15().input() // Q11
    );
    peripherals.GPIOB.moder().modify(|_, w| w
        .moder0().output() // A15
        .moder1().output() // A16
        .moder2().output() // ~{BYTE}V_{PP} = high for word-level, low for byte-level access
        .moder3().output() // ROM ~{OE} = low to enable data output (documented as ~{G})
        .moder4().output() // ROM ~{CS} = low to select chip (documented as ~{E})
        .moder5().output() // A0
        .moder6().output() // A1
        .moder7().output() // A2
        .moder8().output() // A3
        .moder9().output() // A4
        .moder10().input() // Q15A-1: address in byte mode; data in word mode
        .moder11().input() // Q7
        .moder12().input() // Q14
        .moder13().input() // Q6
        .moder14().input() // Q4
        .moder15().input() // Q12
    );
    peripherals.GPIOC.moder().modify(|_, w| w
        // pins 0-5 missing
        .moder6().output() // Q13
        .moder7().output() // Q2
        // pins 8-12 missing
        .moder13().input() // A5
        .moder14().input() // A6
        .moder15().input() // A7
    );
    peripherals.GPIOD.moder().modify(|_, w| w
        .moder0().output() // Q9
        .moder1().output() // Q1
        .moder2().output() // Q8
        .moder3().output() // Q0
        // pins 4-15 missing
    );
    // GPIOE missing
    peripherals.GPIOF.moder().modify(|_, w| w
        .moder0().output() // A17
        .moder1().output() // A18
        // pins 2-15 missing
    );

    // enable level shifter output

    // word address has 19 bits => store as u32
    const MAX_ADDR: u32 = 0b0111_1111_1111_1111_1111;

    for address in 0..=MAX_ADDR {
        // address bits in GPIOA:
        // | PA7 | PA6 | PA5 | PA4 | PA3 | PA2 | PA1 | PA0 |
        // | A14 | A13 | A12 | A11 | A10 |  A9 |  A8 |   x |
        //
        // address bits in GPIOB:
        // | PB9 | PB8 | PB7 | PB6 | PB5 | PB4 | PB3 | PB2 | PB1 | PB0 |
        // |  A4 |  A3 |  A2 |  A1 |  A0 |   x |   x |   x | A16 | A15 |
        //
        // address bits in GPIOC:
        // | PC15 | PC14 | PC13 | (13 more PC* pins)
        // |   A7 |   A6 |   A5 |
        //
        // no address bits in GPIOD
        //
        // no GPIOE
        //
        // address bits in GPIOF:
        // | PF1 | PF0 |
        // | A18 | A17 |

        // set address pins
        peripherals.GPIOA.bsrr().write(|w| {
            set_by_address!(w,  1, address,  8);
            set_by_address!(w,  2, address,  9);
            set_by_address!(w,  3, address, 10);
            set_by_address!(w,  4, address, 11);
            set_by_address!(w,  5, address, 12);
            set_by_address!(w,  6, address, 13);
            set_by_address!(w,  7, address, 14);
            w
        });
        peripherals.GPIOB.bsrr().write(|w| {
            set_by_address!(w,  0, address, 15);
            set_by_address!(w,  1, address, 16);
            set_by_address!(w,  5, address,  0);
            set_by_address!(w,  6, address,  1);
            set_by_address!(w,  7, address,  2);
            set_by_address!(w,  8, address,  3);
            set_by_address!(w,  9, address,  4);
            w
        });
        peripherals.GPIOC.bsrr().write(|w| {
            set_by_address!(w, 13, address,  5);
            set_by_address!(w, 14, address,  6);
            set_by_address!(w, 15, address,  7);
            w
        });
        peripherals.GPIOE.bsrr().write(|w| {
            set_by_address!(w,  0, address, 17);
            set_by_address!(w,  1, address, 18);
            w
        });

        // chip select
        peripherals.GPIOB.bsrr().write(|w| w
            .bs4().clear_bit() // not-chip-select to low
        );

        // TODO: NOPs to stabilize

        // output enable
        peripherals.GPIOB.bsrr().write(|w| w
            .bs3().clear_bit() // not-output-enable to low
        );

        // TODO: NOPs to stabilize

        // data bits in GPIOA:
        // | PA15 | PA14 | PA13 | PA12 | PA11 | PA10 |  PA9 |  PA8 | (8 more PA* pins)
        // |  Q11 |    x |    x |   Q3 |  Q10 |    x |    x |   Q5 |
        //
        // data bits in GPIOB:
        // | PB15 | PB14 | PB13 | PB12 | PB11 | PB10 | (10 more PB* pins)
        // |  Q12 |   Q4 |   Q6 |  Q14 |   Q7 |  Q15 |
        //
        // data bits in GPIOC:
        // | PC7 | PC6 | (5 more PC* pins, theoretically)
        // |  Q2 | Q13 |
        //
        // data bits in GPIOD:
        // | PD3 | PD2 | PD1 | PD0 |
        // |  Q0 |  Q8 |  Q1 |  Q9 |
        //
        // no GPIOE
        //
        // no data bits in GPIOF
        let mut data: u16 = 0;

        let a_read = peripherals.GPIOA.idr().read();
        get_for_data!(a_read,  8, data,  5);
        get_for_data!(a_read, 11, data, 10);
        get_for_data!(a_read, 12, data,  3);
        get_for_data!(a_read, 15, data, 11);

        let b_read = peripherals.GPIOB.idr().read();
        get_for_data!(b_read, 10, data, 15);
        get_for_data!(b_read, 11, data,  7);
        get_for_data!(b_read, 12, data, 14);
        get_for_data!(b_read, 13, data,  6);
        get_for_data!(b_read, 14, data,  4);
        get_for_data!(b_read, 15, data, 12);

        let c_read = peripherals.GPIOC.idr().read();
        get_for_data!(c_read, 10, data, 13);
        get_for_data!(c_read, 11, data,  2);

        let d_read = peripherals.GPIOD.idr().read();
        get_for_data!(d_read,  0, data,  9);
        get_for_data!(d_read,  1, data,  1);
        get_for_data!(d_read,  2, data,  8);
        get_for_data!(d_read,  3, data,  0);

        // naïve assumption that we're little-endian
        let bs = data.to_le_bytes();

        // write out via UART
        crate::uart::write(&mut peripherals, &bs[..]);

        // chip disable, output deselect
        peripherals.GPIOB.bsrr().write(|w| w
            .bs3().set_bit() // not-output-enable to high
            .bs4().set_bit() // not-chip-select to high
        );

        // TODO: some sort of parity calculation?
    }

    crate::uart::write(&mut peripherals, b"\r\nand that's it!");

    loop {
    }
}
