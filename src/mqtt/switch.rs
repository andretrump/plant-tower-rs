use std::collections::HashMap;

use esp_idf_svc::mqtt::client::EspMqttClient;
use json::object;

use crate::mqtt::{device::MqttConfig, Component};

pub struct Switch {
    mqtt_config: MqttConfig,
    command_topic: String,
    additional_discovery_config: HashMap<String, String>,
    is_on: bool,
}

impl Switch {
    pub fn new(
        unique_id: String,
        name: String,
        additional_discovery_config: HashMap<String, String>,
    ) -> Self {
        let platfrom = String::from("switch");
        let command_topic = format!("homeassistant/{}/{}/set", platfrom, unique_id);
        let mqtt_config = MqttConfig::new(unique_id, name, platfrom);
        Self {
            mqtt_config,
            command_topic,
            additional_discovery_config,
            is_on: false,
        }
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn switch_on(&mut self, _mqtt_client: &mut EspMqttClient) {}
}

impl Component for Switch {
    fn unique_id(&self) -> &String {
        self.mqtt_config.unique_id()
    }

    fn state_topic(&self) -> &String {
        self.mqtt_config.state_topic()
    }

    fn process_message(&mut self, _message: String) {}

    fn to_discovery_payload(&self) -> json::JsonValue {
        let mut message = object! {
            platform: self.mqtt_config.platform().as_str(),
            name: self.mqtt_config.name().as_str(),
            unique_id: self.mqtt_config.unique_id().as_str(),
            state_topic: self.mqtt_config.state_topic().as_str(),
            command_topic: self.command_topic.as_str()
        };
        for (key, value) in &self.additional_discovery_config {
            message[key] = value.as_str().into();
        }
        message
    }
}
