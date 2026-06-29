#[toml_cfg::toml_config]
pub struct Config {
    #[default("localhost")]
    mqtt_host: &'static str,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    mqtt_pass: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() {
    if !std::path::Path::new("cfg.toml").exists() {
        panic!("Config file `cfg.toml` not found!");
    }
    embuild::espidf::sysenv::output();
}
