// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

pub mod tca9555;
pub mod tca9554;

pub enum Devices {
    TCA9555(tca9555::TCA9555),
    TCA9554(tca9554::TCA9554)
}