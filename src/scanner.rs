// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::DeviceHolder;
use crate::i2c::I2C;
use crate::device::DeviceState::{Pins16, Pins8};
use crate::evaluator::Evaluator;
use core::ops::Deref;
use core::marker::PhantomData;

/// deviceを使用して、キーの状態をスキャンするもの
pub struct Scanner<E> {
    evaluator: Evaluator,
    phantom: PhantomData<E>
}

impl <E> Scanner<E>
{

    pub fn new(evaluator: Evaluator<>) -> Self {
        Self {
            evaluator,
            phantom: Default::default()
        }
    }

    /// キー・イベントの収拾
    pub fn scan(&mut self, i2c: &mut dyn I2C<E>, holder: &DeviceHolder<E>) {
        // デバイス毎にイベント取得
        for d in holder.devices.deref() {
            let device = d.deref();
            let result = device.read_device(i2c);
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
                Err(_) => {
                    // どうしよっか？
                }
            }
        }
    }
}
