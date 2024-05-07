use std::{fmt::{self, Display}, time::SystemTime};
use chrono::{DateTime, Utc};




pub enum ErrorCode {
    INFO,
    DEBUG,
    TRACE,
    WARNING,
    ERROR,
    CRITICAL
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::INFO => { write!(f, "INFO") },
            ErrorCode::DEBUG => { write!(f, "DEBUG") },
            ErrorCode::TRACE => { write!(f, "DEBUG") },
            ErrorCode::WARNING => { write!(f, "WARNING") },
            ErrorCode::ERROR => { write!(f, "ERROR") },
            ErrorCode::CRITICAL => { write!(f, "CRITICAL") }
        }
    }
}

pub struct AppEvent {
    pub datetime: DateTime<Utc>,
    pub level: ErrorCode,
    pub text: String
}

impl AppEvent {
    pub fn new(text: &str, level: ErrorCode) -> Self {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        
        AppEvent {
            datetime,
            level,
            text: text.to_string()
        }
    }
}