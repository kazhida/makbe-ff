use keyberon::key_code::{KeyCode, KbHidReport};
use usb_device::device::UsbDevice;
use usbd_serial::SerialPort;
use makbe_ff::reporter::Reporter;
use xiao_m0::UsbBus;


pub struct UsbReporter {
    usb_bus: &'static mut UsbDevice<'static, UsbBus>,
    usb_serial: &'static mut SerialPort<'static, UsbBus>
}

impl Reporter for UsbReporter {

    fn send_codes(&self, codes: &[KeyCode]) {
        self.usb_bus.poll(&mut [self.serial]);
        let report: KbHidReport = codes.collect();
        loop {
            if let Some(count) = self.usb_serial.write(report.as_bytes()) {
                if count == 0_usize {
                    break;
                }
            }
        }
    }
}
