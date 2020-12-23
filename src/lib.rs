//! Program and library to control the [USB-SD-Mux device](https://shop.linux-automation.com/usb_sd_mux-D02-R01-V02-C00)
//!
//! The program has the same interface as the original [python control program](https://github.com/linux-automation/usbsdmux) by the manufacturer:
//! ```sh
//! $ usbsdmux /dev/sg0 host
//! $ usbsdmux /dev/sg0 dut
//! $ usbsdmux /dev/sg0 off
//! ```
//!
//! If you want to run the program as normal user you can set the setuid flag:
//!
//! ```sh
//! $ sudo chown root /path/to/usbsdmux
//! $ sudo chmod u+s /path/to/usbsdmux
//! ```
//!

#[macro_use]
extern crate bitflags;

mod pca9536;

use std::{io::Result, thread::sleep, time::Duration};

use usb2642_i2c::USB2642I2C;

use pca9536::{Direction, GpioPin, PCA9536};

#[derive(Debug)]
pub enum Mode {
    Off,
    DUT,
    Host,
}

const DAT_ENABLE: GpioPin = GpioPin::NONE;
const DAT_DISABLE: GpioPin = GpioPin::P0;

const PWR_ENABLE: GpioPin = GpioPin::NONE;
const PWR_DISABLE: GpioPin = GpioPin::P1;

const SELECT_DUT: GpioPin = GpioPin::P2;
const SELECT_HOST: GpioPin = GpioPin::NONE;

const CARD_INSERTED: GpioPin = GpioPin::NONE;
const CARD_REMOVED: GpioPin = GpioPin::P3;

pub struct USBSDMux {
    pca9536: PCA9536,
}

impl USBSDMux {
    pub fn new<S: Into<String>>(sg_dev: S) -> Result<Self> {
        let usb2642 = USB2642I2C::open(sg_dev)?;

        let mut pca9536 = PCA9536::new(usb2642);

        pca9536.set_pin_direction(GpioPin::ALL, Direction::Output)?;

        Ok(Self { pca9536 })
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<()> {
        match mode {
            Mode::Off => {
                self.pca9536
                    .set_output_value(DAT_DISABLE | PWR_DISABLE | SELECT_HOST | CARD_REMOVED)?;

                sleep(Duration::from_millis(300));
            }
            Mode::DUT => {
                self.set_mode(Mode::Off)?;
                self.pca9536
                    .set_output_value(DAT_DISABLE | PWR_DISABLE | SELECT_DUT | CARD_REMOVED)?;
                self.pca9536
                    .set_output_value(DAT_ENABLE | PWR_ENABLE | SELECT_DUT | CARD_REMOVED)?;
            }
            Mode::Host => {
                self.set_mode(Mode::Off)?;
                self.pca9536
                    .set_output_value(DAT_ENABLE | PWR_ENABLE | SELECT_HOST | CARD_INSERTED)?;
            }
        }
        Ok(())
    }
}
