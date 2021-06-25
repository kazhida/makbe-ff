// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use embedded_hal::blocking::i2c::{Write, WriteRead};

/// moduloにおけるI2Cマスターに必要な機能をまとめたもの
pub trait I2C<E>: Write<Error = E> + WriteRead<Error = E> {}
