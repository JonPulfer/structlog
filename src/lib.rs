use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

mod iron_logger;

/// Classic syslog style levels.
pub enum Level {
    DEBUG(Event),
    INFO(Event),
    WARN(Event),
    ERROR(Event),
    CRITICAL(Event),
}

impl Level {
    pub fn debug(event: Event) -> Level {
        Level::DEBUG(event)
    }

    pub fn info(event: Event) -> Level {
        Level::INFO(event)
    }

    pub fn warn(event: Event) -> Level {
        Level::WARN(event)
    }

    pub fn error(event: Event) -> Level {
        Level::ERROR(event)
    }

    pub fn critical(event: Event) -> Level {
        Level::CRITICAL(event)
    }
}

/// Simple event to record to the output channel.
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    payload: HashMap<String, String>,
    created: chrono::DateTime<Utc>,
}

impl Event {
    pub fn new() -> Event {
        Event {
            payload: HashMap::new(),
            created: chrono::Local::now().with_timezone(&Utc),
        }
    }

    fn out(&self) -> String {
        let results = serde_json::to_string(self);
        match results {
            Ok(result) => { result }
            Err(_) => { String::new() }
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.out())
    }
}

#[test]
fn test_event_display() {
    let mut ev = Event::new();
    ev.payload.insert("field".to_string(), "value".to_string());
    println!("{}", ev);
}