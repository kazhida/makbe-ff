// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use crate::event::{Event, EventBuffer};
use crate::switch::Switch;
use crate::event::Event::{Pressed, Released};
use heapless::{Vec, ArrayLength};
use core::marker::PhantomData;

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

pub struct Debouncer<'a, NumPins>
    where
        NumPins: ArrayLength<bool> + ArrayLength<Event<'a>> + core::cmp::PartialEq
{
    cur: Keys<NumPins>,
    new: Keys<NumPins>,
    count: u16,
    limit: u16,
    phantom: &'a PhantomData<*const ()>
}

impl <'a, NumPins> Debouncer<'a, NumPins>
    where
        NumPins: ArrayLength<bool> + ArrayLength<Event<'a>> + core::cmp::PartialEq
{
    pub fn new(limit: u16) -> Self {
        Self {
            cur: Keys::default(),
            new: Keys::default(),
            count: 0,
            limit,
            phantom: &PhantomData::default()
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

    pub fn add_events<F>(&mut self, new: &[bool], events: &mut EventBuffer, switch: F)
        where
            F: Fn(usize)->&'a Switch
    {
        if self.update(&Keys::from(new)) {
            let zipped = self.new.pressed.iter().zip(self.cur.pressed.iter());
            let mapped = zipped.enumerate().map(
                move | (i, (o, n)) | {
                    match (o, n) {
                        (false, true) => Some(Pressed(switch(i))),
                        (true, false) => Some(Released(switch(i))),
                        _ => None
                    }
                }
            );
            let filtered = mapped.filter(|o| o.is_some());
            let unwrapped = filtered.map(|f| f.unwrap());
            for (i, e) in unwrapped.enumerate() {
                events.buffer.push(e.clone());
            }
        };
    }
}

