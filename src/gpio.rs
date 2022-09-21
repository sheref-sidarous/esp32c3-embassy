
use esp32c3_hal::{
    gpio::{Gpio9, IO},
    gpio_types::{Event, Input, PullDown},
};

use core::future::Future;
use core::task::{Context, Poll, Waker};
use core::pin::Pin;

pub struct EmbGPIO<F> {
    hal_gpio : Gpio9<Input<PullDown>>,
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

impl Future for ToggledFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    
            //Poll::Pending

            Poll::Ready(())

    }
}

impl EmbGPIO<ToggledFuture> {

    pub fn new<T>(gpio : Gpio9<T>) -> Self { 
        EmbGPIO {
            hal_gpio: gpio.into_pull_down_input(),            
            toggled_future : ToggledFuture::new(),
        }
    }

    pub fn toggled(&self) -> impl Future<Output = () > {
        self.toggled_future
    }
}



