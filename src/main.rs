use std::collections::HashMap;

use embedded_svc::mqtt::client::{Details::Complete, EventPayload::Error, EventPayload::Received};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    mqtt::client::MqttClientConfiguration,
    mqtt::client::{Details, EspMqttClient},
};
use plant_tower_rs::mqtt;
use plant_tower_rs::wifi;
use plant_tower_rs::{hardware::DigitalOutput, mqtt::Component};

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    mqtt_host: &'static str,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    mqtt_password: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_password: &'static str,
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("Failed to initialize peripherals");
    let sys_loop = EspSystemEventLoop::take().expect("Failed to initialize system loop");

    let app_config = CONFIG;
    let _wifi = wifi::setup(
        peripherals.modem,
        sys_loop.clone(),
        app_config.wifi_ssid,
        app_config.wifi_password,
    )
    .expect("Failed to setup Wifi");

    let broker_url = format!(
        "mqtt://{}:{}@{}",
        app_config.mqtt_user, app_config.mqtt_password, app_config.mqtt_host
    );
    let mut mqtt_client = EspMqttClient::new_cb(
        &broker_url,
        &MqttClientConfiguration::default(),
        move |event| match event.payload() {
            Received { data, details, .. } => process_event(data, details),
            Error(e) => log::warn!("Received error from MQTT: {:?}", e),
            _ => log::info!("Received from MQTT: {:?}", event.payload()),
        },
    )
    .expect("Failed to connect MQTT");

    let mut plant_tower = mqtt::Device::new(
        String::from("plant_tower_rs"),
        String::from("Plant Tower Rust"),
        String::from("Myself"),
    );

    let switch: Box<dyn Component> = Box::new(mqtt::Switch::new(
        String::from("plant_tower_rs_pump_switch"),
        String::from("Pump"),
        HashMap::from([(String::from("icon"), String::from("mdi:pump"))]),
    ));
    plant_tower.register_component(switch);

    plant_tower.send_discovery_message(&mut mqtt_client);

    let mut led = DigitalOutput::new(peripherals.pins.gpio26);

    loop {
        led.switch_on();
        FreeRtos::delay_ms(1000);
        led.switch_off();
        FreeRtos::delay_ms(1000);
    }
}

fn process_event(data: &[u8], details: Details) {
    match details {
        Complete => {
            let payload_string = str::from_utf8(data).expect("Failed to convert payload to string");
            log::info!("{:?}", payload_string);
        }
        _ => {}
    }
}
