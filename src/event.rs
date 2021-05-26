// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::switch::Switch;
use heapless::Vec;
use heapless::consts::U64;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Pressed(&'static Switch),
    Released(&'static Switch)
}

pub struct EventBuffer {
    pub buffer: Vec<Event, U64>
}

impl EventBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }
}

