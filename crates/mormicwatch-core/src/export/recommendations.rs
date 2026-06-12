use crate::{
    models::app_state::AppState,
    recommendations::engine::generate_recommendations,
};

pub fn build_recommendations_json(
    state: &AppState,
) -> String {
    let recommendations =
        generate_recommendations(
            &state.health,
            &state.readiness,
        );

    let mut json =
        String::from("[");

    for (i, rec) in recommendations.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }

        json.push_str(
            &format!(
                r#"{{
"title":"{}",
"description":"{}",
"ffmpeg_fix":"{}"
}}"#,
                rec.title,
                rec.description,
                rec.ffmpeg_fix
                    .as_deref()
                    .unwrap_or("")
            ),
        );
    }

    json.push(']');

    json
}