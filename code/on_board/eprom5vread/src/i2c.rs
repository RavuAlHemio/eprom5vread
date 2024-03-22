//! Functions for I2C (Inter-Integrated Circuit) communication on the STM32G0B0.


use stm32g0b0::Peripherals;


pub(crate) fn setup(peripherals: &mut Peripherals) {
    // enable clock for relevant GPIO (PORTB)
    peripherals.RCC.iopenr().modify(|_, w| w
        .gpioben().set_bit()
    );

    // configure PB6 and PB7 as Alternative Function 6
    peripherals.GPIOB.moder().modify(|_, w| w
        .moder6().alternate_function()
        .moder7().alternate_function()
    );
    peripherals.GPIOB.afrl().modify(|_, w| w
        .afsel6().af6()
        .afsel7().af6()
    );

    // enable clock for I2C1
    peripherals.RCC.apbenr1().modify(|_, w| w
        .i2c1en().set_bit()
    );

    // disable, configure, then enable I2C1
    peripherals.I2C1.i2c_cr1().modify(|_, w| w
        .pe().clear_bit()
    );
    peripherals.I2C1.i2c_cr1().modify(|_, w| w
        .txie().clear_bit() // disable transmit interrupt
        .rxie().clear_bit() // disable receive interrupt
        .addrie().clear_bit() // disable "address match" interrupt (only interesting when implementing peripherals)
        .nackie().clear_bit() // disable "not-acknowledge received" interrupt
        .stopie().clear_bit() // disable "stop detection" interrupt
        .tcie().clear_bit() // disable "transfer complete" interrupt
        .errie().clear_bit() // disable error interrupt
        .dnf().disabled() // disable digital noise filter
        .anfoff().clear_bit() // leave analog noise filter on
        .txdmaen().clear_bit() // disable direct memory access for transmissions
        .rxdmaen().clear_bit() // disable direct memory access for receptions
        .sbc().clear_bit() // disable peripheral byte control
        .nostretch().clear_bit() // disable clock stretching (only interesting when implementing peripherals; must remain 0 for controllers)
        .wupen().clear_bit() // disable waking up from Stop mode
        .gcen().clear_bit() // disable responding to general call (address 0)
        .smbhen().clear_bit() // disable responding to SMBus host address (address 0b0001000x)
        .smbden().clear_bit() // disable responding to SMBus device default address (address 0b1100001x)
        .alerten().clear_bit() // disable SMBus alert
        .pecen().clear_bit() // disable packet error checking byte calculation (SMBus)
    );
    peripherals.I2C1.i2c_cr2().modify(|_, w| w
        .sadd().variant(0x00) // peripheral address (leave this at 0 for the time being)
        .rd_wrn().read() // data direction relative to controller (set to read for the time being)
        .add10().addr_7_bits() // use 7-bit addressing (not 10-bit)
        .start().clear_bit() // do not generate Start condition yet
        .stop().clear_bit() // do not generate Stop condition yet
        .nack().clear_bit() // do not generate not-acknowledge condition yet (only interesting when implementing peripherals)
        .nbytes().variant(0) // send/receive 0 bytes by default
        .reload().clear_bit() // transfer ends after NBYTES bytes
        .autoend().set_bit() // automatically generate an End condition after NBYTES are transferred
        .pecbyte().clear_bit() // do not transmit/receive a packet error checking byte (SMBus)
    );
    // these values are mostly taken from Table 110 (Examples of timings settings for f_{I2CCLK} = 16 MHz)
    // in reference manual RM0454, in 10 kHz Standard mode
    peripherals.I2C1.i2c_timingr().modify(|_, w| w
        .presc().variant(3)
        .scll().variant(0xC7)
        .sclh().variant(0xC3)
        .sdadel().variant(0x2)
        .scldel().variant(0x4)
    );
    peripherals.I2C1.i2c_cr1().modify(|_, w| w
        .pe().set_bit() // enable I2C
    );
}

/// Writes the given byte slice via I2C to the peripheral identified using the given address.
///
/// If the write has been successful, returns `Ok(())`. If the peripheral answered with a
/// not-acknowledged (NAK) state after a specific byte, returns `Err(i)` where `i` is the zero-based
/// index of the byte which provoked the NAK state.
pub(crate) fn write(peripherals: &mut Peripherals, address: u8, slice: &[u8]) -> Result<(), usize> {
    assert_eq!(address & 0b1000_0000, 0b0000_0000);
    assert!(slice.len() < 0x100);

    // load address, transfer direction and byte count
    peripherals.I2C1.i2c_cr2().modify(|_, w| w
        .add10().addr_7_bits()
        .sadd().variant(address.into())
        .rd_wrn().write() // controller to peripheral
        .nbytes().variant(slice.len() as u8)
        .autoend().set_bit() // automatically send Stop once we're done
    );

    // transmit Start state
    peripherals.I2C1.i2c_cr2().modify(|_, w| w
        .start().set_bit()
    );

    for (i, &b) in slice.iter().enumerate() {
        // wait for I2C to free up
        loop {
            let i2c_state = peripherals.I2C1.i2c_isr().read();
            if i2c_state.txis().bit_is_set() {
                // ready to send next bit
                break;
            } else if i2c_state.nackf().bit_is_set() {
                // peripheral sent NAK
                return Err(i.wrapping_sub(1));
            }

            // keep waiting otherwise
        }

        // feed byte
        peripherals.I2C1.i2c_txdr().modify(|_, w| w
            .txdata().variant(b)
        );
    }

    // we are done sending
    // wait until transmission is complete
    loop {
        let i2c_state = peripherals.I2C1.i2c_isr().read();
        if i2c_state.tc().bit_is_set() {
            // done
            break Ok(());
        } else if i2c_state.nackf().bit_is_set() {
            // peripheral sent NAK after last byte
            break Err(slice.len());
        }

        // keep waiting otherwise
    }
}

/// Reads into the given byte slice via I2C from the peripheral identified using the given address.
pub(crate) fn read(peripherals: &mut Peripherals, address: u8, slice: &mut [u8]) {
    assert_eq!(address & 0b1000_0000, 0b0000_0000);
    assert!(slice.len() < 0x100);

    // load address, transfer direction and byte count
    peripherals.I2C1.i2c_cr2().modify(|_, w| w
        .add10().addr_7_bits()
        .sadd().variant(address.into())
        .rd_wrn().read() // peripheral to controller
        .nbytes().variant(slice.len() as u8)
        .autoend().set_bit() // automatically send Stop once we're done
    );

    // transmit Start state
    peripherals.I2C1.i2c_cr2().modify(|_, w| w
        .start().set_bit()
    );

    for b in slice {
        // wait for I2C to receive a bit
        while peripherals.I2C1.i2c_isr().read().rxne().bit_is_clear() {
        }

        // read it out
        *b = peripherals.I2C1.i2c_rxdr().read().rxdata().bits();
    }

    // we are done receiving
    // wait until transmission is complete
    while peripherals.I2C1.i2c_isr().read().tc().bit_is_clear() {
    }
}
