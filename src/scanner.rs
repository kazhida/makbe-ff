// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::DeviceHolder;
use crate::device::DeviceState::{Pins16, Pins8};
use crate::evaluator::Evaluator;
use core::ops::Deref;
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use crate::reporter::Reporter;

/// deviceを使用して、キーの状態をスキャンするもの
pub struct Scanner<I2C, E> {
    evaluator: Evaluator,
    phantom0: PhantomData<I2C>,
    phantom1: PhantomData<E>
}

impl <I2C, E> Scanner<I2C, E>
    where
        I2C: Write<Error = E>,
        I2C: WriteRead<Error = E>
{

    pub fn new(evaluator: Evaluator<>) -> Self {
        Self {
            evaluator,
            phantom0: Default::default(),
            phantom1: Default::default()
        }
    }

    /// キー・イベントの収拾
    pub fn scan(&mut self, i2c: &mut I2C, holder: &DeviceHolder<I2C, E>, reporter: &mut dyn Reporter) {
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
                                self.evaluator.eval(e.clone(), reporter);
                            }
                        }
                        // 8ビットのI/Oエクスパンダ
                        Pins8(pins) => {
                            for e in device.pick_events(&pins).buffer {
                                self.evaluator.eval(e.clone(), reporter);
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
