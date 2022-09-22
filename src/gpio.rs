
use esp32c3_hal::{
    gpio::{Gpio9, IO},
    gpio_types::{Event, Input, PullDown},
    interrupt,
};
use esp32c3_hal::prelude::interrupt;
use esp32c3_hal::pac::{self};


use core::future::Future;
use core::task::{Context, Poll, Waker};
use core::pin::Pin;
use core::cell::RefCell;

use esp32c3_hal::gpio_types::Pin as _;

use critical_section::Mutex;

pub struct EmbGPIO<F> {
    //hal_gpio : Gpio9<Input<PullDown>>,
    toggled_future : F
}

#[derive(Clone)]
pub struct ToggledFuture {

}

impl ToggledFuture {

    pub fn new() -> Self { 
        ToggledFuture {            
            
        }
    }
}

//impl Unpin for ToggledFuture {}

impl Copy for ToggledFuture {}

static recently_toggled : Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
static BUTTON: Mutex<RefCell<Option<Gpio9<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));

impl Future for ToggledFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        let mut outcome = Poll::Pending;

        critical_section::with(|cs| {
            let mut recently_toggled_ref = recently_toggled.borrow_ref_mut(cs);
            if *recently_toggled_ref == true {
                *recently_toggled_ref = false;
                outcome = Poll::Ready(());
            }
        });

        return outcome;

    }
}

impl EmbGPIO<ToggledFuture> {

    pub fn new<T>(gpio : Gpio9<T>) -> Self {

        let mut button = gpio.into_pull_down_input();
        button.listen(Event::FallingEdge);
    
        critical_section::with(|cs| BUTTON.borrow_ref_mut(cs).replace(button));

        interrupt::enable(pac::Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();

        EmbGPIO {
            toggled_future : ToggledFuture::new(),
        }
    }

    pub fn toggled(&self) -> impl Future<Output = () > {
        self.toggled_future
    }
}



#[interrupt]
fn GPIO() {
    critical_section::with(|cs| {
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();

        *recently_toggled.borrow_ref_mut(cs) = true;
    });

    // TODO: call the waker
}
