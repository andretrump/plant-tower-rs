pub mod hardware {
    mod digital_output;
    pub use digital_output::DigitalOutput;

    mod pump;
    pub use pump::Pump;
}

pub mod mqtt {
    mod switch;
    pub use switch::Switch;
}
