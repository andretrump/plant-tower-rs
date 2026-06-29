use esp_idf_svc::mqtt::client::EspMqttClient;
use esp_idf_svc::mqtt::client::QoS;
use json::{object, JsonValue};
use std::collections::HashMap;

pub struct Device {
    mqtt_config: MqttConfig,
    discovery_topic: String,
    manufacturer: String,
    components: HashMap<String, Box<dyn Component>>,
}

impl Device {
    pub fn new(unique_id: String, name: String, manufacturer: String) -> Self {
        let platform = String::from("device");
        let discovery_topic = format!("homeassistant/{}/{}/config", platform, unique_id);
        let mqtt_config = MqttConfig::new(unique_id, name, platform);
        Self {
            mqtt_config,
            discovery_topic,
            manufacturer,
            components: HashMap::new(),
        }
    }

    pub fn register_component(&mut self, component: Box<dyn Component>) {
        self.components
            .insert(component.state_topic().clone(), component);
    }

    pub fn send_discovery_message(&mut self, mqtt_client: &mut EspMqttClient) {
        let payload = self.build_discovery_payload();
        log::info!(
            "Sending discovery message to topic {} with payload\n{}",
            self.discovery_topic,
            json::stringify_pretty(payload.clone(), 2)
        );
        let payload = json::stringify(payload);
        mqtt_client
            .publish(
                &self.discovery_topic,
                QoS::ExactlyOnce,
                true,
                payload.as_bytes(),
            )
            .expect("Failed to send MQTT discovery message");
    }

    fn build_discovery_payload(&self) -> JsonValue {
        let mut payload = object! {
            state_topic: self.mqtt_config.state_topic.as_str(),
            qos: 2,
            dev: {
                ids: self.mqtt_config.unique_id.as_str(),
                name: self.mqtt_config.name.as_str(),
                mf: self.manufacturer.as_str(),
                mdl: self.mqtt_config.name.as_str()
            },
            o: {
                name: self.mqtt_config.name.as_str()
            },
            cmps: {}
        };
        for component in self.components.values() {
            let component_payload = component.to_discovery_payload();
            payload["cmps"][component.unique_id()] = component_payload;
        }
        payload
    }
}

pub struct MqttConfig {
    unique_id: String,
    name: String,
    platform: String,
    state_topic: String,
}

impl MqttConfig {
    pub fn new(unique_id: String, name: String, platform: String) -> Self {
        let state_topic = format!("homeassistant/{}/{}/state", platform, unique_id);
        Self {
            unique_id,
            name,
            platform,
            state_topic,
        }
    }

    pub fn unique_id(&self) -> &String {
        &self.unique_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn platform(&self) -> &String {
        &self.platform
    }

    pub fn state_topic(&self) -> &String {
        &self.state_topic
    }
}

pub trait Component {
    fn unique_id(&self) -> &String;
    fn state_topic(&self) -> &String;
    fn to_discovery_payload(&self) -> JsonValue;
    fn process_message(&mut self, message: String);
}
