use std::collections::VecDeque;

pub const HISTORY_SIZE: usize = 120;

pub fn push_history(
    history: &mut VecDeque<f32>,
    value: f32,
) {
    history.push_back(value);

    while history.len() > HISTORY_SIZE {
        history.pop_front();
    }
}