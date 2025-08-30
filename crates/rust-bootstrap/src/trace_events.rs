use arrow_schema::{DataType, Field, Schema, TimeUnit};
use std::sync::Arc;

pub struct TraceEvent {
    pub timestamp: i64, // Nanoseconds since epoch
    pub event_type: String,
    pub message: String,
    pub details: Option<String>, // Optional JSON string for more complex details
}

pub fn trace_event_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("timestamp", DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into())), false),
        Field::new("event_type", DataType::Utf8, false),
        Field::new("message", DataType::Utf8, false),
        Field::new("details", DataType::Utf8, true), // Nullable
    ]))
}
