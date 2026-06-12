mod app;
mod http;
mod services;

use std::sync::{
    Arc,
    Mutex,
};

use app::{
    dashboard::Dashboard,
    style::apply_style,
};

use http::server::start_http_server;

use mormicwatch_core::models::app_state::AppState;

use services::audio_service::start_audio_service;

fn main() -> eframe::Result<()> {
    let state =
        Arc::new(Mutex::new(AppState::default()));

    start_audio_service(
        state.clone(),
    );

    start_http_server(
        state.clone(),
    );

    let options =
        eframe::NativeOptions {
            viewport:
                egui::ViewportBuilder::default()
                    .with_inner_size([
                        700.0,
                        650.0,
                    ])
                    .with_title(
                        "MorMicWatch",
                    ),
            ..Default::default()
        };

    eframe::run_native(
        "MorMicWatch",
        options,
        Box::new(move |cc| {
            apply_style(
                &cc.egui_ctx,
            );

            Ok(Box::new(
                Dashboard::new(
                    state.clone(),
                ),
            ))
        }),
    )
}