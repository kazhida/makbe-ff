// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

extern crate xiao_m0 as hal;
extern crate paste;

use makbe_ff::key_switch::KeySwitch;
use makbe_ff::switch_pool;
use makbe_ff::device::{Device, DeviceHolder};
use makbe_ff::devices::tca9555::TCA9555;
use keyberon::key_code::KeyCode::*;
use keyberon::action::{k, l, Action};
use keyberon::action::Action::HoldTap;
use xiao_m0::sercom::{I2CError, I2CMaster2, Sercom2Pad0, Sercom2Pad1};
use xiao_m0::gpio::{Pa8, Pa9, PfD};


const BASE: usize = 0;
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

switch_pool!(
    struct SwitchPool,

    switch escape = KeySwitch::new(0.0, 0.0).apply(|s| s.append_action(k(Escape))),
    switch kb1 = KeySwitch::new(1.0, 0.0).apply(|s| s.append_action(k(Kb1))),
    switch kb2 = KeySwitch::new(2.0, 0.0).apply(|s| s.append_action(k(Kb2))),
    switch kb3 = KeySwitch::new(3.0, 0.0).apply(|s| s.append_action(k(Kb3))),
    switch kb4 = KeySwitch::new(4.0, 0.0).apply(|s| s.append_action(k(Kb4))),
    switch kb5 = KeySwitch::new(5.0, 0.0).apply(|s| s.append_action(k(Kb5))),
    switch kb6 = KeySwitch::new(9.0, 0.0).apply(|s| s.append_action(k(Kb6))),
    switch kb7 = KeySwitch::new(10.0, 0.0).apply(|s| s.append_action(k(Kb7))),
    switch kb8 = KeySwitch::new(11.0, 0.0).apply(|s| s.append_action(k(Kb8))),
    switch kb9 = KeySwitch::new(12.0, 0.0).apply(|s| s.append_action(k(Kb9))),
    switch kb0 = KeySwitch::new(13.0, 0.0).apply(|s| s.append_action(k(Kb0))),
    switch minus = KeySwitch::new(14.0, 0.0).apply(|s| s.append_action(k(Minus))),
    switch equal = KeySwitch::new(15.0, 0.0).apply(|s| s.append_action(k(Equal))),
    switch b_slash = KeySwitch::new(16.0, 0.0).apply(|s| s.append_action(k(Bslash))),
    switch grave = KeySwitch::new(17.0, 0.0).apply(|s| s.append_action(k(Grave))),

    switch tab = KeySwitch::new_with_width(0.0, 1.0, 1.5).apply(|s| s.append_action(FUNCS_TAB)),
    switch q = KeySwitch::new(1.5, 1.0).apply(|s| s.append_action(k(Q))),
    switch w = KeySwitch::new(2.5, 1.0).apply(|s| s.append_action(k(W))),
    switch e = KeySwitch::new(3.5, 1.0).apply(|s| s.append_action(k(E))),
    switch r = KeySwitch::new(4.5, 1.0).apply(|s| s.append_action(k(R))),
    switch t = KeySwitch::new(5.5, 1.0).apply(|s| s.append_action(k(T))),
    switch y = KeySwitch::new(9.5, 1.0).apply(|s| s.append_action(k(Y))),
    switch u = KeySwitch::new(10.5, 1.0).apply(|s| s.append_action(k(U))),
    switch i = KeySwitch::new(11.5, 1.0).apply(|s| s.append_action(k(I))),
    switch o = KeySwitch::new(12.5, 1.0).apply(|s| s.append_action(k(O))),
    switch p = KeySwitch::new(13.5, 1.0).apply(|s| s.append_action(k(P))),
    switch l_bracket = KeySwitch::new(14.5, 1.0).apply(|s| s.append_action(k(LBracket))),
    switch r_bracket = KeySwitch::new(15.5, 1.0).apply(|s| s.append_action(k(RBracket))),
    switch b_space = KeySwitch::new_with_width(16.5, 1.0, 1.5).apply(|s| s.append_action(k(BSpace))),

    switch l_ctrl = KeySwitch::new_with_width(0.0, 2.0, 1.75).apply(|s| s.append_action(k(LCtrl))),
    switch a = KeySwitch::new(1.75, 2.0).apply(|s| s.append_action(k(A))),
    switch s = KeySwitch::new(2.75, 2.0).apply(|s| s.append_action(k(S))),
    switch d = KeySwitch::new(3.75, 2.0).apply(|s| s.append_action(k(D))),
    switch f = KeySwitch::new(4.75, 2.0).apply(|s| s.append_action(k(F))),
    switch g = KeySwitch::new(5.75, 2.0).apply(|s| s.append_action(k(G))),
    switch h = KeySwitch::new(9.75, 2.0).apply(|s| s.append_action(k(H))),
    switch j = KeySwitch::new(10.75, 2.0).apply(|s| s.append_action(k(J))),
    switch k = KeySwitch::new(11.75, 2.0).apply(|s| s.append_action(k(K))),
    switch l = KeySwitch::new(12.75, 2.0).apply(|s| s.append_action(k(L))),
    switch s_colon = KeySwitch::new(13.75, 2.0).apply(|s| s.append_action(k(SColon))),
    switch quote = KeySwitch::new(14.75, 2.0).apply(|s| s.append_action(k(Quote))),
    switch enter = KeySwitch::new_with_width(15.75, 2.0, 2.25).apply(|s| s.append_action(k(Enter))),

    switch l_shift = KeySwitch::new_with_width(0.0, 3.0, 2.0).apply(|s| s.append_action(k(LCtrl))),
    switch z = KeySwitch::new(2.0, 3.0).apply(|s| s.append_action(k(Z))),
    switch x = KeySwitch::new(3.0, 3.0).apply(|s| s.append_action(k(X))),
    switch c = KeySwitch::new(4.0, 3.0).apply(|s| s.append_action(k(C))),
    switch v = KeySwitch::new(5.0, 3.0).apply(|s| s.append_action(k(V))),
    switch b = KeySwitch::new(6.0, 3.0).apply(|s| s.append_action(k(B))),
    switch n = KeySwitch::new(10.0, 3.0).apply(|s| s.append_action(k(N))),
    switch m = KeySwitch::new(11.0, 3.0).apply(|s| s.append_action(k(M))),
    switch comma = KeySwitch::new(12.0, 3.0).apply(|s| s.append_action(k(Comma))),
    switch dot = KeySwitch::new(13.0, 3.0).apply(|s| s.append_action(k(Dot))),
    switch slash = KeySwitch::new(14.0, 3.0).apply(|s| s.append_action(k(Slash))),
    switch r_shift = KeySwitch::new(15.0, 3.0).apply(|s| s.append_action(k(RShift))),
    switch up = KeySwitch::new(16.0, 3.0).apply(|s| s.append_action(k(Up))),
    switch delete = KeySwitch::new(17.0, 3.0).apply(|s| s.append_action(k(Delete))),

    switch caps_lock = KeySwitch::new_with_width(0.0, 4.0, 1.75).apply(|s| s.append_action(k(CapsLock))),
    switch l_opt = KeySwitch::new_with_width(1.75, 4.0, 1.25).apply(|s| s.append_action(k(LAlt))),
    switch l_cmd = KeySwitch::new(3.0, 4.0).apply(|s| s.append_action(k(LGui))),
    switch lower = KeySwitch::new_with_width(4.0, 4.0, 1.25).apply(|s| s.append_action(LOWER_EISU)),
    switch space = KeySwitch::new_with_width(5.25, 4.0, 6.25).apply(|s| s.append_action(k(Space))),
    switch raise = KeySwitch::new_with_width(11.5, 4.0, 1.25).apply(|s| s.append_action(SHIFT_KANA)),
    switch r_alt = KeySwitch::new_with_width(12.75, 4.0, 1.25).apply(|s| s.append_action(k(RAlt))),
    switch app = KeySwitch::new(14.0, 4.0).apply(|s| s.append_action(k(RGui))),
    switch left = KeySwitch::new(15.0, 4.0).apply(|s| s.append_action(k(Left))),
    switch down = KeySwitch::new(16.0, 4.0).apply(|s| s.append_action(k(Down))),
    switch right = KeySwitch::new(17.0, 4.0).apply(|s| s.append_action(k(Right))),
);

