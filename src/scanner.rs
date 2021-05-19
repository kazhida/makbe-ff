// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::Device;

/// deviceを使用して、キーの状態をスキャンするもの
pub trait Scanner<R, C> {

    fn devices(&self) -> &[dyn Device];

    /// 全体のキーをスキャンして、その状態（押されていればtrue）を返す
    /// deviceの意味が変わっちゃったからこれは没
    fn scan(&self) -> &[bool] {
        // let mut offset = 0;
        // let size = self.devices().iter().fold(0: usize, |a, d| a + d.pin_count());
        let size = 0;
        let buffer = Vec::<bool>::with_capacity(size).as_mut_slice();
        // for (i, dev) in self.devices().iter() {
        //     let count = dev.pin_count();
        //     let tail = offset + count;
        //     let pins = &buffer[offset..tail];
        //     dev.read_pins(pins);
        //     offset +=  count;
        // }
        &buffer
    }
}


