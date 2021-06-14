// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

extern crate xiao_m0 as hal;
extern crate paste;

use makbe_ff::switch::Switch;
use makbe_ff::switch_pool;
use makbe_ff::device::Device;
use makbe_ff::devices::tca9555::TCA9555;
use keyberon::key_code::KeyCode::*;
use heapless::Vec;
use heapless::consts::U5;
use xiao_m0::hal::common::sercom::I2CError;
use makbe_ff::devices::Devices;


switch_pool!(
    struct SwitchPool,

    sw Escape = Switch::new(0.0, 0.0),
    sw K1 = Switch::new(1.0, 0.0),
    sw K2 = Switch::new(2.0, 0.0),
    sw K3 = Switch::new(3.0, 0.0),
    sw K4 = Switch::new(4.0, 0.0),
    sw K5 = Switch::new(5.0, 0.0),
    sw K6 = Switch::new(9.0, 0.0),
    sw K7 = Switch::new(10.0, 0.0),
    sw K8 = Switch::new(11.0, 0.0),
    sw K9 = Switch::new(12.0, 0.0),
    sw K0 = Switch::new(13.0, 0.0),
    sw Minus = Switch::new(14.0, 0.0),
    sw Equal = Switch::new(15.0, 0.0),
    sw Bslash = Switch::new(16.0, 0.0),
    sw Grave = Switch::new(17.0, 0.0),

    sw Tab = Switch::new_with_width(0.0, 1.0, 1.5),
    sw Q = Switch::new(1.5, 1.0),
    sw W = Switch::new(2.5, 1.0),
    sw E = Switch::new(3.5, 1.0),
    sw R = Switch::new(4.5, 1.0),
    sw T = Switch::new(5.5, 1.0),
    sw Y = Switch::new(9.5, 1.0),
    sw U = Switch::new(10.5, 1.0),
    sw I = Switch::new(11.5, 1.0),
    sw O = Switch::new(12.5, 1.0),
    sw P = Switch::new(13.5, 1.0),
    sw LBracket = Switch::new(14.5, 1.0),
    sw RBracket = Switch::new(15.5, 1.0),
    sw BSpace = Switch::new_with_width(16.5, 1.0, 1.5),

    sw LCtrl = Switch::new_with_width(0.0, 2.0, 1.75),
    sw A = Switch::new(1.75, 2.0),
    sw S = Switch::new(2.75, 2.0),
    sw D = Switch::new(3.75, 2.0),
    sw F = Switch::new(4.75, 2.0),
    sw G = Switch::new(5.75, 2.0),
    sw H = Switch::new(9.75, 2.0),
    sw J = Switch::new(10.75, 2.0),
    sw K = Switch::new(11.75, 2.0),
    sw L = Switch::new(12.75, 2.0),
    sw SColon = Switch::new(13.75, 2.0),
    sw Quote = Switch::new(14.75, 2.0),
    sw Enter = Switch::new_with_width(15.75, 2.0, 2.25),

    sw LShift = Switch::new_with_width(0.0, 3.0, 2.0),
    sw Z = Switch::new(2.0, 3.0),
    sw X = Switch::new(3.0, 3.0),
    sw C = Switch::new(4.0, 3.0),
    sw V = Switch::new(5.0, 3.0),
    sw B = Switch::new(6.0, 3.0),
    sw N = Switch::new(10.0, 3.0),
    sw M = Switch::new(11.0, 3.0),
    sw Comma = Switch::new(12.0, 3.0),
    sw Dot = Switch::new(13.0, 3.0),
    sw Slash = Switch::new(14.0, 3.0),
    sw RShift = Switch::new(15.0, 3.0),
    sw Up = Switch::new(16.0, 3.0),
    sw Delete = Switch::new(17.0, 3.0),

    sw CapsLock = Switch::new_with_width(0.0, 4.0, 1.75),
    sw LOpt = Switch::new_with_width(1.75, 4.0, 1.25),
    sw LCmd = Switch::new(3.0, 4.0),
    sw Lower = Switch::new_with_width(4.0, 4.0, 1.25),
    sw Space = Switch::new_with_width(5.25, 4.0, 6.25),
    sw Raise = Switch::new_with_width(11.5, 4.0, 1.25),
    sw RAlt = Switch::new_with_width(12.75, 4.0, 1.25),
    sw App = Switch::new(14.0, 4.0),
    sw Left = Switch::new(15.0, 4.0),
    sw Down = Switch::new(16.0, 4.0),
    sw Right = Switch::new(17.0, 4.0),
);


// type DeviceArray = &'static[Devices];
//
// static DEVICES: DeviceArray = &[
//     TCA9555(TCA9555::new(0x0, 200)),
//     TCA9555(TCA9555::new(0x1, 200)),
//     TCA9555(TCA9555::new(0x2, 200)),
//     TCA9555(TCA9555::new(0x3, 200)),
//     TCA9555(TCA9555::new(0x4, 200))
// ];
