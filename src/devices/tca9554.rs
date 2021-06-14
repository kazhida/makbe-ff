// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::i2c::I2C;
use crate::switch::Switch;
use crate::debouncer::{Debouncer};
use crate::device::{Device, DeviceState};
use crate::event::EventBuffer;
use heapless::Vec;
use heapless::consts::U8;
use core::cell::RefCell;
use crate::device::DeviceState::Pins8;
use crate::event::IndexEvent::{PressedAt, ReleasedAt};
use crate::event::KeyEvent::{Pressed, Released};

/// TCA9554
/// PCA9554も同じ
pub struct TCA9554 {
    dev_addr: u8,
    debouncer: RefCell<Debouncer<U8>>,
    switches: Vec<Switch, U8>
}

impl TCA9554 {

    pub fn new(addr: u8, debounce: u16) -> Self {
        Self {
            dev_addr: 0x20u8 + addr,
            debouncer: RefCell::new(Debouncer::new(debounce)),
            switches: Vec::default()
        }
    }

    fn ref_switch(&'static self, i: usize) -> &'static Switch {
        &self.switches[i]
    }
}

/// I2Cの実装がMCU（チップセット）毎にバラバラなので、エラーの型をジェネリクスのパラメータで渡す形になってしまう
impl<I2cError> Device<I2cError> for TCA9554 {

    fn init_device(&self, i2c: &mut dyn I2C<I2cError>) -> Result<(), I2cError> {
        // All input
        i2c.write(self.dev_addr, &[0x06_u8, 0xFF_u8])?;
        i2c.write(self.dev_addr, &[0x07_u8, 0xFF_u8])
    }

    fn read_device(&self, i2c: &mut dyn I2C<I2cError>) -> Result<DeviceState, I2cError> {
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

    fn assign(&mut self, pin: usize, switch: Switch) -> Result<usize, usize> {
        if pin < 8 {
            self.switches[pin] = switch;
            Ok(pin)
        } else {
            Err(pin)
        }
    }

    fn has_assigned(&self) -> bool {
        self.switches.iter().any(|s| s.actions.len() > 0)
    }

    fn add_event(&'static self, pins: &[bool], events: &mut EventBuffer) {
        let indexes = self.debouncer.borrow_mut().events(pins);
        for idx in indexes.buffer {
            let event = match idx {
                PressedAt(i) => Pressed(self.ref_switch(i)),
                ReleasedAt(i) => Released(self.ref_switch(i))
            };
            let _ = events.buffer.push(event);
        }
    }
}
