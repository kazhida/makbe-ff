// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use keyberon::key_code::KeyCode;

pub trait Reporter {
    fn send_report(&mut self, iter: impl Iterator<Item=KeyCode>);
}
