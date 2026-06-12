#[derive(Debug, Clone)]
pub struct HealthScore {
    pub score: u8,

    pub signal_present: bool,
    pub volume_healthy: bool,
    pub clipping: bool,
    pub device_connected: bool,

    pub reasons: Vec<String>,
}

impl Default for HealthScore {
    fn default() -> Self {
        Self {
            score: 0,

            signal_present: false,
            volume_healthy: false,
            clipping: false,
            device_connected: false,

            reasons: Vec::new(),
        }
    }
}