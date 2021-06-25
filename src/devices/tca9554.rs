// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::switch::Switch;
use crate::debouncer::{Debouncer};
use crate::device::{Device, DeviceState};
use crate::event::EventBuffer;
use heapless::Vec;
use heapless::consts::U8;
use core::cell::RefCell;
use crate::device::DeviceState::Pins8;
use crate::event::IndexEvent::{PressedAt, ReleasedAt};
use crate::event::KeyEvent::{Pressed, Released};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Write, WriteRead};

/// TCA9554
/// PCA9554も同じ
pub struct TCA9554<I2C, E> {
    dev_addr: u8,
    debouncer: RefCell<Debouncer<U8>>,
    switches: Vec<&'static Switch, U8>,
    phantom0: PhantomData<I2C>,
    phantom1: PhantomData<E>
}

impl<I2C, E> TCA9554<I2C, E> {

    pub fn new(addr: u8, debounce: u16) -> Self {
        Self {
            dev_addr: 0x20u8 + addr,
            debouncer: RefCell::new(Debouncer::new(debounce)),
            switches: Vec::default(),
            phantom0: Default::default(),
            phantom1: Default::default()
        }
    }
}

/// I2Cの実装がMCU（チップセット）毎にバラバラなので、エラーの型をジェネリクスのパラメータで渡す形になってしまう
impl<I2C, E> Device<I2C, E> for TCA9554<I2C, E>
    where
        I2C: Write<Error = E>,
        I2C: WriteRead<Error = E>
{


    fn init_device(&self, i2c: &mut I2C) -> Result<(), E> {
        // All input
        i2c.write(self.dev_addr, &[0x06_u8, 0xFF_u8])?;
        i2c.write(self.dev_addr, &[0x07_u8, 0xFF_u8])
    }

    fn read_device(&self, i2c: &mut I2C) -> Result<DeviceState, E> {
        let reg_addr = &[0x00_u8];
        let data = &mut [0x00_u8];
        i2c.write_read(self.dev_addr, reg_addr, data)?;

        let mut pressed = [false;  8];
        let mut mask = 0x01_u8;
        for j in 0..8 {
            pressed[j] = mask & data[0] == 0;    // スイッチが押されていたらLowなので0
            mask <<= 1;
        }
        Ok(Pins8(pressed))
    }

    fn assign(&mut self, pin: usize, switch: &'static Switch) -> Result<usize, usize> {
        if pin < 8 {
            self.switches[pin] = switch;
            Ok(pin)
        } else {
            Err(pin)
        }
    }

    fn has_assigned(&self) -> bool {
        self.switches.iter().any(|s| s.actions.len() > 0)
    }

    fn pick_events(&self, pins: &[bool]) -> EventBuffer {
        let mut event_buffer = EventBuffer::new();
        let indexes = self.debouncer.borrow_mut().events(pins);
        for idx in indexes.buffer {
            let event = match idx {
                PressedAt(i) => Pressed(&self.switches.get(i).unwrap()),
                ReleasedAt(i) => Released(&self.switches.get(i).unwrap())
            };
            let _ = event_buffer.buffer.push(event);
        }
        event_buffer
    }
}
