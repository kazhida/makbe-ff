// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//


use keyberon::action::Action;
use keyberon::action::Action::*;
use keyberon::key_code::KeyCode;
use crate::event::KeyEvent;
use crate::event::KeyEvent::{Released, Pressed};
use crate::switch::Switch;
use heapless::Vec;
use heapless::consts::U64;
use arraydeque::{ArrayDeque, Wrapping};

use KeyState::*;
use crate::reporter::Reporter;
use keyberon::key_code;

pub struct Evaluator {
    default_layer: usize,
    states: Vec<KeyState, U64>,
    waiting: Option<WaitingState>,
    stacked: ArrayDeque<[Stacked; 16], Wrapping>,
    reporter: &'static dyn Reporter
}

impl Evaluator {

    pub fn new(reporter: &'static dyn Reporter) -> Self {
        Self {
            default_layer: 0,
            states: Vec::new(),
            waiting: None,
            stacked: ArrayDeque::new(),
            reporter
        }
    }

    pub fn eval(&mut self, event: KeyEvent)  {
        if let Some(stacked) = self.stacked.push_back(event.into()) {
            self.waiting_into_hold();
            self.unstack(stacked);
        }
        if self
            .waiting
            .as_ref()
            .map(|w| w.is_corresponding_release(&event))
            .unwrap_or(false)
        {
            self.waiting_into_tap();
        }
        self.reporter.send_codes(&self.keycodes()[..]);
    }

    pub fn tick(&mut self) {
        self.states = self.states.iter().filter_map(KeyState::tick).collect();
        self.stacked.iter_mut().for_each(Stacked::tick);
        match &mut self.waiting {
            Some(w) => {
                if w.tick() {
                    self.waiting_into_hold();
                }
            }
            None => {
                if let Some(s) = self.stacked.pop_front() {
                    self.unstack(s);
                }
            }
        }
        self.reporter.send_codes(&self.keycodes()[..]);
    }

    fn keycodes(&self) -> Vec<KeyCode, U64> {
        let mut codes: Vec<KeyCode, U64> = Vec::new();
        for kc in self.states.iter().filter_map(KeyState::keycode) {
            codes.push(kc);
        }
        codes
    }

    fn waiting_into_hold(&mut self) {
        if let Some(w) = &self.waiting {
            let hold = w.hold;
            let switch = w.switch;
            self.waiting = None;
            self.do_action(hold, switch, 0);
        }
    }

    fn waiting_into_tap(&mut self) {
        if let Some(w) = &self.waiting {
            let tap = w.tap;
            let switch = w.switch;
            self.waiting = None;
            self.do_action(tap, switch, 0);
        }
    }

    fn unstack(&mut self, stacked: Stacked) {
        match stacked.event {
            Released(switch) => {
                self.states = self
                    .states
                    .iter()
                    .filter_map(|s| s.release(switch))
                    .collect()
            }
            Pressed(switch) => {
                let action = self.press_as_action(switch, self.current_layer());
                self.do_action(action, switch, stacked.since);
            }
        }
    }

    fn press_as_action(&self, switch: &'static Switch, layer: usize) -> &'static Action {
        let action = switch.action_at(layer);
        match action {
            None => &NoOp,
            Some(Trans) => {
                if layer != self.default_layer {
                    self.press_as_action(switch, self.default_layer)
                } else {
                    &NoOp
                }
            }
            Some(a) => a
        }
    }

    fn do_action(&mut self, action: &Action, switch: &'static Switch, delay: u16) {
        assert!(self.waiting.is_none());
        use Action::*;
        match *action {
            NoOp | Trans => (),
            HoldTap { timeout, hold, tap } => {
                let waiting = WaitingState {
                    switch,
                    timeout: timeout.saturating_sub(delay),
                    hold,
                    tap,
                };
                self.waiting = Some(waiting);
                if let Some(Stacked { since, .. }) = self
                    .stacked
                    .iter()
                    .find(|s| waiting.is_corresponding_release(&s.event))
                {
                    if timeout >= delay - since {
                        self.waiting_into_tap();
                    } else {
                        self.waiting_into_hold();
                    }
                }
            }
            KeyCode(keycode) => {
                let _ = self.states.push(NormalKey { switch, keycode });
            }
            MultipleKeyCodes(v) => {
                for &keycode in v {
                    let _ = self.states.push(NormalKey { switch, keycode });
                }
            }
            MultipleActions(v) => {
                for action in v {
                    self.do_action(action, switch, delay);
                }
            }
            Layer(value) => {
                let _ = self.states.push(LayerModifier { value, switch });
            }
            DefaultLayer(value) => {
                self.default_layer = value
            }
            _ => {}
        }
    }

    fn current_layer(&self) -> usize {
        let mut iter = self.states.iter().filter_map(KeyState::get_layer);
        let mut layer = match iter.next() {
            None => self.default_layer,
            Some(l) => l,
        };
        for l in iter {
            layer += l;
        }
        layer
    }
}

#[derive(Debug, Clone, Copy)]
enum KeyState {
    NormalKey { keycode: KeyCode, switch: &'static Switch },
    LayerModifier { value: usize, switch: &'static Switch },
}

impl KeyState {

    fn keycode(&self) -> Option<KeyCode> {
        match self {
            NormalKey { keycode, .. } => Some(*keycode),
            _ => None,
        }
    }

    fn tick(&self) -> Option<Self> {
        match *self {
            _ => Some(*self),
        }
    }

    fn release(&self, s: &Switch) -> Option<Self> {
        match *self {
            NormalKey { switch, .. } | LayerModifier { switch, .. } if switch == s => None,
            _ => Some(*self),
        }
    }

    fn get_layer(&self) -> Option<usize> {
        match self {
            LayerModifier { value, .. } => Some(*value),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct WaitingState  {
    switch: &'static Switch,
    timeout: u16,
    hold: &'static Action,
    tap: &'static Action,
}

impl WaitingState {

    fn tick(&mut self) -> bool {
        self.timeout = self.timeout.saturating_sub(1);
        self.timeout == 0
    }

    fn is_corresponding_release(&self, event: &KeyEvent) -> bool {
        match event {
            KeyEvent::Released(switch) if *switch == self.switch => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Stacked {
    event: KeyEvent,
    since: u16,
}

impl From<KeyEvent> for Stacked {
    fn from(event: KeyEvent) -> Self {
        Stacked { event, since: 0 }
    }
}

impl Stacked {
    fn tick(&mut self) {
        self.since = self.since.saturating_add(1);
    }
}
