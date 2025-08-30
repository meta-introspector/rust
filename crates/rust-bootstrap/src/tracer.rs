use std::sync::Mutex;
use once_cell::sync::Lazy;
use chrono::Utc;

pub struct TraceEvent {
    pub timestamp: i64,
    pub event_type: String,
    pub message: String,
    pub details: Option<String>,
}

static TRACE_EVENTS: Lazy<Mutex<Vec<TraceEvent>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn record_trace_event(event_type: &str, message: &str, details: Option<&str>) {
    let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or_default();
    let event = TraceEvent {
        timestamp,
        event_type: event_type.to_string(),
        message: message.to_string(),
        details: details.map(|s| s.to_string()),
    };
    TRACE_EVENTS.lock().unwrap().push(event);
    // For immediate feedback, print to stderr
    eprintln!("[TRACE] {}: {} - {}", event_type, message, details.unwrap_or(""));
}

pub fn get_trace_events() -> Vec<TraceEvent> {
    TRACE_EVENTS.lock().unwrap().drain(..).collect()
}
