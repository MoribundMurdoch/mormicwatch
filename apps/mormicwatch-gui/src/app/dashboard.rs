use std::sync::{Arc, Mutex};

use eframe::egui::{
    self,
    Color32,
    RichText,
};

use mormicwatch_core::models::{
    app_state::AppState,
    mic_status::MicStatus,
};

pub struct Dashboard {
    pub state: Arc<Mutex<AppState>>,
}

impl Dashboard {
    pub fn new(
        state: Arc<Mutex<AppState>>,
    ) -> Self {
        Self { state }
    }

    fn status_color(
        status: MicStatus,
    ) -> Color32 {
        match status {
            MicStatus::Healthy =>
                Color32::GREEN,

            MicStatus::Quiet =>
                Color32::YELLOW,

            MicStatus::Clipping =>
                Color32::from_rgb(
                    255,
                    140,
                    0,
                ),

            MicStatus::NoSignal =>
                Color32::RED,

            MicStatus::Disconnected =>
                Color32::DARK_RED,
        }
    }
}

impl eframe::App for Dashboard {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _: &mut eframe::Frame,
    ) {
        let state =
            self.state.lock().unwrap().clone();

        let status_color =
            Self::status_color(
                state.status,
            );

        egui::CentralPanel::default()
            .show(ctx, |ui| {

                ui.vertical_centered(
                    |ui| {

                    ui.heading(
                        "MorMicWatch",
                    );

                    ui.add_space(10.0);

                    ui.label(
                        RichText::new(
                            state.status
                                .label(),
                        )
                        .size(42.0)
                        .color(
                            status_color,
                        ),
                    );

                    ui.label(
                        RichText::new(
                            if state
                                .metrics
                                .speaking_now
                            {
                                "SPEAKING"
                            } else {
                                "SILENT"
                            },
                        )
                        .size(28.0),
                    );

                    ui.add_space(10.0);

                    ui.label(
                        RichText::new(
                            format!(
                                "{}%",
                                state
                                    .health
                                    .score
                            ),
                        )
                        .size(24.0),
                    );

                    ui.separator();

                    ui.label(
                        format!(
                            "Device: {}",
                            state
                                .device_name
                        ),
                    );

                    ui.add_space(10.0);

                    ui.label(
                        format!(
                            "Current: {:.1} dB",
                            state
                                .metrics
                                .current_db
                        ),
                    );

                    ui.label(
                        format!(
                            "Peak: {:.1} dB",
                            state
                                .metrics
                                .peak_db
                        ),
                    );

                    ui.label(
                        format!(
                            "Peak Hold: {:.1} dB",
                            state
                                .metrics
                                .peak_hold_db
                        ),
                    );

                    if let Some(
                        last_signal,
                    ) = state
                        .metrics
                        .last_signal_at
                    {
                        ui.label(
                            format!(
                                "Last Signal: {} sec ago",
                                last_signal
                                    .elapsed()
                                    .as_secs()
                            ),
                        );
                    }

                    if !state
                        .health
                        .reasons
                        .is_empty()
                    {
                        ui.add_space(
                            10.0,
                        );

                        ui.separator();

                        for reason in
                            &state
                                .health
                                .reasons
                        {
                            ui.colored_label(
                                Color32::RED,
                                reason,
                            );
                        }
                    }
                });
            });

        ctx.request_repaint();
    }
}