// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::event::{KeyEvent, IndexEvents};
use heapless::{Vec, ArrayLength};
use crate::event::IndexEvent::{PressedAt, ReleasedAt};

#[derive(Default, PartialEq, Eq)]
pub struct Keys<NumPins>
    where
        NumPins: ArrayLength<bool> + core::cmp::PartialEq
{
    pub pressed: Vec<bool, NumPins>
}

impl<NumPins> Keys<NumPins>
    where
        NumPins: ArrayLength<bool> + core::cmp::PartialEq
{
    pub fn from(keys: &[bool]) -> Self {
        Self {
            pressed: Vec::from_slice(keys).unwrap()
        }
    }
}

pub struct Debouncer<NumPins>
    where
        NumPins: ArrayLength<bool> + ArrayLength<KeyEvent> + core::cmp::PartialEq
{
    cur: Keys<NumPins>,
    new: Keys<NumPins>,
    count: u16,
    limit: u16
}

impl <NumPins> Debouncer<NumPins>
    where
        NumPins: ArrayLength<bool> + ArrayLength<KeyEvent> + core::cmp::PartialEq
{
    pub fn new(limit: u16) -> Self {
        Self {
            cur: Keys::default(),
            new: Keys::default(),
            count: 0,
            limit
        }
    }

    pub fn update(&mut self, new: &Keys<NumPins>) ->bool {
        if self.cur == *new {
            self.count = 0;
            false
        } else {
            if self.new == *new {
                self.count += 1;
            } else {
                self.new.pressed = new.pressed.clone();
                self.count = 1;
            }
            if self.count <= self.limit {
                false
            } else {
                core::mem::swap(&mut self.cur, &mut self.new);
                self.count = 0;
                true
            }
        }
    }

    pub fn events(&mut self, new: &[bool]) -> IndexEvents {
        let mut result = IndexEvents::new();
        if self.update(&Keys::from(new)) {
            let zipped = self.new.pressed.iter().zip(self.cur.pressed.iter());
            let mapped = zipped.enumerate().map(
                move | (i, (o, n)) | {
                    match (o, n) {
                        (false, true) => Some(PressedAt(i)),
                        (true, false) => Some(ReleasedAt(i)),
                        _ => None
                    }
                }
            );
            let filtered = mapped.filter(|o| o.is_some());
            let unwrapped = filtered.map(|f| f.unwrap());
            for e in unwrapped {
                let _ = result.buffer.push(e);
            }
        }
        result
    }
}

