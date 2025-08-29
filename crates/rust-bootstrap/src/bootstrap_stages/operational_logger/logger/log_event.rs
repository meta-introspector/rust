use std::time::{SystemTime, UNIX_EPOCH};
use super::log_entry::{LogEntry, LOG_ENTRIES};

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