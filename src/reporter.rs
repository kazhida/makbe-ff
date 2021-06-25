// Copyright 2021 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use keyberon::key_code::KeyCode;


pub trait Reporter {
    fn send_codes(&mut self, codes: &[KeyCode]);
}