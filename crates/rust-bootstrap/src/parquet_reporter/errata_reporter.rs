use std::fs::File;
use std::sync::Arc;
use arrow_array::{RecordBatch, StringArray, TimestampNanosecondArray};
use arrow_schema::{Schema, Field, DataType, TimeUnit};
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use crate::git_analyzer::errata::GitErrata;

pub fn write_errata_to_parquet(
    errata: Vec<GitErrata>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if errata.is_empty() {
        println!("No errata to write to Parquet.");
        return Ok(());
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("oid", DataType::Utf8, false),
        Field::new("error_message", DataType::Utf8, false),
        Field::new(
            "timestamp",
            DataType::Timestamp(TimeUnit::Nanosecond, Some(Arc::from("UTC"))),
            false,
        ),
    ]));

    let oids: Vec<String> = errata.iter().map(|e| e.oid.clone()).collect();
    let error_messages: Vec<String> = errata.iter().map(|e| e.error_message.clone()).collect();
    let timestamps: Vec<i64> = errata.iter().map(|e| e.timestamp.timestamp_nanos_opt().unwrap_or_default()).collect();

    let oids_array = StringArray::from(oids);
    let error_messages_array = StringArray::from(error_messages);
    let timestamps_array = TimestampNanosecondArray::from(timestamps);

    let record_batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(oids_array),
            Arc::new(error_messages_array),
            Arc::new(timestamps_array),
        ],
    )?;

    let file = File::create(file_path)?;
    let props = Some(WriterProperties::builder().build());
    let mut writer = ArrowWriter::try_new(file, schema, props)?;

    writer.write(&record_batch)?;
    writer.close()?;

    println!("Errata written to {}", file_path);
    Ok(())
}
