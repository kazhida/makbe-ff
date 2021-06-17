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
use xiao_m0::hal::common::sercom::I2CError;
use keyberon::action::{k, l, Action};
use keyberon::action::Action::HoldTap;
use xiao_m0::clock::AcAnaClock;


switch_pool!(
    struct SwitchPool,

    switch Escape = Switch::new(0.0, 0.0),
    switch Kb1 = Switch::new(1.0, 0.0),
    switch Kb2 = Switch::new(2.0, 0.0),
    switch Kb3 = Switch::new(3.0, 0.0),
    switch Kb4 = Switch::new(4.0, 0.0),
    switch Kb5 = Switch::new(5.0, 0.0),
    switch Kb6 = Switch::new(9.0, 0.0),
    switch Kb7 = Switch::new(10.0, 0.0),
    switch Kb8 = Switch::new(11.0, 0.0),
    switch Kb9 = Switch::new(12.0, 0.0),
    switch Kb0 = Switch::new(13.0, 0.0),
    switch Minus = Switch::new(14.0, 0.0),
    switch Equal = Switch::new(15.0, 0.0),
    switch Bslash = Switch::new(16.0, 0.0),
    switch Grave = Switch::new(17.0, 0.0),

    switch Tab = Switch::new_with_width(0.0, 1.0, 1.5),
    switch Q = Switch::new(1.5, 1.0),
    switch W = Switch::new(2.5, 1.0),
    switch E = Switch::new(3.5, 1.0),
    switch R = Switch::new(4.5, 1.0),
    switch T = Switch::new(5.5, 1.0),
    switch Y = Switch::new(9.5, 1.0),
    switch U = Switch::new(10.5, 1.0),
    switch I = Switch::new(11.5, 1.0),
    switch O = Switch::new(12.5, 1.0),
    switch P = Switch::new(13.5, 1.0),
    switch LBracket = Switch::new(14.5, 1.0),
    switch RBracket = Switch::new(15.5, 1.0),
    switch BSpace = Switch::new_with_width(16.5, 1.0, 1.5),

    switch LCtrl = Switch::new_with_width(0.0, 2.0, 1.75),
    switch A = Switch::new(1.75, 2.0),
    switch S = Switch::new(2.75, 2.0),
    switch D = Switch::new(3.75, 2.0),
    switch F = Switch::new(4.75, 2.0),
    switch G = Switch::new(5.75, 2.0),
    switch H = Switch::new(9.75, 2.0),
    switch J = Switch::new(10.75, 2.0),
    switch K = Switch::new(11.75, 2.0),
    switch L = Switch::new(12.75, 2.0),
    switch SColon = Switch::new(13.75, 2.0),
    switch Quote = Switch::new(14.75, 2.0),
    switch Enter = Switch::new_with_width(15.75, 2.0, 2.25),

    switch LShift = Switch::new_with_width(0.0, 3.0, 2.0),
    switch Z = Switch::new(2.0, 3.0),
    switch X = Switch::new(3.0, 3.0),
    switch C = Switch::new(4.0, 3.0),
    switch V = Switch::new(5.0, 3.0),
    switch B = Switch::new(6.0, 3.0),
    switch N = Switch::new(10.0, 3.0),
    switch M = Switch::new(11.0, 3.0),
    switch Comma = Switch::new(12.0, 3.0),
    switch Dot = Switch::new(13.0, 3.0),
    switch Slash = Switch::new(14.0, 3.0),
    switch RShift = Switch::new(15.0, 3.0),
    switch Up = Switch::new(16.0, 3.0),
    switch Delete = Switch::new(17.0, 3.0),

    switch CapsLock = Switch::new_with_width(0.0, 4.0, 1.75),
    switch LOpt = Switch::new_with_width(1.75, 4.0, 1.25),
    switch LCmd = Switch::new(3.0, 4.0),
    switch Lower = Switch::new_with_width(4.0, 4.0, 1.25),
    switch Space = Switch::new_with_width(5.25, 4.0, 6.25),
    switch Raise = Switch::new_with_width(11.5, 4.0, 1.25),
    switch RAlt = Switch::new_with_width(12.75, 4.0, 1.25),
    switch App = Switch::new(14.0, 4.0),
    switch Left = Switch::new(15.0, 4.0),
    switch Down = Switch::new(16.0, 4.0),
    switch Right = Switch::new(17.0, 4.0),
);


