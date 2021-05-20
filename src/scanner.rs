// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::Device;
use crate::i2c::I2C;
use heapless::Vec;
use heapless::consts::U8;

/// deviceを使用して、キーの状態をスキャンするもの
pub struct Scanner<'a, E> {
    i2c: &'a dyn I2C<E>,
    devices: Vec<&'a mut dyn Device<'a, E>, U8>
}

impl<'a, E> Scanner<'a, E> {

    pub fn scan() {

    }
}
