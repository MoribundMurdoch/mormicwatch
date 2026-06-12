use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use crossbeam_channel::unbounded;

use mormicwatch_core::{
    audio::{
        capture::{start_capture, AudioFrame},
        devices::default_input_name,
        health::{
            calculate_health,
            classify,
            SPEAKING_THRESHOLD_DB,
        },
        history::push_history,
    },
    models::app_state::AppState,
};

pub fn start_audio_service(
    state: Arc<Mutex<AppState>>,
) {
    let (tx, rx) = unbounded::<AudioFrame>();

    {
        let mut state = state.lock().unwrap();

        if let Some(device_name) =
            default_input_name()
        {
            state.device_name = device_name;
        }
    }

    thread::spawn(move || {
        let _stream = start_capture(tx)
            .expect("failed to start audio capture");

        loop {
            if let Ok(frame) = rx.recv() {
                let mut state =
                    state.lock().unwrap();

                state.metrics.current_db =
                    frame.rms_db;

                state.metrics.rms_db =
                    frame.rms_db;

                state.metrics.peak_db =
                    frame.peak_db;

                state.metrics.samples_processed += 1;

                state.metrics.peak_hold_db =
                    if frame.peak_db >
                        state.metrics.peak_hold_db
                    {
                        frame.peak_db
                    } else {
                        (
                            state.metrics
                                .peak_hold_db
                                - 0.05
                        )
                        .max(frame.peak_db)
                    };

                push_history(
                    &mut state.metrics.history,
                    frame.rms_db,
                );

                state.metrics.speaking_now =
                    frame.peak_db
                        >= SPEAKING_THRESHOLD_DB;

                if state.metrics.speaking_now {
                    state.metrics.last_signal_at =
                        Some(Instant::now());
                }

                state.status = classify(
                    frame.peak_db,
                    state.metrics.last_signal_at,
                );

                state.health =
                    calculate_health(
                        frame.peak_db,
                        frame.rms_db,
                        state.metrics.last_signal_at,
                        true,
                    );
            }
        }
    });
}