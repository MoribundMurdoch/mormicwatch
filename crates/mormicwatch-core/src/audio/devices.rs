use cpal::traits::{
    DeviceTrait,
    HostTrait,
};

pub fn default_input_name() -> Option<String> {
    let host = cpal::default_host();

    host.default_input_device()
        .and_then(|device| device.name().ok())
}

pub fn input_device_connected() -> bool {
    let host = cpal::default_host();

    host.default_input_device().is_some()
}

pub fn list_input_devices() -> Vec<String> {
    let host = cpal::default_host();

    let Ok(devices) = host.input_devices() else {
        return Vec::new();
    };

    devices
        .filter_map(|d| d.name().ok())
        .collect()
}