use std::{
    collections::VecDeque,
    time::Instant,
};

#[derive(Debug, Clone)]
pub struct AudioMetrics {
    pub current_db: f32,
    pub peak_db: f32,
    pub rms_db: f32,

    pub peak_hold_db: f32,

    pub speaking_now: bool,

    pub last_signal_at: Option<Instant>,

    pub samples_processed: u64,

    pub history: VecDeque<f32>,
}

impl Default for AudioMetrics {
    fn default() -> Self {
        Self {
            current_db: -100.0,
            peak_db: -100.0,
            rms_db: -100.0,

            peak_hold_db: -100.0,

            speaking_now: false,

            last_signal_at: None,

            samples_processed: 0,

            history: VecDeque::with_capacity(120),
        }
    }
}