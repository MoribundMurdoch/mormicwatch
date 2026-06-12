use super::{
    audio_metrics::AudioMetrics,
    health_score::HealthScore,
    mic_status::MicStatus,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub device_name: String,

    pub metrics: AudioMetrics,

    pub health: HealthScore,

    pub status: MicStatus,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            device_name: "Unknown".to_string(),

            metrics: AudioMetrics::default(),

            health: HealthScore::default(),

            status: MicStatus::NoSignal,
        }
    }
}