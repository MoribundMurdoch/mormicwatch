pub fn linear_to_db(value: f32) -> f32 {
    if value <= 0.00001 {
        -100.0
    } else {
        20.0 * value.log10()
    }
}

pub fn peak(samples: &[f32]) -> f32 {
    samples
        .iter()
        .map(|s| s.abs())
        .fold(0.0, f32::max)
}

pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    let sum: f32 = samples.iter().map(|s| s * s).sum();

    (sum / samples.len() as f32).sqrt()
}