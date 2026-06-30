use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use plant_tower_rs::mqtt::{self, Component};
use plant_tower_rs::wifi;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::time::{Duration, Instant};

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

enum Components {
    PumpSwitch,
}

impl fmt::Display for Components {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Components::PumpSwitch => write!(f, "plant_tower_rs_pump_switch"),
        }
    }
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().expect("Failed to initialize peripherals");
    let sys_loop = EspSystemEventLoop::take().expect("Failed to initialize system loop");
    let nvs = EspDefaultNvsPartition::take().expect("Failed to initialize NVS");
    let app_config = CONFIG;

    let _wifi = wifi::setup(
        peripherals.modem,
        sys_loop.clone(),
        nvs.clone(),
        app_config.wifi_ssid,
        app_config.wifi_password,
    )
    .expect("Failed to setup Wifi");

    let (mut mqtt_client, receiver) = mqtt::setup(
        app_config.mqtt_user,
        app_config.mqtt_password,
        app_config.mqtt_host,
    )
    .expect("Failed to setup MQTT");

    let mut plant_tower = mqtt::Device::new(
        String::from("plant_tower_rs"),
        String::from("Plant Tower Rust"),
        String::from("Myself"),
    );
    let pump_switch = Rc::new(RefCell::new(mqtt::Switch::new(
        Components::PumpSwitch.to_string(),
        String::from("Pump"),
        HashMap::from([(String::from("icon"), String::from("mdi:pump"))]),
    )));
    plant_tower.register_component(Rc::clone(&pump_switch) as Rc<RefCell<dyn Component>>);
    plant_tower.send_discovery_message(&mut mqtt_client);
    plant_tower.subscribe_command_topics(&mut mqtt_client);

    pump_switch
        .borrow_mut()
        .switch_on(&mut mqtt_client)
        .unwrap_or_else(|err| log::warn!("Failed to switch on pump: {}", err));
    let mut last_switched = Instant::now();

    loop {
        if let Ok((topic, payload)) = receiver.try_recv() {
            plant_tower.dispatch_event(&mut mqtt_client, topic.as_str(), payload.as_str());
        }
        if last_switched.elapsed() >= Duration::from_secs(10) {
            pump_switch
                .borrow_mut()
                .toggle(&mut mqtt_client)
                .unwrap_or_else(|err| log::warn!("Failed to toggle pump switch: {}", err));
            last_switched = Instant::now();
        }
        FreeRtos::delay_ms(10);
    }
}
