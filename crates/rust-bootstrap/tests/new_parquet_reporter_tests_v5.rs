#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    use rust_bootstrap::parquet_reporter;
    use rust_bootstrap::BuildState;
    use rust_bootstrap::Args;
    use rust_bootstrap::config::loader::Config; // Corrected import for Config
    use rust_bootstrap::bootstrap_stages::stage0_detector::Stage0;
    use rust_bootstrap::BuildStateCreationArgs;
    use std::path::{PathBuf, Path};
    use std::fs;
    use clap::Parser; // Added for Args::parse_from
    

    #[test]
    fn test_write_build_config_to_parquet_v5() {
        let temp_dir = PathBuf::from("target/new_test_parquet_reporter_v5/build_config");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let build_state = super::helpers::setup_test_build_state();

        let output_path = temp_dir.join("build_config.parquet");
        let result = parquet_reporter::write_build_config_to_parquet(&build_state, output_path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to write build config to parquet: {:?}", result.err());

        // Verify the file exists
        assert!(output_path.exists(), "Parquet file should exist");

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_write_record_batch_to_parquet_v5() {
        let temp_dir = PathBuf::from("target/new_test_parquet_reporter_v5/record_batch");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        // Create a dummy RecordBatch (requires arrow-array and arrow-schema)
        use arrow_array::{ArrayRef, RecordBatch};
        use arrow_array::builder::{Int32Builder, StringBuilder};
        use arrow_schema::{Field, Schema, DataType};
        use std::sync::Arc;

        let schema = Schema::new(vec![
            Field::new("id", DataType::Int32, false),
            Field::new("name", DataType::Utf8, false),
        ]);

        let mut id_builder = Int32Builder::new();
        let mut name_builder = StringBuilder::new();

        id_builder.append_value(1);
        name_builder.append_value("test1");

        id_builder.append_value(2);
        name_builder.append_value("test2");

        let record_batch = RecordBatch::try_new(
            Arc::new(schema),
            vec![
                Arc::new(id_builder.finish()) as ArrayRef,
                Arc::new(name_builder.finish()) as ArrayRef,
            ],
        ).unwrap();

        let output_path = temp_dir.join("record_batch.parquet");
        let result = parquet_reporter::write_record_batch_to_parquet(record_batch, output_path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to write record batch to parquet: {:?}", result.err());

        // Verify the file exists
        assert!(output_path.exists(), "Parquet file should exist");

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_read_and_summarize_build_config_metrics_v5() {
        // This test requires a valid parquet file to read from.
        // We can reuse the file created by test_write_build_config_to_parquet.
        let temp_dir = PathBuf::from("target/new_test_parquet_reporter_v5/summarize");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        let build_state = super::helpers::setup_test_build_state();

        let output_path = temp_dir.join("build_config_to_summarize.parquet");
        parquet_reporter::write_build_config_to_parquet(&build_state, output_path.to_str().unwrap()).unwrap();

        let result = parquet_reporter::read_and_summarize_build_config_metrics(output_path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to read and summarize build config metrics: {:?}", result.err());

        // Further assertions would involve checking the content of the summary.

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
