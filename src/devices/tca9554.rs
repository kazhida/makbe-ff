// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::i2c::I2C;
use crate::switch::Switch;
use crate::debouncer::{Debouncer};
use crate::device::{Device, State};
use crate::event::EventBuffer;
use heapless::Vec;
use heapless::consts::U8;
use core::cell::RefCell;
use crate::device::State::Pins8;

/// TCA9554
/// PCA9554も同じ
pub struct TCA9554 {
    dev_addr: u8,
    debouncer: RefCell<Debouncer<U8>>,
    switches: Vec<&'static Switch, U8>
}

impl TCA9554 {

    pub fn new(addr: u8, debounce: u16) -> Self {
        Self {
            dev_addr: 0x20u8 + addr,
            debouncer: RefCell::new(Debouncer::new(debounce)),
            switches: Vec::default()
        }
    }
}

/// I2Cの実装がMCU（チップセット）毎にバラバラなので、エラーの型をジェネリクスのパラメータで渡す形になってしまう
impl<I2cError> Device<I2cError> for TCA9554 {

    fn init_device(&self, i2c: &mut dyn I2C<I2cError>) -> Result<(), I2cError> {
        // All input
        i2c.write(self.dev_addr, &[0x06_u8, 0xFF_u8])?;
        i2c.write(self.dev_addr, &[0x07_u8, 0xFF_u8])
    }

    fn read_device(&self, i2c: &mut dyn I2C<I2cError>) -> Result<State, I2cError> {
        let reg_addr = &[0x00_u8];
        let data = &mut [0x00_u8];
        i2c.write_read(self.dev_addr, reg_addr, data)?;

        let mut pressed = [false;  8];
        let mut mask = 0x01_u8;
        for j in 0..8 {
            pressed[j] = mask & data[0] == 0;    // スイッチが押されていたらLowなので0
            mask <<= 1;
        }
        Ok(Pins8(pressed))
    }

    fn assign(&mut self, pin: usize, switch: &'static Switch) -> Result<&Switch, &Switch> {
        if pin < 8 {
            self.switches[pin] = switch;
            Ok(switch)
        } else {
            Err(switch)
        }
    }

    fn add_event(&self, pins: &[bool], events: &mut EventBuffer) {
        self.debouncer.borrow_mut().add_events(pins, events, |i| &self.switches[i])
    }
}
