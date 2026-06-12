#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicStatus {
    Healthy,
    Quiet,
    Clipping,
    NoSignal,
    Disconnected,
}

impl MicStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Healthy => "HEALTHY",
            Self::Quiet => "QUIET",
            Self::Clipping => "CLIPPING",
            Self::NoSignal => "NO SIGNAL",
            Self::Disconnected => "DISCONNECTED",
        }
    }
}