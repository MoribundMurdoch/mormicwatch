use std::time::{Duration, Instant};

use crate::models::{
    health_score::HealthScore,
    mic_status::MicStatus,
    recording_quality::RecordingQuality,
};

pub const SPEAKING_THRESHOLD_DB: f32 = -35.0;
pub const CLIPPING_THRESHOLD_DB: f32 = -1.0;
pub const NO_SIGNAL_TIMEOUT_SECS: u64 = 60;

pub fn classify(
    peak_db: f32,
    last_signal_at: Option<Instant>,
) -> MicStatus {
    if peak_db >= CLIPPING_THRESHOLD_DB {
        return MicStatus::Clipping;
    }

    let Some(last_signal_at) = last_signal_at else {
        return MicStatus::NoSignal;
    };

    if last_signal_at.elapsed()
        > Duration::from_secs(
            NO_SIGNAL_TIMEOUT_SECS,
        )
    {
        return MicStatus::NoSignal;
    }

    if peak_db >= SPEAKING_THRESHOLD_DB {
        MicStatus::Healthy
    } else {
        MicStatus::Quiet
    }
}

pub fn classify_recording_quality(
    peak_db: f32,
) -> RecordingQuality {
    if peak_db <= -90.0 {
        return RecordingQuality::NoSignal;
    }

    if peak_db >= -3.0 {
        return RecordingQuality::Clipping;
    }

    if peak_db >= -12.0 {
        return RecordingQuality::Excellent;
    }

    if peak_db >= -18.0 {
        return RecordingQuality::Good;
    }

    if peak_db >= -24.0 {
        return RecordingQuality::Marginal;
    }

    RecordingQuality::TooQuiet
}

pub fn calculate_health(
    peak_db: f32,
    rms_db: f32,
    last_signal_at: Option<Instant>,
    device_connected: bool,
) -> HealthScore {
    let mut score = 100u8;
    let mut reasons = Vec::new();

    let signal_present = match last_signal_at {
        Some(t) => {
            t.elapsed()
                <= Duration::from_secs(
                    NO_SIGNAL_TIMEOUT_SECS,
                )
        }
        None => false,
    };

    let volume_healthy =
        peak_db >= -24.0;

    let clipping =
        peak_db >= CLIPPING_THRESHOLD_DB;

    if !device_connected {
        score = 0;

        reasons.push(
            "Recording device disconnected"
                .into(),
        );
    }

    if !signal_present {
        score = score.saturating_sub(50);

        reasons.push(
            "No microphone activity detected recently"
                .into(),
        );
    }

    if clipping {
        score = score.saturating_sub(40);

        reasons.push(
            "Microphone clipping detected"
                .into(),
        );
    }

    if peak_db < -24.0 {
        score = score.saturating_sub(25);

        reasons.push(
            "Recording level too low"
                .into(),
        );
    }

    if rms_db < -60.0 {
        score = score.saturating_sub(15);

        reasons.push(
            "Noise floor extremely low compared to speech"
                .into(),
        );
    }

    HealthScore {
        score,

        signal_present,
        volume_healthy,

        clipping,
        device_connected,

        reasons,
    }
}