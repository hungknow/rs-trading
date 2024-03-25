#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash, Default)]
pub enum Resolution {
    S1,
    #[default]
    M1,
    M5,
    M15,
    M30,
    H1,
    H4,
    D1,
    W1,
}

impl Resolution {
    pub fn to_seconds(&self) -> i64 {
        match self {
            Resolution::S1 => 1,
            Resolution::M1 => 60,
            Resolution::M5 => 300,
            Resolution::M15 => 900,
            Resolution::M30 => 1800,
            Resolution::H1 => 3600,
            Resolution::H4 => 14400,
            Resolution::D1 => 86400,
            Resolution::W1 => 604800,
        }
    }

    pub fn to_milliseconds(&self) -> i64 {
        self.to_seconds() * 1000
    }

    pub fn from_seconds(seconds: i64) -> Option<Resolution> {
        match seconds.abs() {
            1 => Some(Resolution::S1),
            60 => Some(Resolution::M1),
            300 => Some(Resolution::M5),
            900 => Some(Resolution::M15),
            1800 => Some(Resolution::M30),
            3600 => Some(Resolution::H1),
            14400 => Some(Resolution::H4),
            86400 => Some(Resolution::D1),
            604800 => Some(Resolution::W1),
            _ => None,
        }
    }
}
