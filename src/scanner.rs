// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use heapless::{Vec, ArrayLength};
use crate::device::Device;
use crate::i2c::I2C;
use crate::device::DeviceState::{Pins16, Pins8};
use crate::evaluator::Evaluator;
use core::ops::DerefMut;

/// deviceを使用して、キーの状態をスキャンするもの
pub struct Scanner<'a, I2cError>
{
    i2c: &'a mut dyn I2C<I2cError>,
    evaluator: Evaluator<'a>
}

impl <'a, I2cError> Scanner<'a, I2cError>
{
    /// キー・イベントの収拾
    pub fn scan<NumDevices>(&mut self, devices: &Vec<&'a mut dyn Device<I2cError>, NumDevices>)
        where
            NumDevices: ArrayLength<&'a mut dyn Device<I2cError>>
    {
        // デバイス毎にイベント取得
        for d in devices.iter() {
            let device = d.deref_mut();
            let result = device.read_device(self.i2c);
            match result {
                Ok(state) => {
                    match state {
                        // 16ビットのI/Oエクスパンダ
                        Pins16(pins) => {
                            for e in device.pick_events(&pins).buffer {
                                self.evaluator.eval(e.clone());
                            }
                        }
                        // 8ビットのI/Oエクスパンダ
                        Pins8(pins) => {
                            for e in device.pick_events(&pins).buffer {
                                self.evaluator.eval(e.clone());
                            }
                        }
                        // その他のデバイス
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
