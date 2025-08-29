use arrow_schema::{Schema, Field, DataType, TimeUnit};
use std::sync::Arc;

pub fn bootstrap_operational_log_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("timestamp", DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into())), false),
        Field::new("event_type", DataType::Utf8, false),
        Field::new("module", DataType::Utf8, false),
        Field::new("function", DataType::Utf8, false),
        Field::new("description", DataType::Utf8, false),
        Field::new("details", DataType::Utf8, true), // Nullable
        Field::new("duration_ns", DataType::UInt64, true), // Nullable
        Field::new("status", DataType::Utf8, false),
    ]))
}
