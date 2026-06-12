use crate::models::{
    readiness_report::ReadinessReport,
    recording_quality::RecordingQuality,
};

pub fn build_readiness_report(
    quality: RecordingQuality,
) -> ReadinessReport {
    let mut report =
        ReadinessReport::default();

    match quality {
        RecordingQuality::Excellent => {
            report.ready = true;
            report.score = 100;
        }

        RecordingQuality::Good => {
            report.ready = true;
            report.score = 90;
        }

        RecordingQuality::Marginal => {
            report.ready = true;
            report.score = 70;

            report
                .recommendations
                .push(
                    "Increase microphone gain slightly"
                        .into(),
                );
        }

        RecordingQuality::TooQuiet => {
            report.ready = false;
            report.score = 40;

            report
                .recommendations
                .push(
                    "Increase microphone gain"
                        .into(),
                );

            report
                .recommendations
                .push(
                    "Move closer to microphone"
                        .into(),
                );
        }

        RecordingQuality::Clipping => {
            report.ready = false;
            report.score = 25;

            report
                .recommendations
                .push(
                    "Reduce microphone gain"
                        .into(),
                );
        }

        RecordingQuality::NoSignal => {
            report.ready = false;
            report.score = 0;

            report
                .recommendations
                .push(
                    "No microphone signal detected"
                        .into(),
                );
        }
    }

    report
}
