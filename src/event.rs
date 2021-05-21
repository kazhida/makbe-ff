// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::switch::Switch;

#[derive(Debug, Clone)]
pub enum Event {
    Pressed(Switch),
    Released(Switch)
}

