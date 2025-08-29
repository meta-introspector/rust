use arrow_schema::{DataType, Field, Schema, TimeUnit};
use std::sync::Arc;

pub fn create_arrow_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("command_name", DataType::Utf8, false),
        Field::new("command_status", DataType::Utf8, false),
        Field::new("command_stdout", DataType::Utf8, false),
        Field::new("command_stderr", DataType::Utf8, false),
        Field::new("command_duration_ns", DataType::Timestamp(TimeUnit::Nanosecond, None), false),
    ]))
}