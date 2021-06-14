// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::switch::Switch;
use heapless::Vec;
use heapless::consts::U64;

#[derive(Debug, Clone, Copy)]
pub enum KeyEvent {
    Pressed(&'static Switch),
    Released(&'static Switch)
}

pub struct EventBuffer {
    pub buffer: Vec<KeyEvent, U64>
}

impl EventBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IndexEvent {
    PressedAt(usize),
    ReleasedAt(usize)
}

pub struct IndexEvents {
    pub buffer: Vec<IndexEvent, U64>
}

impl IndexEvents {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }
}
