use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
enum Level {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::DEBUG => {
                write!(f, "debug")
            }
            Level::ERROR => {
                write!(f, "error")
            }
            Level::INFO => {
                write!(f, "info")
            }
            Level::WARN => {
                write!(f, "warn")
            }
        }
    }
}

/// Simple event to record to the output channel.
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    attributes: HashMap<String, String>,
    pub created: DateTime<Utc>,
    level: Level,
    severity: Level,
}

impl Event {
    pub fn new() -> Event {
        Event {
            attributes: HashMap::new(),
            created: Utc::now(),
            level: Level::INFO,
            severity: Level::INFO,
        }
    }

    /// Chainable method to add a key:value pair into the event payload. This enables the event to
    /// be enriched at multiple points during an operation to enable a better understanding of the
    /// state of things at the time of the event.
    pub fn add_field(&mut self, key: String, value: String) -> &mut Self {
        self.attributes.insert(key, value);
        self
    }

    fn set_level(self, level: Level) -> Event {
        Event {
            attributes: self.attributes.clone(),
            created: self.created.clone(),
            level,
            severity: level.clone(),
        }
    }

    pub fn info(self) -> Event {
        self.set_level(Level::INFO)
    }

    pub fn debug(self) -> Event {
        self.set_level(Level::DEBUG)
    }

    pub fn warn(self) -> Event {
        self.set_level(Level::WARN)
    }

    pub fn error(self) -> Event {
        self.set_level(Level::ERROR)
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match serde_json::to_string(&self) {
            Ok(serialised) => serialised,
            Err(json_error) => {
                let error_message = match serde_json::to_string(&json_error.to_string()) {
                    Ok(serialised_error) => serialised_error,
                    Err(total_fail_error) => {
                        format!("unable to serialise to json: {}", total_fail_error)
                    }
                };
                error_message
            }
        };
        write!(f, "{}", output)
    }
}

impl str::FromStr for Event {
    type Err = ParseEventError;

    fn from_str(message: &str) -> Result<Self, Self::Err> {
        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(String::from("message"), message.to_string());
        Ok(Event {
            attributes,
            created: Utc::now(),
            level: Level::INFO,
            severity: Level::INFO,
        })
    }
}

impl From<&str> for Event {
    fn from(message: &str) -> Event {
        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(String::from("message"), message.to_string());
        Event {
            attributes,
            created: Utc::now(),
            level: Level::INFO,
            severity: Level::INFO,
        }
    }
}

impl From<Box<dyn Error>> for Event {
    fn from(error: Box<dyn Error>) -> Event {
        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(String::from("error"), error.to_string());
        Event {
            attributes,
            created: Utc::now(),
            level: Level::ERROR,
            severity: Level::ERROR,
        }
    }
}

#[derive(Debug)]
pub struct ParseEventError {
    message: String,
}

impl fmt::Display for ParseEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to parse event: {}", self.message)
    }
}

#[test]
fn test_event_display() {
    let mut expected = HashMap::new();
    expected.insert(String::from("field"), String::from("value"));

    let mut ev = Event::new();
    ev.attributes
        .insert("field".to_string(), "value".to_string());
    assert_eq!(ev.attributes, expected);
}

#[test]
fn test_add_field() {
    let mut expected = HashMap::new();
    expected.insert(String::from("first_key"), String::from("first_value"));

    let mut ev = Event::new();
    ev.add_field(String::from("first_key"), String::from("first_value"));
    assert_eq!(ev.attributes, expected);
}

#[test]
fn test_add_field_in_chain() {
    let mut expected = HashMap::new();
    expected.insert(String::from("first_key"), String::from("first_value"));
    expected.insert(String::from("second_key"), String::from("second_value"));

    let mut ev = Event::new();
    ev.add_field(String::from("first_key"), String::from("first_value"))
        .add_field(String::from("second_key"), String::from("second_value"));
    assert_eq!(ev.attributes, expected);
}
