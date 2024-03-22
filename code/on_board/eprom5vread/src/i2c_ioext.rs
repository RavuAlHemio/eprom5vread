//! Handlers for the Microchip MCP23008 I2C I/O expander.


use from_to_repr::FromToRepr;
use stm32g0b0::Peripherals;


/// Base 7-bit I2C address of the I/O expander.
///
/// The lowest three bits can be set independently for each expander.
///
/// When transmitting the I2C address, the 7-bit address is followed by the direction bit, which is
/// 0 for writes and 1 for reads.
pub const I2C_BASE_ADDRESS: u8 = 0b010_0000;


/// Registers of the I/O expander and their addresses.
#[derive(Clone, Copy, Debug, Default, Eq, FromToRepr, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum Register {
    /// Bit field containing the I/O direction for the GPIO pins (0 = output, 1 = input).
    ///
    /// After reset, all pins are inputs.
    #[default] IoDir = 0x00,

    /// Polarity for input GPIO pins (0 = regular, 1 = inverse).
    ///
    /// With regular polarity, a pin pulled high reads 1 and a pin pulled low reads 0. With inverse
    /// polarity, a pin pulled high reads 0 and a pin pulled low reads 1. The same inversion is
    /// applied to the latches of output pins.
    ///
    /// After reset, all pins are set to regular polarity.
    IPol = 0x01,

    /// Enables or disables GPIO Interrupt-on-Change for pins (0 = disable, 1 = enable).
    ///
    /// If the corresponding pin in GPIntEn is set and the condition for the interrupt is met, the
    /// INT line of the port expander is asserted. The interrupt condition is set in `IntCon`; the
    /// polarity of the INT line in the `IntPol` bit of `IoCon`.
    ///
    /// By default, interrupts are disabled for all pins.
    GPIntEn = 0x02,

    /// Sets the per-pin default value for comparisons if the interrupt condition is set to compare.
    ///
    /// By default, the comparison value is 0.
    DefVal = 0x03,

    /// Sets the interrupt condition for the given pin (0 = edge, 1 = compare).
    ///
    /// In edge mode, an interrupt is raised on any signal edge, i.e. when the pin changes value.
    /// In compare mode, an interrupt is raised when the pin changes value to one different than the
    /// corresponding bit in `DefVal`.
    ///
    /// By default, the interrupt condition for all pins is edge.
    IntCon = 0x04,

    /// Sets configuration options for the port expander.
    ///
    /// bit 0 (LSB): reserved, reads 0
    /// bit 1: `IntPol`, sets the polarity of the INT pin (0 = active-low, 1 = active-high)
    /// bit 2: `ODr`, configures the INT pin as an open-drain output (0 = active driver, 1 = open drain)
    /// bit 3: reserved on the I2C model
    /// bit 4: `DisSlw`, disables slew rate control on the SDA pin (0 = slew rate enabled, 1 = slew rate disabled)
    /// bit 5: `SeqOp`, enables or disables sequential operation mode (0 = advance to next address, 1 = stay at address)
    /// bits 6 and 7 (MSB): reserved, read 0
    ///
    /// By default, all options are set to 0.
    IoCon = 0x05,

    /// Enables or disables pull-up resistors for the given pins if they are inputs (0 = disabled, 1 = enabled).
    ///
    /// By default, pull-ups are disabled.
    GPPU = 0x06,

    /// Stores which pins caused an interrupt since the last time IntCap or GPIO have been read.
    ///
    /// If a pin's corresponding bit is set to 1, it has caused an interrupt.
    ///
    /// By default, the interrupt states are set to 0.
    IntF = 0x07,

    /// Stores the value of all pins when the first pin (since the last time IntCap or GPIO have
    /// been read) caused an interrupt.
    IntCap = 0x08,

    /// Contains the current state of the GPIO registers.
    ///
    /// Writing to this register has the same effect as writing to `OLat`.
    GPIO = 0x09,

    /// Sets the desired state of the output register latches (0 = logic low, 1 = logic high).
    ///
    /// The actual state of the output pins depends on the polarity configured in `IPol`.
    ///
    /// By default, all pins are set to logic-low.
    OLat = 0x0A,
}

const BIT_SEQ_OP: u8 = 5;


pub(crate) struct IoExtHandler {
    /// The I2C address of the I/O expander.
    i2c_address: u8,

    /// Bit mask of which GPIO pins on the I/O expander are set to input.
    input_pins: u8,

    /// The current value of the output pins on the I/O expander.
    output_pin_value: u8,
}
impl IoExtHandler {
    pub fn new(peripherals: &mut Peripherals, i2c_address: u8) -> Self {
        let mut data_array = [0u8];

        // read current settings
        crate::i2c::write(peripherals, i2c_address, &[Register::IoCon.into()]);
        crate::i2c::read(peripherals, i2c_address, &mut data_array);
        if (data_array[0] & (1 << BIT_SEQ_OP)) == 0 {
            // sequential mode is on; disable it
            data_array[0] &= !(1 << BIT_SEQ_OP);
            crate::i2c::write(peripherals, i2c_address, &[Register::IoCon.into(), data_array[0]]);
        }

        // ask for the data direction
        crate::i2c::write(peripherals, i2c_address, &[Register::IoDir.into()]);
        crate::i2c::read(peripherals, i2c_address, &mut data_array);
        let input_pins = data_array[0];

        // set the polarity to regular
        crate::i2c::write(peripherals, i2c_address, &[Register::IPol.into(), 0x00]);
        
        // disable interrupts
        crate::i2c::write(peripherals, i2c_address, &[Register::GPIntEn.into(), 0x00]);

        // read the current output pin state
        crate::i2c::write(peripherals, i2c_address, &[Register::OLat.into()]);
        crate::i2c::read(peripherals, i2c_address, &mut data_array);
        let output_pin_value = data_array[0] & input_pins;

        Self {
            i2c_address,
            input_pins,
            output_pin_value,
        }
    }

    pub const fn get_pin_direction(&self) -> u8 { self.input_pins }

    pub fn set_pin_direction(&mut self, peripherals: &mut Peripherals, input_pins: u8) {
        if self.input_pins == input_pins {
            return;
        }

        crate::i2c::write(peripherals, self.i2c_address, &[Register::OLat.into(), input_pins]);
        self.input_pins = input_pins;

        // also mask output_pin_value with the inverse of input_pins
        self.output_pin_value &= !input_pins;
    }

    pub fn set_pin_values(&mut self, peripherals: &mut Peripherals, pin_values: u8) {
        let set_value = pin_values & !self.input_pins;
        if self.output_pin_value == set_value {
            return;
        }

        crate::i2c::write(peripherals, self.i2c_address, &[Register::OLat.into(), set_value]);
        self.output_pin_value = set_value;
    }

    pub fn read_pin_values(&self, peripherals: &mut Peripherals) -> u8 {
        let mut buf = [0];
        crate::i2c::write(peripherals, self.i2c_address, &[Register::GPIO.into()]);
        crate::i2c::read(peripherals, self.i2c_address, &mut buf);
        buf[0]
    }

    pub fn enable_pins(&mut self, peripherals: &mut Peripherals, enable_values: u8) {
        let masked_value = enable_values & !self.input_pins;
        let target_value = self.output_pin_value | masked_value;
        self.set_pin_values(peripherals, target_value);
    }

    pub fn disable_pins(&mut self, peripherals: &mut Peripherals, disable_values: u8) {
        let masked_value = disable_values & !self.input_pins;
        let target_value = self.output_pin_value & (!masked_value);
        self.set_pin_values(peripherals, target_value);
    }
}
