use arrow_array::{RecordBatch, StringArray, TimestampNanosecondArray, UInt64Array};
use arrow_schema::Schema;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;
use crate::bootstrap_stages::operational_logger::schemas::bootstrap_operational_log_schema;

// A global, thread-safe vector to store log entries
lazy_static::lazy_static! {
    static ref LOG_ENTRIES: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());
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

pub fn log_event(
    event_type: &str,
    module: &str,
    function: &str,
    description: &str,
    details: Option<String>,
    duration_ns: Option<u64>,
    status: &str,
) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    let entry = LogEntry {
        timestamp,
        event_type: event_type.to_string(),
        module: module.to_string(),
        function: function.to_string(),
        description: description.to_string(),
        details,
        duration_ns,
        status: status.to_string(),
    };

    LOG_ENTRIES.lock().unwrap().push(entry);
}

pub fn get_logged_events_as_record_batch() -> Result<RecordBatch, Box<dyn std::error.Error>> {
    let log_entries = LOG_ENTRIES.lock().unwrap();

    let mut timestamps = Vec::new();
    let mut event_types = Vec::new();
    let mut modules = Vec::new();
    let mut functions = Vec::new();
    let mut descriptions = Vec::new();
    let mut details = Vec::new();
    let mut duration_ns = Vec::new();
    let mut statuses = Vec::new();

    for entry in log_entries.iter() {
        timestamps.push(entry.timestamp as i64); // Arrow Timestamp is i64
        event_types.push(entry.event_type.clone());
        modules.push(entry.module.clone());
        functions.push(entry.function.clone());
        descriptions.push(entry.description.clone());
        details.push(entry.details.clone());
        duration_ns.push(entry.duration_ns);
        statuses.push(entry.status.clone());
    }

    let schema = bootstrap_operational_log_schema();

    let record_batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(TimestampNanosecondArray::from(timestamps)),
            Arc::new(StringArray::from(event_types)),
            Arc::new(StringArray::from(modules)),
            Arc::new(StringArray::from(functions)),
            Arc::new(StringArray::from(descriptions)),
            Arc::new(StringArray::from(details)),
            Arc::new(UInt64Array::from(duration_ns)),
            Arc::new(StringArray::from(statuses)),
        ],
    )?;

    Ok(record_batch)
}
