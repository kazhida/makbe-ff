// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::Device;
use std::cell::RefCell;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub struct TCA9555 {
    dev_addr: u8,
    register: [bool; 16]
}

impl Device for TCA9555 {


    fn init_pins(&self, i2cm: &RefCell<dyn Write>) -> Result<(), Write::Error> {
        // All input
        let mut s = i2cm.borrow_mut();
        s.write(self.dev_addr, &[0x06u8, 0xFFu8])?;
        s.write(self.dev_addr, &[0x07u8, 0xFFu8])
    }

    fn read_pins(&mut self, i2cm: &RefCell<dyn WriteRead>) -> Result<&[bool], WriteRead::Error> {
        let reg_addr = &[0x00u8];
        let mut data = &[0x00u8, 0x00u8];
        i2cm.write_read(self.dev_addr, reg_addr, data)?;

        // todo: registerにboolの配列として格納する

        Ok(&self.register)
    }
}