type I2CMaster = I2CMaster2<Sercom2Pad0<Pa8<PfD>>, Sercom2Pad1<Pa9<PfD>>>;

static mut SWITCH_POOL: Option<SwitchPool> = None;
static mut DEVICE0: Option<TCA9555<I2CMaster, I2CError>> = None;
static mut DEVICE1: Option<TCA9555<I2CMaster, I2CError>> = None;
static mut DEVICE2: Option<TCA9555<I2CMaster, I2CError>> = None;
static mut DEVICE3: Option<TCA9555<I2CMaster, I2CError>> = None;
static mut DEVICE4: Option<TCA9555<I2CMaster, I2CError>> = None;


pub struct Layout {
    pub device0: &'static TCA9555<I2CMaster, I2CError>,
    pub device1: &'static TCA9555<I2CMaster, I2CError>,
    pub device2: &'static TCA9555<I2CMaster, I2CError>,
    pub device3: &'static TCA9555<I2CMaster, I2CError>,
    pub device4: &'static TCA9555<I2CMaster, I2CError>
}

static mut LAYOUT: Option<Layout> = None;

impl Layout {

    pub fn new() -> Self {
        unsafe {
            SWITCH_POOL = Some(SwitchPool::new());
            let switches = SWITCH_POOL.as_ref().unwrap();
            DEVICE0 = Some(Self::dev0(switches));
            DEVICE1 = Some(Self::dev1(switches));
            DEVICE2 = Some(Self::dev2(switches));
            DEVICE3 = Some(Self::dev3(switches));
            DEVICE4 = Some(Self::dev4(switches));
            let layout = Self {
                device0: DEVICE0.as_ref().unwrap(),
                device1: DEVICE1.as_ref().unwrap(),
                device2: DEVICE2.as_ref().unwrap(),
                device3: DEVICE3.as_ref().unwrap(),
                device4: DEVICE4.as_ref().unwrap()
            };
            layout
        }
    }

