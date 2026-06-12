#[derive(Debug, Clone)]
pub struct ReadinessReport {
    pub ready: bool,
    pub score: u8,
    pub recommendations: Vec<String>,
}

impl Default for ReadinessReport {
    fn default() -> Self {
        Self {
            ready: false,
            score: 0,
            recommendations: Vec::new(),
        }
    }
}
