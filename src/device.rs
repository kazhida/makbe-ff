// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::i2c::I2C;
use crate::switch::Switch;
use crate::event::EventBuffer;
use heapless::{Vec, ArrayLength};

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
pub trait Device<I2cError>
{
    /// # デバイスの初期化
    ///
    /// I/Oエクスパンダ上のピンの設定とか
    fn init_device(&self, i2c: &mut dyn I2C<I2cError>) -> Result<(), I2cError>;

    /// # 読込
    ///
    /// 返値はそのデバイスの状態
    fn read_device(&self, i2c: &mut dyn I2C<I2cError>) -> Result<DeviceState, I2cError>;

    /// # キーの割付
    fn assign(&mut self, pin: usize, switch: Switch) -> Result<usize, usize>;

    /// # キーが割り付けられているか
    fn has_assigned(&self) -> bool;

    /// # イベントの検出
    fn pick_events(&mut self, pins: &[bool]) -> EventBuffer;
}

pub struct DeviceHolder<'a, NumDevices, I2cError>
    where
        NumDevices: ArrayLength<&'a dyn Device<I2cError>>
{
    devices: Vec<&'a dyn Device<I2cError>, NumDevices>
}
