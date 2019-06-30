use std::collections::HashMap;
use chrono::Utc;

mod iron_logger;

#[derive(Debug)]
pub struct LogContext {
    pub fields: HashMap<String, String>,
    created: chrono::DateTime<Utc>,
}

impl LogContext {
    pub fn new(fields: HashMap<String, String>) -> LogContext {
        LogContext{
            created: chrono::Local::now().with_timezone(&Utc),
            fields,
        }
    }
}