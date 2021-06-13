// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::switch::Switch;
use heapless::Vec;
use heapless::consts::U64;

#[derive(Debug, Clone, Copy)]
pub enum Event<'a> {
    Pressed(&'a Switch),
    Released(&'a Switch)
}

pub struct EventBuffer<'a> {
    pub buffer: Vec<Event<'a>, U64>
}

impl EventBuffer<'_> {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }
}

