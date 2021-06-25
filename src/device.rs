// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::switch::Switch;
use crate::event::EventBuffer;
use heapless::Vec;
use heapless::consts::U128;
use embedded_hal::blocking::i2c::{Write, WriteRead};

/// デバイスが返す状態
///
/// キースイッチだけでなく、ロータリーエンコーダーも使えるようにしたいので、Stateというenumでラップした。
/// Value8とかValue16とか抽象化が全然できてなくてダサいけど、Stateでくくることが主目的
#[derive(Debug, Clone)]
pub enum DeviceState {
    /// 普通のキースイッチ（16bit）
    Pins16([bool; 16]),
    /// 普通のキースイッチ（16bit）
    Pins8([bool; 8]),
    /// ロータリーエンコーダ(0-0xFF)
    Value8(u8),
    /// ロータリーエンコーダ(0-0xFFFF)
    Value16(u16),
    /// ロータリーエンコーダ(0-0xFFFFFFFF)
    Value32(u32)
}

/// デバイスの機能
pub trait Device<I2C, E>
    where
        I2C: Write<Error = E>,
        I2C: WriteRead<Error = E>
{
    /// # デバイスの初期化
    ///
    /// I/Oエクスパンダ上のピンの設定とか
    fn init_device(&self, i2c: &mut I2C) -> Result<(), E>;

    /// # 読込
    ///
    /// 返値はそのデバイスの状態
    fn read_device(&self, i2c: &mut I2C) -> Result<DeviceState, E>;


    /// # キーの割付
    fn assign(&mut self, pin: usize, switch: &'static Switch) -> Result<usize, usize>;

    /// # キーが割り付けられているか
    fn has_assigned(&self) -> bool;

    /// # イベントの検出
    fn pick_events(&self, pins: &[bool]) -> EventBuffer;
}

pub struct DeviceHolder<I2C: 'static, E: 'static> {
    pub devices: Vec<&'static dyn Device<I2C, E>, U128>
}

impl<I2C, E: 'static> DeviceHolder<I2C, E> {

    pub fn new() -> Self {
        Self {
            devices: Vec::new()
        }
    }
}
