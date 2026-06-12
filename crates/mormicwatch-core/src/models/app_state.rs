use super::{
    audio_metrics::AudioMetrics,
    health_score::HealthScore,
    mic_status::MicStatus,
    readiness_report::ReadinessReport,
    recording_quality::RecordingQuality,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub device_name: String,

    pub metrics: AudioMetrics,

    pub health: HealthScore,

    pub status: MicStatus,

    pub recording_quality: RecordingQuality,

    pub readiness: ReadinessReport,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            device_name: "Unknown".to_string(),

            metrics: AudioMetrics::default(),

            health: HealthScore::default(),

            status: MicStatus::NoSignal,

            recording_quality:
                RecordingQuality::NoSignal,

            readiness:
                ReadinessReport::default(),
        }
    }
}