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
            Ok(result) => result,
            Err(_) => String::new(),
        }
    }

    /// Chainable method to add a key:value pair into the event payload. This enables the event to
    /// be enriched at multiple points during an operation to enable a better understanding of the
    /// state of things at the time of the event.
    pub fn add_field(&mut self, key: String, value: String) -> &mut Self {
        self.payload.insert(key, value);
        self
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.out())
    }
}

#[test]
fn test_event_display() {
    let mut expected = HashMap::new();
    expected.insert(String::from("field"), String::from("value"));

    let mut ev = Event::new();
    ev.payload.insert("field".to_string(), "value".to_string());
    assert_eq!(ev.payload, expected);
}

#[test]
fn test_add_field() {
    let mut expected = HashMap::new();
    expected.insert(String::from("first_key"), String::from("first_value"));

    let mut ev = Event::new();
    ev.add_field(String::from("first_key"), String::from("first_value"));
    assert_eq!(ev.payload, expected);
}

#[test]
fn test_add_field_in_chain() {
    let mut expected = HashMap::new();
    expected.insert(String::from("first_key"), String::from("first_value"));
    expected.insert(String::from("second_key"), String::from("second_value"));

    let mut ev = Event::new();
    ev.add_field(String::from("first_key"), String::from("first_value"))
        .add_field(String::from("second_key"), String::from("second_value"));
    assert_eq!(ev.payload, expected);
}
