use std::{error::Error, fmt::Display};

use structopt::StructOpt;

use usbsdmux::{Mode, USBSDMux};

#[derive(Debug)]
struct ParseModeError {
    mode: String,
}

impl ParseModeError {
    pub fn new(mode: String) -> Self {
        Self { mode }
    }
}

impl Display for ParseModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Unknown mode `{}`", self.mode)
    }
}

fn parse_mode(mode: &str) -> Result<Mode, ParseModeError> {
    if mode == "host" {
        Ok(Mode::Host)
    } else if mode == "dut" {
        Ok(Mode::DUT)
    } else if mode == "off" {
        Ok(Mode::Off)
    } else {
        Err(ParseModeError::new(mode.into()))
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    sg_dev: String,
    #[structopt(parse(try_from_str = parse_mode))]
    mode: Mode,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let mut usbsdmux = USBSDMux::new(opt.sg_dev)?;
    usbsdmux.set_mode(opt.mode)?;

    println!("Success");

    Ok(())
}