    fn apply<F>(mut switch: KeySwitch, mut f: F) -> KeySwitch
        where
            F: FnMut(&mut KeySwitch) -> &mut KeySwitch
    {
        f(&mut switch);
        switch
    }

    unsafe fn dev0(switches: &'static SwitchPool) -> TCA9555<I2CMaster, I2CError> {
        let mut device = TCA9555::new(0x0, 200);

        device.assign(0, &switches.escape);
        device.assign(1, &switches.kb1);
        device.assign(2, &switches.kb2);
        device.assign(3, &switches.kb3);
        device.assign(4, &switches.kb4);
        device.assign(5, &switches.kb5);
        device.assign(6, &switches.kb6);
        device.assign(7, &switches.kb7);
        device.assign(8, &switches.kb8);
        device.assign(9, &switches.kb9);
        device.assign(10, &switches.kb0);
        device.assign(11, &switches.minus);
        device.assign(12, &switches.equal);
        device.assign(13, &switches.b_slash);
        device.assign(14, &switches.grave);

        device
    }

    unsafe fn dev1(switches: &'static SwitchPool) -> TCA9555<I2CMaster, I2CError> {
        let mut device = TCA9555::new(0x1, 200);

        device.assign(0, &switches.tab);
        device.assign(1, &switches.q);
        device.assign(2, &switches.w);
        device.assign(3, &switches.e);
        device.assign(4, &switches.r);
        device.assign(5, &switches.t);

        device.assign(8, &switches.l_ctrl);
        device.assign(9, &switches.a);
        device.assign(10, &switches.s);
        device.assign(11, &switches.d);
        device.assign(12, &switches.f);
        device.assign(13, &switches.g);

        device
    }

    unsafe fn dev2(switches: &'static SwitchPool) -> TCA9555<I2CMaster, I2CError> {
        let mut device = TCA9555::new(0x2, 200);

        device.assign(0, &switches.y);
        device.assign(1, &switches.u);
        device.assign(2, &switches.i);
        device.assign(3, &switches.o);
        device.assign(4, &switches.p);
        device.assign(5, &switches.l_bracket);
        device.assign(6, &switches.r_bracket);
        device.assign(7, &switches.b_space);

        device.assign(8, &switches.h);
        device.assign(9, &switches.j);
        device.assign(10, &switches.k);
        device.assign(11, &switches.l);
        device.assign(12, &switches.s_colon);
        device.assign(13, &switches.quote);
        device.assign(14, &switches.enter);

        device
    }

    unsafe fn dev3(switches: &'static SwitchPool) -> TCA9555<I2CMaster, I2CError> {
        let mut device = TCA9555::new(0x3, 200);

        device.assign(0, &switches.l_shift);
        device.assign(1, &switches.z);
        device.assign(2, &switches.x);
        device.assign(3, &switches.c);
        device.assign(4, &switches.v);
        device.assign(5, &switches.b);

        device.assign(8, &switches.caps_lock);
        device.assign(9, &switches.l_opt);
        device.assign(10, &switches.l_cmd);
        device.assign(11, &switches.lower);
        device.assign(12, &switches.space);
        device.assign(13, &switches.raise);

        device
    }

    unsafe fn dev4(switches: &'static SwitchPool) -> TCA9555<I2CMaster, I2CError> {
        let mut device = TCA9555::new(0x4, 200);

        device.assign(0, &switches.n);
        device.assign(1, &switches.m);
        device.assign(2, &switches.comma);
        device.assign(3, &switches.dot);
        device.assign(4, &switches.slash);
        device.assign(5, &switches.r_shift);
        device.assign(6, &switches.up);
        device.assign(7, &switches.delete);

        device.assign(8, &switches.r_alt);
        device.assign(9, &switches.app);
        device.assign(10, &switches.left);
        device.assign(11, &switches.down);
        device.assign(12, &switches.right);

        device
    }

    pub fn init_devices(&mut self, i2c: &mut I2CMaster2<Sercom2Pad0<Pa8<PfD>>, Sercom2Pad1<Pa9<PfD>>>) {
        self.device0.init_device(i2c);
        self.device1.init_device(i2c);
        self.device2.init_device(i2c);
        self.device3.init_device(i2c);
        self.device4.init_device(i2c);
    }

    pub fn device_holder(&self) -> DeviceHolder<I2CMaster, I2CError> {
        let mut holder = DeviceHolder::new();

        unsafe {
            holder.devices.push(DEVICE0.as_ref().unwrap());
            holder.devices.push(DEVICE1.as_ref().unwrap());
            holder.devices.push(DEVICE2.as_ref().unwrap());
            holder.devices.push(DEVICE3.as_ref().unwrap());
            holder.devices.push(DEVICE4.as_ref().unwrap());
        }

        holder
    }
}