pub struct Layout {
    switches: SwitchPool,
    pub device0: TCA9555<'_, I2CError>,
    pub device1: TCA9555<'_, I2CError>,
    pub device2: TCA9555<'_, I2CError>,
    pub device3: TCA9555<'_, I2CError>,
    pub device4: TCA9555<'_, I2CError>
}

const LOWER: usize = 1;
const RAISE: usize = 2;
const FUNCS: usize = 3;


const LOWER_EISU: Action = HoldTap {
    timeout: 200,
    hold: &l(LOWER),
    tap: &k(Lang2),
};

const SHIFT_KANA: Action = HoldTap {
    timeout: 200,
    hold: &k(RShift),
    tap: &k(Lang1),
};

const FUNCS_TAB: Action = HoldTap {
    timeout: 200,
    hold: &l(FUNCS),
    tap: &k(Tab),
};

impl Layout {

    pub fn new() -> Self {
        Self {
            switches: SwitchPool::new(),
            device0: Self::dev0(),
            device1: Self::dev1(),
            device2: Self::dev2(),
            device3: Self::dev3(),
            device4: Self::dev4(),
        }
    }

    fn apply<F>(mut switch: Switch, mut f: F) -> Switch
        where
            F: FnMut(&mut Switch) -> &mut Switch
    {
        f(&mut switch);
        switch
    }

    fn dev0() -> TCA9555<I2CError> {
        let mut device = TCA9555::new(0x0, 200);

        device.assign(0, Self::apply(SW_Escape, |s| s
            .append_action(k(Escape))
        ));
        device.assign(1, Self::apply(SW_Kb1, |s| s.append_action(k(Kb1))));
        device.assign(2, Self::apply(SW_Kb2, |s| s.append_action(k(Kb2))));
        device.assign(3, Self::apply(SW_Kb3, |s| s.append_action(k(Kb3))));
        device.assign(4, Self::apply(SW_Kb4, |s| s.append_action(k(Kb4))));
        device.assign(5, Self::apply(SW_Kb5, |s| s.append_action(k(Kb5))));
        device.assign(6, Self::apply(SW_Kb6, |s| s.append_action(k(Kb6))));
        device.assign(7, Self::apply(SW_Kb7, |s| s.append_action(k(Kb7))));
        device.assign(8, Self::apply(SW_Kb8, |s| s.append_action(k(Kb8))));
        device.assign(9, Self::apply(SW_Kb9, |s| s.append_action(k(Kb9))));
        device.assign(10, Self::apply(SW_Kb0, |s| s.append_action(k(Kb0))));
        device.assign(11, Self::apply(SW_Minus, |s| s.append_action(k(Minus))));
        device.assign(12, Self::apply(SW_Equal, |s| s.append_action(k(Equal))));
        device.assign(13, Self::apply(SW_Bslash, |s| s.append_action(k(Bslash))));
        device.assign(14, Self::apply(SW_Grave, |s| s.append_action(k(Grave))));

        device
    }

    fn dev1() -> TCA9555<I2CError> {
        let mut device = TCA9555::new(0x1, 200);

        device.assign(0, Self::apply(SW_Tab, |s| s.append_action(FUNCS_TAB)));
        device.assign(1, Self::apply(SW_Q, |s| s.append_action(k(Q))));
        device.assign(2, Self::apply(SW_W, |s| s.append_action(k(W))));
        device.assign(3, Self::apply(SW_E, |s| s.append_action(k(E))));
        device.assign(4, Self::apply(SW_R, |s| s.append_action(k(R))));
        device.assign(5, Self::apply(SW_T, |s| s.append_action(k(T))));

        device.assign(8, Self::apply(SW_LCtrl, |s| s.append_action(k(LCtrl))));
        device.assign(9, Self::apply(SW_A, |s| s.append_action(k(A))));
        device.assign(10, Self::apply(SW_S, |s| s.append_action(k(S))));
        device.assign(11, Self::apply(SW_D, |s| s.append_action(k(D))));
        device.assign(12, Self::apply(SW_F, |s| s.append_action(k(F))));
        device.assign(13, Self::apply(SW_G, |s| s.append_action(k(G))));

        device
    }

