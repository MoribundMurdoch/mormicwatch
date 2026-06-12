use crate::models::{
    app_state::AppState,
    status_snapshot::StatusSnapshot,
};

pub fn build_status_json(
    state: &AppState,
) -> String {
    let snapshot =
        StatusSnapshot::from_state(
            state,
        );

    format!(
        r#"{{
  "ready": {},
  "quality": "{}",
  "score": {},

  "current_db": {:.1},
  "peak_db": {:.1},
  "rms_db": {:.1},

  "target_zone_ok": {},
  "signal_present": {},
  "speaking_now": {},

  "device": "{}"
}}"#,
        snapshot.ready,
        snapshot.quality,
        snapshot.score,

        snapshot.current_db,
        snapshot.peak_db,
        snapshot.rms_db,

        snapshot.target_zone_ok,
        snapshot.signal_present,
        snapshot.speaking_now,

        snapshot.device_name,
    )
}