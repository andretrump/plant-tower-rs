use esp_idf_hal::gpio::OutputPin;

use crate::{hardware::DigitalOutput, interface::Switchable};

pub struct Pump<'a> {
    output: DigitalOutput<'a>,
}

impl<'a> Pump<'a> {
    pub fn new<P: OutputPin + 'a>(pin: P) -> Self {
        let output = DigitalOutput::new(pin);
        Self { output }
    }
}

impl<'a> Switchable for Pump<'a> {
    fn switch_on(&mut self) {
        self.output.switch_on();
    }

    fn switch_off(&mut self) {
        self.output.switch_off();
    }
}
