// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::State::Pins;
use crate::i2c::I2C;
use core::cell::RefCell;
use crate::switch::Switch;
use generic_array::typenum::U16;
use generic_array::GenericArray;
use generic_array::sequence::GenericSequence;
use crate::debouncer::{Keys, Debouncer};
use crate::device::{Device, State};

/// TCA9555
/// PCA9555も同じ
pub struct TCA9555 {
    dev_addr: u8,
    register: Keys<U16>,
    debouncer: Debouncer<U16>,
    switches: GenericArray<Switch, U16>
}

impl TCA9555 {

    fn new(addr: u8, debounce: u16) -> Self {
        Self {
            dev_addr: 0x20u8 + addr,
            register: Keys::default(),
            debouncer: Debouncer::new(debounce),
            switches: GenericArray::generate(|_| Switch::dummy())
        }
    }
}

/// I2Cの実装がMCU（チップセット）毎にバラバラなので、エラーの型をジェネリクスのパラメータで渡す形になってしまう
impl<I2cError> Device<'static, I2cError> for TCA9555 {

    fn init_device(&self, i2cm: &RefCell<dyn I2C<I2cError>>) -> Result<(), I2cError> {
        // All input
        let mut s = i2cm.borrow_mut();
        s.write(self.dev_addr, &[0x06u8, 0xFFu8])?;
        s.write(self.dev_addr, &[0x07u8, 0xFFu8])
    }

    fn read_device(&mut self, i2cm: &RefCell<dyn I2C<I2cError>>) -> Result<State, I2cError> {
        let reg_addr = &[0x00u8];
        let data = &mut [0x00u8, 0x00u8];
        let mut s = i2cm.borrow_mut();
        s.write_read(self.dev_addr, reg_addr, data)?;

        // todo: registerにboolの配列として格納する

        Ok(Pins(&self.register.pressed[0..]))
    }

    fn assign(&mut self, pin: usize, switch: Switch) -> Result<&Switch, Switch> {
        if pin < 16 {
            self.switches[pin] = switch;
            let s = &self.switches[pin];
            Ok(s)
        } else {
            Err(switch)
        }
    }
}
