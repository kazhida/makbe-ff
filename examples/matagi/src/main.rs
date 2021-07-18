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

use xiao_m0::{entry, i2c_master, uart};
use xiao_m0::pac::{NVIC, interrupt, Peripherals, CorePeripherals};
use xiao_m0::clock::GenericClockController;
use xiao_m0::hal::common::sercom::{I2CMaster2, Sercom2Pad0, Sercom2Pad1, Sercom4Pad0, Sercom4Pad1, UART4};
use xiao_m0::gpio::{PfD, Pa8, Pa9, Pb8, Pb9};
use usb_device::prelude::*;
use makbe_ff::evaluator::Evaluator;
use makbe_ff::scanner::Scanner;
use crate::layout::Layout;
use crate::usb_reporter::UsbReporter;
use keyberon::keyboard::Leds;
use xiao_m0::time::U32Ext;
use xiao_m0::prelude::*;

struct NoLeds {}

impl Leds for NoLeds {}

type UART = UART4<Sercom4Pad1<Pb9<PfD>>, Sercom4Pad0<Pb8<PfD>>, (), ()>;

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

    let mut uart = uart(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.a7,
        pins.a6,
        &mut pins.port
    );

    dprint(&mut uart, "start, world\n");

    let bus_allocator = xiao_m0::usb_allocator(
        peripherals.USB,
        &mut clocks,
        &mut peripherals.PM,
        pins.usb_dm,
        pins.usb_dp,
        &mut pins.port
    );

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    let mut i2c: I2CMaster2<Sercom2Pad0<Pa8<PfD>>, Sercom2Pad1<Pa9<PfD>>> = i2c_master(
        &mut clocks,
        400.khz(),
        peripherals.SERCOM2,
        &mut peripherals.PM,
        pins.a4,
        pins.a5,
        &mut pins.port
    );

    for c in b"start, world\n".iter() {
        nb::block!(uart.write(*c)).unwrap();
    }

    let mut layout = Layout::new();

    let leds = NoLeds{};
    let mut reporter = UsbReporter {
        usb_class: keyberon::new_class(&bus_allocator, leds),
        usb_dev: UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(VENDOR_ID, PRODUCT_ID))
            .manufacturer(MANUFACTURER)
            .product(PRODUCT)
            .serial_number(SERIAL_NUMBER)
            .build()
    };

    let evaluator = Evaluator::new();
    let mut scanner = Scanner::new(evaluator);

    layout.init_devices(&mut i2c);

    let device_holder = layout.device_holder();
    loop {
        scanner.scan(&mut i2c, &device_holder, &mut reporter);
        dprint(&mut uart, "hello, world\n");
    }
}

fn dprint(uart: &mut UART, message: &str) {
    let buffer = message.as_bytes();
    uart.bwrite_all(buffer);
    uart.bflush();
}
