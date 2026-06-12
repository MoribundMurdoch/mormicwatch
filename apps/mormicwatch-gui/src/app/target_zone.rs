use eframe::egui::{
    self,
    RichText,
};

use crate::app::theme;

pub fn draw_target_zone(
    ui: &mut egui::Ui,
    peak_db: f32,
) {
    let status = if peak_db >= -12.0
        && peak_db <= -6.0
    {
        "IDEAL"
    } else if peak_db >= -18.0 {
        "GOOD"
    } else if peak_db >= -24.0 {
        "LOW"
    } else {
        "TOO QUIET"
    };

    let color = match status {
        "IDEAL" => theme::GOOD,
        "GOOD" => theme::GOOD,
        "LOW" => theme::WARNING,
        _ => theme::DANGER,
    };

    ui.group(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(
                    "TARGET ZONE",
                )
                .size(24.0)
                .color(theme::TEXT),
            );

            ui.label(
                RichText::new(
                    status,
                )
                .size(30.0)
                .color(color),
            );

            ui.label(
                format!(
                    "Current Peak: {:.1} dB",
                    peak_db,
                ),
            );

            ui.label(
                "Ideal Range: -12 dB to -6 dB",
            );
        });
    });
}
