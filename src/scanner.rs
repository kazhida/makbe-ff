// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use heapless::{Vec, ArrayLength};
use core::ops::DerefMut;
use crate::device::Device;
use crate::i2c::I2C;
use crate::event::EventBuffer;
use crate::device::DeviceState::{Pins16, Pins8};
use crate::evaluator::Evaluator;
use crate::reporter::Reporter;

/// deviceを使用して、キーの状態をスキャンするもの
pub struct Scanner<I2cError: 'static, NumDevices>
    where
        NumDevices: ArrayLength<&'static mut dyn Device<I2cError>>
{
    i2c: &'static mut dyn I2C<I2cError>,
    devices: Vec<&'static mut dyn Device<I2cError>, NumDevices>,
    evaluator: Evaluator<'static>
}

impl<I2cError, NumDevices> Scanner<I2cError, NumDevices>
    where
        NumDevices: ArrayLength<&'static mut dyn Device<I2cError>>
{
    pub fn scan(&'static mut self, reporter: &mut dyn Reporter) -> EventBuffer {
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
        EventBuffer
    }
}
