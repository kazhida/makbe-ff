#![no_main]
#![no_std]

mod layout;

const VENDOR_ID:  u16 = 0xFEED;
const PRODUCT_ID: u16 = 0x0000;
const DEVICE_VER: &str = "0x0001";
const MANUFACTURER: &str = "ABplus Inc. kazhida";
const PRODUCT: &str = "Matagi(xiao)";

extern crate panic_halt;
extern crate xiao_m0;

use xiao_m0::{entry, i2c_master};
use xiao_m0::pac::{NVIC, interrupt, Peripherals, CorePeripherals};
use xiao_m0::clock::GenericClockController;
use xiao_m0::hal::common::time::KiloHertz;
use xiao_m0::hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write(&[c.clone()]).unwrap();
                        // LED.as_mut().map(|led| led.toggle());
                    }
                };
            });
        });
    };
}



// #[interrupt]
// fn USB() {
//     poll_usb();
// }

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = xiao_m0::Pins::new(peripherals.PORT);

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(xiao_m0::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
            &mut pins.port
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(VENDOR_ID, PRODUCT_ID))
                .manufacturer(MANUFACTURER)
                .product(PRODUCT)
                .serial_number(DEVICE_VER)
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    let mut i2c = i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM2,
        &mut peripherals.PM,
        pins.a4,
        pins.a5,
        &mut pins.port
    );




    loop {

        // todo
    }
}
