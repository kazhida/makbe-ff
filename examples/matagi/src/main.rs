#![no_main]
#![no_std]

extern crate panic_halt;
extern crate usb_device;
extern crate usbd_serial;
extern crate xiao_m0 as hal;

use hal::entry;

use usb_device::bus::UsbBusAllocator;
use usbd_serial::SerialPort;
use hal::hal::samd21::usb::UsbBus;

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

#[entry]
fn main() -> ! {
    loop {
        // todo
    }
}
