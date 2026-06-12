use crate::models::{
    health_score::HealthScore,
    readiness_report::ReadinessReport,
};

#[derive(Debug, Clone)]
pub struct AudioRecommendation {
    pub title: String,
    pub description: String,
    pub ffmpeg_fix: Option<String>,
}

pub fn generate_recommendations(
    health: &HealthScore,
    readiness: &ReadinessReport,
) -> Vec<AudioRecommendation> {
    let mut out = Vec::new();

    if !health.signal_present {
        out.push(AudioRecommendation {
            title: "No Signal".into(),
            description: "No microphone signal detected.".into(),
            ffmpeg_fix: None,
        });
    }

    if !health.volume_healthy {
        out.push(AudioRecommendation {
            title: "Increase Gain".into(),
            description: "Voice level is below the target range.".into(),
            ffmpeg_fix: Some("-af \"volume=+8dB\"".into()),
        });
    }

    if health.clipping {
        out.push(AudioRecommendation {
            title: "Reduce Gain".into(),
            description: "Audio clipping detected.".into(),
            ffmpeg_fix: Some("-af \"volume=-4dB\"".into()),
        });
    }

    if readiness.ready && out.is_empty() {
        out.push(AudioRecommendation {
            title: "Ready To Record".into(),
            description: "Microphone levels look good.".into(),
            ffmpeg_fix: None,
        });
    }

    out
}
