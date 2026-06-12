use crate::config::settings::Settings;

pub fn load_settings() -> Settings {
    Settings::default()
}

pub fn save_settings(_settings: &Settings) {}