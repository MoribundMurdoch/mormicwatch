mod app;
mod services;

use std::sync::{
    Arc,
    Mutex,
};

use app::dashboard::Dashboard;
use mormicwatch_core::models::app_state::AppState;
use services::audio_service::start_audio_service;

fn main() -> eframe::Result<()> {
    let state =
        Arc::new(Mutex::new(AppState::default()));

    start_audio_service(state.clone());

    let options =
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([500.0, 400.0]),
            ..Default::default()
        };

    eframe::run_native(
        "MorMicWatch",
        options,
        Box::new(|_| {
            Ok(Box::new(
                Dashboard::new(state),
            ))
        }),
    )
}