#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingQuality {
    Excellent,
    Good,
    Marginal,
    TooQuiet,
    Clipping,
    NoSignal,
}

impl RecordingQuality {
    pub fn from_peak_db(
        peak_db: f32,
        signal_present: bool,
    ) -> Self {
        if !signal_present {
            return Self::NoSignal;
        }

        match peak_db {
            db if db < -18.0 => Self::TooQuiet,

            db if (-18.0..-12.0).contains(&db) => {
                Self::Marginal
            }

            db if (-12.0..=-6.0).contains(&db) => {
                Self::Excellent
            }

            db if (-6.0..=-3.0).contains(&db) => {
                Self::Good
            }

            db if db > -3.0 => Self::Clipping,

            _ => Self::Marginal,
        }
    }

    pub fn target_zone_ok(
        peak_db: f32,
    ) -> bool {
        (-12.0..=-6.0)
            .contains(&peak_db)
    }

    pub fn target_zone_status(
        peak_db: f32,
        signal_present: bool,
    ) -> &'static str {
        if !signal_present {
            return "NO SIGNAL";
        }

        match peak_db {
            db if db < -18.0 => "TOO QUIET",

            db if (-18.0..-12.0).contains(&db) => {
                "LOW"
            }

            db if (-12.0..=-6.0).contains(&db) => {
                "IDEAL"
            }

            db if (-6.0..=-3.0).contains(&db) => {
                "HOT"
            }

            db if db > -3.0 => {
                "CLIPPING RISK"
            }

            _ => "UNKNOWN",
        }
    }

    pub fn label(
        &self,
    ) -> &'static str {
        match self {
            Self::Excellent => "EXCELLENT",
            Self::Good => "GOOD",
            Self::Marginal => "MARGINAL",
            Self::TooQuiet => "TOO QUIET",
            Self::Clipping => "CLIPPING",
            Self::NoSignal => "NO SIGNAL",
        }
    }
}