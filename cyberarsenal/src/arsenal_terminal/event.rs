use std::{fmt::{self, Display}, time::SystemTime};
use chrono::{DateTime, Utc};




pub enum LevelCode {
    INFO,
    DEBUG,
    TRACE,
    WARNING,
    ERROR,
    CRITICAL
}

impl Display for LevelCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LevelCode::INFO => { write!(f, "INFO") },
            LevelCode::DEBUG => { write!(f, "DEBUG") },
            LevelCode::TRACE => { write!(f, "DEBUG") },
            LevelCode::WARNING => { write!(f, "WARNING") },
            LevelCode::ERROR => { write!(f, "ERROR") },
            LevelCode::CRITICAL => { write!(f, "CRITICAL") }
        }
    }
}

pub struct AppEvent {
    pub datetime: DateTime<Utc>,
    pub level: LevelCode,
    pub text: String
}

impl AppEvent {
    pub fn new(text: &str, level: LevelCode) -> Self {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        
        AppEvent {
            datetime,
            level,
            text: text.to_string()
        }
    }
}