    fn dev2() -> TCA9555<I2CError> {
        let mut device = TCA9555::new(0x2, 200);

        device.assign(0, Self::apply(SW_Y, |s| s.append_action(k(Y))));
        device.assign(1, Self::apply(SW_U, |s| s.append_action(k(U))));
        device.assign(2, Self::apply(SW_I, |s| s.append_action(k(I))));
        device.assign(3, Self::apply(SW_O, |s| s.append_action(k(O))));
        device.assign(4, Self::apply(SW_P, |s| s.append_action(k(P))));
        device.assign(5, Self::apply(SW_LBracket, |s| s.append_action(k(LBracket))));
        device.assign(6, Self::apply(SW_RBracket, |s| s.append_action(k(RBracket))));
        device.assign(7, Self::apply(SW_BSpace, |s| s.append_action(k(BSpace))));

        device.assign(8, Self::apply(SW_H, |s| s.append_action(k(H))));
        device.assign(9, Self::apply(SW_J, |s| s.append_action(k(J))));
        device.assign(10, Self::apply(SW_K, |s| s.append_action(k(K))));
        device.assign(11, Self::apply(SW_L, |s| s.append_action(k(L))));
        device.assign(12, Self::apply(SW_SColon, |s| s.append_action(k(SColon))));
        device.assign(13, Self::apply(SW_Quote, |s| s.append_action(k(Quote))));
        device.assign(14, Self::apply(SW_Enter, |s| s.append_action(k(Enter))));

        device
    }

    fn dev3() -> TCA9555<I2CError> {
        let mut device = TCA9555::new(0x3, 200);

        device.assign(0, Self::apply(SW_LShift, |s| s.append_action(k(LShift))));
        device.assign(1, Self::apply(SW_Z, |s| s.append_action(k(Z))));
        device.assign(2, Self::apply(SW_X, |s| s.append_action(k(X))));
        device.assign(3, Self::apply(SW_C, |s| s.append_action(k(C))));
        device.assign(4, Self::apply(SW_V, |s| s.append_action(k(V))));
        device.assign(5, Self::apply(SW_B, |s| s.append_action(k(B))));

        device.assign(8, Self::apply(SW_CapsLock, |s| s.append_action(k(CapsLock))));
        device.assign(9, Self::apply(SW_LOpt, |s| s.append_action(k(LAlt))));
        device.assign(10, Self::apply(SW_LCmd, |s| s.append_action(k(LGui))));
        device.assign(11, Self::apply(SW_Lower, |s| s.append_action(LOWER_EISU)));
        device.assign(12, Self::apply(SW_Space, |s| s.append_action(k(Space))));
        device.assign(13, Self::apply(SW_Raise, |s| s.append_action(SHIFT_KANA)));

        device
    }

    fn dev4() -> TCA9555<I2CError> {
        let mut device = TCA9555::new(0x4, 200);

        device.assign(0, Self::apply(SW_N, |s| s.append_action(k(N))));
        device.assign(1, Self::apply(SW_M, |s| s.append_action(k(M))));
        device.assign(2, Self::apply(SW_Comma, |s| s.append_action(k(Comma))));
        device.assign(3, Self::apply(SW_Dot, |s| s.append_action(k(Dot))));
        device.assign(4, Self::apply(SW_Slash, |s| s.append_action(k(Slash))));
        device.assign(5, Self::apply(SW_RShift, |s| s.append_action(k(RShift))));
        device.assign(6, Self::apply(SW_Up, |s| s.append_action(k(Up))));
        device.assign(7, Self::apply(SW_Delete, |s| s.append_action(k(Delete))));

        device.assign(8, Self::apply(SW_RAlt, |s| s.append_action(l(RAISE))));
        device.assign(9, Self::apply(SW_App, |s| s.append_action(l(RAISE))));
        device.assign(10, Self::apply(SW_Left, |s| s.append_action(k(Left))));
        device.assign(11, Self::apply(SW_Down, |s| s.append_action(k(Down))));
        device.assign(12, Self::apply(SW_Right, |s| s.append_action(k(Right))));

        device
    }
}
