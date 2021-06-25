#![no_main]
#![no_std]

mod layout;
mod usb_reporter;

const VENDOR_ID:  u16 = 0xFEED;
const PRODUCT_ID: u16 = 0x0000;
const SERIAL_NUMBER: &str = "0x0001";
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
use makbe_ff::evaluator::Evaluator;
use crate::usb_reporter::UsbReporter;
use makbe_ff::scanner::Scanner;
use crate::layout::Layout;

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

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

    let usb_serial = unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_SERIAL.as_mut().unwrap()
    };

    let usb_bus = unsafe {
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(VENDOR_ID, PRODUCT_ID))
                .manufacturer(MANUFACTURER)
                .product(PRODUCT)
                .serial_number(SERIAL_NUMBER)
                .device_class(USB_CLASS_CDC)
                .build(),
        );
        USB_BUS.as_mut().unwrap()
    };

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

    let mut layout = Layout::new();

    let reporter = UsbReporter{
        usb_bus,
        usb_serial
    };

    let evaluator = Evaluator::new(&reporter);
    let mut scanner = Scanner::new(evaluator);

    layout.init_devices(&mut i2c);

    let device_holder = layout.device_holder();
    loop {
        scanner.scan(&mut i2c, &device_holder)
    }
}
