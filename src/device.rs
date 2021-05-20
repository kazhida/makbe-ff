// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::i2c::I2C;
use core::cell::RefCell;
use crate::switch::Switch;

/// デバイスが返す状態
///
/// キースイッチだけでなく、ロータリーエンコーダーも使えるようにしたいので、Stateというenumでラップした。
/// V8とかV16とか抽象化が全然できてなくてダサいけど、Stateでくくることが主目的
pub enum State<'a> {
    /// 普通のキースイッチ
    Pins(&'a[bool]),
    /// ロータリーエンコーダ(0-0xFF)
    V8(&'a[u8]),
    /// ロータリーエンコーダ(0-0xFFFF)
    V16(&'a[u16]),
    /// ロータリーエンコーダ(0-0xFFFFFFFF)
    V32(&'a[u32])
}

/// デバイスの機能
pub trait Device<'a, I2cError> {

    /// # デバイスの初期化
    ///
    /// I/Oエクスパンダ上のピンの設定とか
    fn init_device(&self, i2cm: &RefCell<dyn I2C<I2cError>>) -> Result<(), I2cError>;

    /// # 読込
    ///
    /// 返値はそのデバイスの状態
    fn read_device(&mut self, i2cm: &RefCell<dyn I2C<I2cError>>) -> Result<State, I2cError>;

    /// # キーの割付
    fn assign(&mut self, pin: usize, switch: Switch) -> Result<Switch, Switch>;
}
