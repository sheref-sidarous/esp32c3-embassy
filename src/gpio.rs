
use esp32c3_hal::{
    gpio::{Gpio9, IO},
    gpio_types::{Event, Input, Pin, PullDown},
};

pub struct EmbGPIO { 
    hal_gpio : Gpio9<Input<PullDown>>
}

impl EmbGPIO {

    pub fn new<T>(gpio : Gpio9<T>) -> Self { 
        EmbGPIO {
            hal_gpio: gpio.into_pull_down_input(),
        }
    }

    pub async fn toggled(&self) -> () {
        
    }
}



