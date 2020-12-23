use std::io::Result;

use num_derive::ToPrimitive;
use num_traits::ToPrimitive;

use usb2642_i2c::USB2642I2C;

const I2C_ADDRESS: u8 = 0x41;

#[derive(ToPrimitive)]
pub enum Register {
    InputPort = 0,
    OutputPort = 1,
    PolarityInversion = 2,
    Configuration = 3,
}

bitflags! {
    pub struct GpioPin: u8 {
        const NONE = 0b0000;
        const P0 = 0b0001;
        const P1 = 0b0010;
        const P2 = 0b0100;
        const P3 = 0b1000;
        const ALL = 0b1111;
    }
}

#[derive(ToPrimitive)]
pub enum Direction {
    Output = 0,
    Input = 1,
}

pub struct PCA9536 {
    usb2642: USB2642I2C,
    direction_mask: u8,
}

impl PCA9536 {
    pub fn new(usb2642: USB2642I2C) -> Self {
        Self {
            usb2642,
            direction_mask: 0xff,
        }
    }

    fn write_register(&mut self, register: Register, value: u8) -> Result<()> {
        let mut payload = [register.to_u8().unwrap(), value];
        self.usb2642.write(I2C_ADDRESS, &mut payload)?;
        Ok(())
    }

    fn read_register(&mut self, register: Register) -> Result<u8> {
        let data = [register.to_u8().unwrap()];
        let data = self.usb2642.write_read(I2C_ADDRESS, &data, 1)?;
        Ok(data[0])
    }

    pub fn get_input_port(&mut self) -> Result<u8> {
        self.read_register(Register::InputPort)
    }

    pub fn set_output_value(&mut self, pins: GpioPin) -> Result<()> {
        self.write_register(Register::OutputPort, pins.bits)
    }

    pub fn set_polarity_inversion(&mut self, pins: GpioPin) -> Result<()> {
        self.write_register(Register::PolarityInversion, pins.bits)
    }

    pub fn set_pin_direction(&mut self, pins: GpioPin, direction: Direction) -> Result<()> {
        match direction {
            Direction::Output => {
                self.direction_mask &= !pins.bits;
            }
            Direction::Input => {
                self.direction_mask &= pins.bits;
            }
        }
        self.write_register(Register::Configuration, self.direction_mask)
    }
}
