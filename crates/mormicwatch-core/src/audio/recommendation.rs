use crate::models::recording_quality::RecordingQuality;

use super::quality::classify_recording_quality;

#[derive(Debug, Clone)]
pub struct AudioRecommendation {
    pub title: String,
    pub description: String,
    pub ffmpeg_fix: Option<String>,
}

pub fn build_recommendation(
    peak_db: f32,
) -> Option<AudioRecommendation> {
    match classify_recording_quality(
        peak_db,
    ) {
        RecordingQuality::TooQuiet => {
            Some(AudioRecommendation {
                title:
                    "Input Too Quiet"
                        .into(),

                description:
                    "Increase microphone gain or move closer to the microphone."
                        .into(),

                ffmpeg_fix: Some(
                    "-af \"volume=+8dB\""
                        .into(),
                ),
            })
        }

        RecordingQuality::Clipping => {
            Some(AudioRecommendation {
                title:
                    "Clipping Detected"
                        .into(),

                description:
                    "Reduce microphone gain."
                        .into(),

                ffmpeg_fix: Some(
                    "-af \"volume=-4dB\""
                        .into(),
                ),
            })
        }

        _ => None,
    }
}