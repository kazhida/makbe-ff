// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use generic_array::{GenericArray, ArrayLength};
use crate::event::Event;
use crate::switch::Switch;
use crate::event::Event::{Pressed, Released};
use either::Either::{Left, Right};

#[derive(Default, PartialEq, Eq)]
pub struct Keys<NumPins>
    where
        NumPins: ArrayLength<bool> + core::cmp::PartialEq
{
    pub pressed: GenericArray<bool, NumPins>
}

pub struct Debouncer<NumPins>
    where
        NumPins: ArrayLength<bool> + core::cmp::PartialEq
{
    cur: Keys<NumPins>,
    new: Keys<NumPins>,
    count: u16,
    limit: u16
}

impl <NumPins> Debouncer<NumPins>
    where
        NumPins: ArrayLength<bool> + core::cmp::PartialEq
{

    pub fn new(limit: u16) -> Self {
        Self {
            cur: Keys::default(),
            new: Keys::default(),
            count: 0,
            limit
        }
    }

    pub fn update(&mut self, new: Keys<NumPins>) ->bool {
        if self.cur == new {
            self.count = 0;
            false
        } else {
            if self.new == new {
                self.count += 1;
            } else {
                self.new = new;
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

    pub fn events<F: 'static>(&mut self, new: Keys<NumPins>, switch: F) -> impl Iterator<Item = Option<Event>> + '_
        where
            F: Fn(usize)->Switch,
    {
        if self.update(new) {
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
            Left(mapped)
        } else {
            Right(core::iter::empty())
        }
    }
}

