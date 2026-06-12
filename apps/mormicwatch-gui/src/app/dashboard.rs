use std::sync::{Arc, Mutex};

use eframe::egui::{self, RichText};

use crate::app::{
    history_graph::draw_history_graph,
    target_zone::draw_target_zone,
    theme,
};

use mormicwatch_core::models::app_state::AppState;

pub struct Dashboard {
    pub state: Arc<Mutex<AppState>>,
}

impl Dashboard {
    pub fn new(
        state: Arc<Mutex<AppState>>,
    ) -> Self {
        Self { state }
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

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    ui.label(
                        RichText::new(
                            "MORMICWATCH",
                        )
                        .size(36.0)
                        .color(theme::TEXT),
                    );

                    ui.add_space(12.0);

                    let ready_text =
                        if state.readiness.ready {
                            "READY TO RECORD"
                        } else {
                            "NOT READY"
                        };

                    let ready_color =
                        if state.readiness.ready {
                            theme::GOOD
                        } else {
                            theme::DANGER
                        };

                    ui.label(
                        RichText::new(
                            ready_text,
                        )
                        .size(54.0)
                        .color(ready_color),
                    );

                    ui.label(
                        RichText::new(
                            format!(
                                "QUALITY: {}",
                                state
                                    .recording_quality
                                    .label()
                            ),
                        )
                        .size(28.0)
                        .color(ready_color),
                    );

                    ui.label(
                        RichText::new(
                            format!(
                                "CONFIDENCE: {}%",
                                state
                                    .readiness
                                    .score
                            ),
                        )
                        .size(24.0)
                        .color(theme::TEXT),
                    );

                    ui.add_space(12.0);

                    draw_target_zone(
                        ui,
                        state.metrics.peak_db,
                    );

                    if !state.readiness.ready
                        && !state
                            .health
                            .reasons
                            .is_empty()
                    {
                        ui.separator();

                        ui.heading(
                            "HOW TO FIX IT",
                        );

                        for reason in
                            &state.health.reasons
                        {
                            ui.colored_label(
                                theme::WARNING,
                                format!(
                                    "• {}",
                                    reason
                                ),
                            );
                        }
                    }

                    ui.separator();

                    ui.heading(
                        "DEVICE",
                    );

                    ui.label(
                        RichText::new(
                            &state.device_name,
                        )
                        .color(theme::TEXT),
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
                    } else {
                        ui.label(
                            "Last Signal: Never",
                        );
                    }

                    ui.separator();

                    ui.heading(
                        "DIAGNOSTICS",
                    );

                    ui.label(
                        format!(
                            "Current Level: {:.1} dB",
                            state
                                .metrics
                                .current_db
                        ),
                    );

                    ui.label(
                        format!(
                            "Peak Level: {:.1} dB",
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

                    ui.label(
                        format!(
                            "Signal Present: {}",
                            if state
                                .health
                                .signal_present
                            {
                                "YES"
                            } else {
                                "NO"
                            }
                        ),
                    );

                    ui.label(
                        format!(
                            "Speaking: {}",
                            if state
                                .metrics
                                .speaking_now
                            {
                                "YES"
                            } else {
                                "NO"
                            }
                        ),
                    );

                    ui.separator();

                    ui.heading(
                        "HISTORY",
                    );

                    draw_history_graph(
                        ui,
                        state
                            .metrics
                            .history
                            .iter()
                            .copied(),
                    );
                });
            });

        ctx.request_repaint();
    }
}