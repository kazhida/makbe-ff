// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::Device;
use crate::i2c::I2C;
use crate::device::State::Pins;
use generic_array::{GenericArray, ArrayLength};
use core::cell::RefCell;

/// deviceを使用して、キーの状態をスキャンするもの
pub struct Scanner<'a, I2cError, NumDevices>
    where
        NumDevices: ArrayLength<&'a mut dyn Device<'a, I2cError>>
{
    i2c: &'a RefCell<dyn I2C<I2cError>>,
    devices: GenericArray<&'a mut dyn Device<'a, I2cError>, NumDevices>
}

impl<'a, I2cError, NumDevices> Scanner<'a, I2cError, NumDevices>
    where
        NumDevices: ArrayLength<&'a mut dyn Device<'a, I2cError>>
{

    pub fn scan(&mut self) {
        for d in self.devices.as_mut_slice() {
            let result = d.read_device(&self.i2c);
            match result {
                Ok(state) => {
                    match state {
                        Pins(pins) => {
                            for (j, p) in pins.iter().enumerate() {
                                // pressed_keys[i][j] = p;
                            }
                        }
                        _ => {
                            // ロータリーエンコーダのこととかはまだ考えない
                        }
                    }
                },
                Err(e) => {
                    // どうしよっか？
                }
            }
        }
    }
}
