use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub low_db: f32,
    pub clip_db: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            low_db: -40.0,
            clip_db: -1.0,
        }
    }
}