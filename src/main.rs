use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use plant_tower_rs::hardware::DigitalOutput;

fn main() {
    // Boilerplate code. Do not change!
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("Failed to initialize peripherals");
    let mut led = DigitalOutput::new(peripherals.pins.gpio26);

    log::info!("Hello, world!");

    loop {
        led.switch_on();
        FreeRtos::delay_ms(1000);
        led.switch_off();
        FreeRtos::delay_ms(1000);
    }
}
