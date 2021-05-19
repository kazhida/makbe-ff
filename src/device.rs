// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use embedded_hal::blocking::i2c::{Write, WriteRead};
use std::cell::RefCell;

pub trait Device {
    fn init_pins(&self, i2cm: &RefCell<dyn Write>) -> Result<(), Write::Error>;
    fn read_pins(&mut self, i2cm: &RefCell<dyn WriteRead>) -> Result<&[bool], WriteRead::Error>;
}
