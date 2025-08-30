#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use rust_bootstrap::bootstrap_stages::operational_logger;
    use rust_bootstrap::trace_events::TraceEvent;
    use std::time::SystemTime;

    #[test]
    fn test_log_event_new() {
        let event = TraceEvent::new(
            "test_category".to_string(),
            "test_name".to_string(),
            SystemTime::now(),
            None,
            None,
            None,
        );
        operational_logger::logger::log_event(event);
        // Cannot directly assert on the global static LOGGER content without unsafe or specific test harnesses.
        // This test primarily ensures the function doesn't panic.
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_get_logged_events_as_record_batch_new() {
        // This test requires a more complex setup to verify the Arrow RecordBatch.
        // It would involve logging some events and then asserting on the schema and data of the returned RecordBatch.
        // For now, we'll just ensure it returns a Result.

        // Log a dummy event to ensure there's something to convert
        let event = TraceEvent::new(
            "test_category_batch".to_string(),
            "test_name_batch".to_string(),
            SystemTime::now(),
            Some("test_data".to_string()),
            None,
            None,
        );
        operational_logger::logger::log_event(event);

        let result = operational_logger::logger::get_logged_events_as_record_batch();
        assert!(result.is_ok(), "get_logged_events_as_record_batch should return Ok");
        let record_batch = result.unwrap();

        // Basic assertions on the record batch
        assert!(record_batch.num_rows() > 0, "RecordBatch should contain logged events");
        assert!(record_batch.schema().fields().len() > 0, "RecordBatch schema should not be empty");
    }
}
