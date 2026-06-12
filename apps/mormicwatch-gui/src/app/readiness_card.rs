use eframe::egui::{
    self,
    RichText,
};

use crate::app::theme;

use mormicwatch_core::models::{
    readiness_report::ReadinessReport,
    recording_quality::RecordingQuality,
};

pub fn draw_readiness_card(
    ui: &mut egui::Ui,
    quality: RecordingQuality,
    readiness: &ReadinessReport,
) {
    let quality_color = match quality {
        RecordingQuality::Excellent =>
            theme::GOOD,

        RecordingQuality::Good =>
            theme::GOOD,

        RecordingQuality::Marginal =>
            theme::WARNING,

        RecordingQuality::TooQuiet =>
            theme::WARNING,

        RecordingQuality::Clipping =>
            theme::DANGER,

        RecordingQuality::NoSignal =>
            theme::DANGER,
    };

    ui.group(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(
                    if readiness.ready {
                        "RECORDING READY"
                    } else {
                        "NOT READY"
                    },
                )
                .size(28.0)
                .color(
                    if readiness.ready {
                        theme::GOOD
                    } else {
                        theme::DANGER
                    },
                ),
            );

            ui.add_space(5.0);

            ui.label(
                RichText::new(
                    format!(
                        "QUALITY: {}",
                        quality.label()
                    ),
                )
                .size(22.0)
                .color(quality_color),
            );

            ui.add_space(5.0);

            ui.label(
                RichText::new(
                    format!(
                        "SCORE: {}%",
                        readiness.score
                    ),
                )
                .size(20.0)
                .color(theme::TEXT),
            );

            if !readiness
                .recommendations
                .is_empty()
            {
                ui.separator();

                ui.label(
                    RichText::new(
                        "RECOMMENDATIONS",
                    )
                    .color(theme::TEXT),
                );

                for recommendation in
                    &readiness.recommendations
                {
                    ui.label(
                        format!(
                            "• {}",
                            recommendation
                        ),
                    );
                }
            }
        });
    });
}
