// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::Device;
use crate::i2c::I2C;
use heapless::{Vec, ArrayLength};
use crate::event::EventBuffer;
use core::ops::DerefMut;
use crate::device::State::{Pins16, Pins8};

/// deviceを使用して、キーの状態をスキャンするもの
pub struct Scanner<I2cError: 'static, NumDevices>
    where
        NumDevices: ArrayLength<&'static mut dyn Device<I2cError>>
{
    i2c: &'static mut dyn I2C<I2cError>,
    devices: Vec<&'static mut dyn Device<I2cError>, NumDevices>
}

impl<I2cError, NumDevices> Scanner<I2cError, NumDevices>
    where
        NumDevices: ArrayLength<&'static mut dyn Device<I2cError>>
{

    pub fn scan(&'static mut self) {
        // キー・イベントの収拾
        let mut event_buffer = EventBuffer::new();
        for d in self.devices.iter_mut() {
            let device = d.deref_mut();
            let result = device.read_device(self.i2c);
            match result {
                Ok(state) => {
                    match state {
                        Pins16(pins) => {
                            device.add_event(&pins, &mut event_buffer)
                        }
                        Pins8(pins) => {
                            device.add_event(&pins, &mut event_buffer)
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
        // キー・イベントの処理




    }
}
