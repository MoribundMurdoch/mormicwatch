use super::app_state::AppState;

#[derive(Debug, Clone)]
pub struct StatusSnapshot {
    pub ready: bool,
    pub quality: String,
    pub score: u8,

    pub current_db: f32,
    pub peak_db: f32,
    pub rms_db: f32,

    pub device_name: String,

    pub recommendations: Vec<String>,

    pub target_zone_ok: bool,
    pub signal_present: bool,
    pub speaking_now: bool,
}

impl StatusSnapshot {
    pub fn from_state(
        state: &AppState,
    ) -> Self {
        let target_zone_ok =
            state.metrics.peak_db >= -12.0
                && state.metrics.peak_db <= -6.0;

        Self {
            ready: state.readiness.ready,

            quality: state
                .recording_quality
                .label()
                .to_string(),

            score: state.readiness.score,

            current_db: state.metrics.current_db,

            peak_db: state.metrics.peak_db,

            rms_db: state.metrics.rms_db,

            device_name: state.device_name.clone(),

            recommendations: state
                .health
                .reasons
                .clone(),

            target_zone_ok,

            signal_present: state
                .health
                .signal_present,

            speaking_now: state
                .metrics
                .speaking_now,
        }
    }
}