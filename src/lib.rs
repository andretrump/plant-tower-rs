pub mod hardware {
    mod digital_output;
    pub use digital_output::DigitalOutput;

    mod pump;
    pub use pump::Pump;
}

pub mod mqtt {
    mod device;
    pub use device::Component;
    pub use device::Device;

    mod switch;
    pub use switch::Switch;
}

pub mod wifi;
