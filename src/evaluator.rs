// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use keyberon::action::Action;

pub trait Evaluator {

    fn eval(pins: &[bool]) -> dyn Iterator<Item = Action>;
}