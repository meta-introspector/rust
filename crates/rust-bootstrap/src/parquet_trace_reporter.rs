use arrow_array::{RecordBatch, StringArray, TimestampNanosecondArray};
use arrow_schema::{DataType, Field, Schema, TimeUnit};
use std::sync::Arc;
use std::error::Error;
use std::fs::File;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

use crate::tracer::TraceEvent;
use crate::trace_events::trace_event_schema;

pub fn write_trace_events_to_parquet(events: Vec<TraceEvent>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let schema = trace_event_schema();

    let timestamps: Vec<i64> = events.iter().map(|e| e.timestamp).collect();
    let event_types: Vec<&str> = events.iter().map(|e| e.event_type.as_str()).collect();
    let messages: Vec<&str> = events.iter().map(|e| e.message.as_str()).collect();
    let details: Vec<Option<&str>> = events.iter().map(|e| e.details.as_deref()).collect();

    let timestamp_array = TimestampNanosecondArray::from(timestamps);
    let event_type_array = StringArray::from(event_types);
    let message_array = StringArray::from(messages);
    let details_array = StringArray::from_iter(details);

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(timestamp_array),
            Arc::new(event_type_array),
            Arc::new(message_array),
            Arc::new(details_array),
        ],
    )?;

    let file = File::create(file_path)?;

    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(props))?;
    writer.write(&batch)?;
    writer.close()?;

    println!("Trace events written to {}.", file_path);

    Ok(())
}
