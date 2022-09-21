//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO5)

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp32c3_hal::{
    clock::ClockControl,
    gpio::IO,
//    pac::Peripherals,
    pac::{self, Peripherals},
    prelude::*,
    system::SystemExt,
    timer::TimerGroup,
    Delay,
    Rtc,
    systimer::{Alarm, Periodic, SystemTimer, Target},
    interrupt,
    interrupt::Priority,
};
use core::cell::{RefCell, RefMut, Ref};
use embassy_time::{Duration, Timer};
use embassy_time::driver::{AlarmHandle, Driver};

use embassy_executor::Spawner;

use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    

    let syst = SystemTimer::new(peripherals.SYSTIMER);
    let alarm0 = syst.alarm0;

    unsafe {
        esp_alam.replace(Some(alarm0));
    }


    // Set GPIO5 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio5.into_push_pull_output();

    led.set_high().unwrap();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    //let mut delay = Delay::new(&clocks);

    loop {
        led.toggle().unwrap();
        Timer::after(Duration::from_millis(1000)).await;
        //delay.delay_ms(500u32);
    }
}

struct EmbassyTimeDriver {
    alarm_allocatd : bool,
}

static mut esp_alam: RefCell<Option<Alarm<Target, 0>>> = RefCell::new(None);
//static mut esp_systimer : RefCell<Option<SystemTimer>> = RefCell::new(None);
static mut callback_fn : RefCell<Option<fn(*mut ())>> = RefCell::new(None);
static mut callback_context : *mut () = core::ptr::null_mut();


impl Driver for EmbassyTimeDriver {

    fn now(&self) -> u64 {
        SystemTimer::now()
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
/*
        if self.alarm_allocatd {
            None
        } else {
            self.alarm_allocatd = true;
            Some(AlarmHandle::new(0))
        }
        */
        Some(AlarmHandle::new(0))
    }

    fn set_alarm_callback(&self, _alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {

        unsafe {
        callback_fn.replace(Some(callback));
        callback_context = ctx;
        }
    }

    fn set_alarm(&self, alarm: AlarmHandle, timestamp: u64) {
        unsafe {
            esp_alam.borrow().as_ref().unwrap().set_target(timestamp);
            esp_alam.borrow().as_ref().unwrap().enable_interrupt();
            interrupt::enable(pac::Interrupt::SYSTIMER_TARGET0, Priority::Priority1).unwrap();
        }
    }
}


embassy_time::time_driver_impl!(static DRIVER: EmbassyTimeDriver = EmbassyTimeDriver {
    alarm_allocatd: false
});

#[interrupt]
unsafe fn SYSTIMER_TARGET0() {

    let x = callback_fn.borrow();
    let callback = x.as_ref().unwrap();
    //let cntxt = callback_context.borrow().as_ref().unwrap();

    callback(callback_context);

    let a : Ref<Option<Alarm<Target, 0>>> = esp_alam.borrow();
    let b : Option<&Alarm<Target, 0>> = a.as_ref();
    let c : &Alarm<Target, 0> = b.unwrap();
    c.clear_interrupt();
}