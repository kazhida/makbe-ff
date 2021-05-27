// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::i2c::I2C;
use crate::switch::Switch;
use crate::debouncer::{Debouncer};
use crate::device::{Device, DeviceState};
use crate::event::EventBuffer;
use heapless::Vec;
use heapless::consts::U16;
use core::cell::RefCell;
use crate::device::DeviceState::Pins16;

/// TCA9555
/// PCA9555も同じ
pub struct TCA9555 {
    dev_addr: u8,
    debouncer: RefCell<Debouncer<U16>>,
    switches: Vec<&'static Switch, U16>
}

impl TCA9555 {

    fn new(addr: u8, debounce: u16) -> Self {
        Self {
            dev_addr: 0x20u8 + addr,
            debouncer: RefCell::new(Debouncer::new(debounce)),
            switches: Vec::default()
        }
    }
}

/// I2Cの実装がMCU（チップセット）毎にバラバラなので、エラーの型をジェネリクスのパラメータで渡す形になってしまう
impl<I2cError> Device<I2cError> for TCA9555 {

    fn init_device(&self, i2c: &mut dyn I2C<I2cError>) -> Result<(), I2cError> {
        // All input
        i2c.write(self.dev_addr, &[0x06_u8, 0xFF_u8])?;
        i2c.write(self.dev_addr, &[0x07_u8, 0xFF_u8])
    }

    fn read_device(&self, i2c: &mut dyn I2C<I2cError>) -> Result<DeviceState, I2cError> {
        let reg_addr = &[0x00_u8];
        let data = &mut [0x00_u8, 0x00_u8];
        i2c.write_read(self.dev_addr, reg_addr, data)?;

        let mut pressed = [false;  16];
        for i in 0..2 {
            let mut mask = 0x01_u8;
            for j in 0..8 {
                let index = i * 8 + j;
                pressed[index] = mask & data[i] == 0;    // スイッチが押されていたらLowなので0
                mask <<= 1;
            }
        }
        Ok(Pins16(pressed))
    }

    fn assign(&mut self, pin: usize, switch: &'static Switch) -> Result<&Switch, &Switch> {
        if pin < 16 {
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
