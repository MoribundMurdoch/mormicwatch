use crate::models::recording_quality::RecordingQuality;

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
