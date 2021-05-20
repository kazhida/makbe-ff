// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::{Device, State};
use crate::device::State::Pins;
use crate::i2c::I2C;
use core::cell::RefCell;
use crate::switch::Switch;

/// TCA9555
/// PCA9555も同じ
pub struct TCA9555 {
    dev_addr: u8,
    register: [bool; 16],
    switches: [Switch; 16]
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

        Ok(Pins(&self.register))
    }

    fn assign(&mut self, pin: usize, switch: Switch) -> Result<Switch, Switch> {
        self.switches[pin] = switch.clone();
        Ok(switch)
    }
}
