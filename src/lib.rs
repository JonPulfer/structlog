use chrono::Utc;
use serde::Serializer;
use std::collections::HashMap;

mod iron_logger;

/// Context for the event to record in the log.
#[derive(Debug)]
pub struct LogContext {
    pub fields: HashMap<String, String>,
    created: chrono::DateTime<Utc>,
}

impl LogContext {
    pub fn new() -> LogContext {
        LogContext {
            created: chrono::Local::now().with_timezone(&Utc),
            fields: HashMap::new(),
        }
    }
}

/// A data object that can be used as a loggable event.
pub trait Loggable {
    fn add(&mut self, key: String, value: impl Serializer) -> Self;
}

/// Classic syslog style levels.
pub enum Event {
    DEBUG(Loggable),
    INFO(Loggable),
    WARN(Loggable),
    ERROR(Loggable),
    CRITICAL(Loggable),
}

impl Event {
    pub fn debug(event: impl Loggable) -> Event {
        Event::DEBUG(event)
    }

    pub fn info(event: impl Loggable) -> Event {
        Event::INFO(event)
    }

    pub fn warn(event: impl Loggable) -> Event {
        Event::WARN(event)
    }

    pub fn error(event: impl Loggable) -> Event {
        Event::ERROR(event)
    }

    pub fn critical(event: impl Loggable) -> Event {
        Event::CRITICAL(event)
    }
}
