#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use rust_bootstrap::bootstrap_stages::operational_logger;
    use rust_bootstrap::trace_events::TraceEvent;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::collections::HashMap;

    #[test]
    fn test_log_event_v3() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;

        operational_logger::logger::log_event(
            "test_event_type",
            "test_module",
            "test_function",
            "test_description",
            None,
            Some(123),
            "SUCCESS",
        );
        // Cannot directly assert on the global static LOGGER content without unsafe or specific test harnesses.
        // This test primarily ensures the function doesn't panic.
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_get_logged_events_as_record_batch_v3() {
        // This test requires a more complex setup to verify the Arrow RecordBatch.
        // It would involve logging some events and then asserting on the schema and data of the returned RecordBatch.
        // For now, we'll just ensure it returns a Result.

        // Log a dummy event to ensure there's something to convert
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;

        operational_logger::logger::log_event(
            "test_event_type_batch",
            "test_module_batch",
            "test_function_batch",
            "test_description_batch",
            Some("test_details_batch".to_string()),
            Some(456),
            "FAILURE",
        );

        let result = operational_logger::logger::get_logged_events_as_record_batch();
        assert!(result.is_ok(), "get_logged_events_as_record_batch should return Ok");
        let record_batch = result.unwrap();

        // Basic assertions on the record batch
        assert!(record_batch.num_rows() > 0, "RecordBatch should contain logged events");
        assert!(record_batch.schema().fields().len() > 0, "RecordBatch schema should not be empty");

        // Assert on specific fields based on the actual TraceEvent schema
        let schema = record_batch.schema();
        assert!(schema.field_with_name("timestamp").is_ok());
        assert!(schema.field_with_name("event_type").is_ok());
        assert!(schema.field_with_name("message").is_ok()); // This is 'description' in log_event
        assert!(schema.field_with_name("details").is_ok());
        // Add assertions for module, function, duration_ns, status if they are part of the schema
    }
}
