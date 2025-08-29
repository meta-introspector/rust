use std::sync::Mutex;

// A global, thread-safe vector to store log entries
lazy_static::lazy_static! {
    pub static ref LOG_ENTRIES: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());
}

// Define a struct to hold a single log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64, // Nanoseconds since epoch
    pub event_type: String,
    pub module: String,
    pub function: String,
    pub description: String,
    pub details: Option<String>,
    pub duration_ns: Option<u64>,
    pub status: String,
}