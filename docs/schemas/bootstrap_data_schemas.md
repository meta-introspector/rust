## Rust Bootstrap Data Schemas (Arrow/Parquet)

This document defines the conceptual Arrow/Parquet schemas for the inputs (domains) and outputs (ranges) of the highly modularized functions (basic blocks) within the `rust-bootstrap` crate. This aligns with the architectural vision of treating each basic block as a data transformation unit.

---

### Module: `bootstrap_stages::cli_parser`

#### `Args` Struct Schema (Output of `parse_args_from_cli.rs`)

| Field Name | Arrow Data Type | Nullable |
| :--------- | :-------------- | :------- |
| `verbose`  | `UInt8`         | No       |
| `config`   | `Utf8`          | Yes      |

---

### Module: `bootstrap_stages::config_loader`

#### `read_file_content.rs`

*   **Input (Domain) Schema:**

    | Field Name | Arrow Data Type |
    | :--------- | :-------------- |
    | `path`     | `Utf8`          |

*   **Output (Range) Schema:**

    | Field Name | Arrow Data Type |
    | :--------- | :-------------- |
    | `content`  | `Utf8`          |

#### `Config` Struct Schema (Output of `parse_toml_content.rs`)

| Field Name            | Arrow Data Type |
| :-------------------- | :-------------- |
| `build.download_ci_rustc` | `Boolean`       |
| `build.download_ci_llvm`  | `Boolean`       |

---

### Module: `bootstrap_stages::stage0_detector`

#### `detect_rustc_path.rs`

*   **Input (Domain) Schema:** (None explicit - implicitly consumes environment variables)

*   **Output (Range) Schema:**

    | Field Name   | Arrow Data Type |
    | :----------- | :-------------- |
    | `rustc_path` | `Utf8`          |

#### `detect_cargo_path.rs`

*   **Input (Domain) Schema:** (None explicit - implicitly consumes environment variables)

*   **Output (Range) Schema:**

    | Field Name   | Arrow Data Type |
    | :----------- | :-------------- |
    | `cargo_path` | `Utf8`          |

#### `Stage0` Struct Schema (Output of `Stage0::detect`)

| Field Name | Arrow Data Type |
| :--------- | :-------------- |
| `rustc`    | `Utf8`          |
| `cargo`    | `Utf8`          |

---

### Module: `bootstrap_stages::env_setup`

*Note: Functions in this module primarily have side effects (setting environment variables) and do not produce explicit Arrow/Parquet dataset outputs.*

#### `set_rustc_bootstrap.rs`

*   **Input (Domain) Schema:** (None explicit)
*   **Output (Range) Schema:** (None explicit - side effect: sets environment variable)

#### `set_rustc_stage0.rs`

*   **Input (Domain) Schema:**

    | Field Name   | Arrow Data Type |
    | :----------- | :-------------- |
    | `rustc_path` | `Utf8`          |

*   **Output (Range) Schema:** (None explicit - side effect: sets environment variable)

#### `set_cargo_stage0.rs`

*   **Input (Domain) Schema:**

    | Field Name   | Arrow Data Type |
    | :----------- | :-------------- |
    | `cargo_path` | `Utf8`          |

*   **Output (Range) Schema:** (None explicit - side effect: sets environment variable)

---

### Module: `bootstrap_stages::command_executor`

#### `capture_start_time.rs`

*   **Input (Domain) Schema:** (None)

*   **Output (Range) Schema:**

    | Field Name   | Arrow Data Type           |
    | :----------- | :------------------------ |
    | `start_time` | `Timestamp(Nanosecond, None)` |

#### `execute_shell_command.rs`

*   **Input (Domain) Schema:**

    | Field Name    | Arrow Data Type |
    | :------------ | :-------------- |
    | `command_str` | `Utf8`          |

*   **Output (Range) Schema:**

    | Field Name    | Arrow Data Type |
    | :------------ | :-------------- |
    | `status_code` | `Int32`         |
    | `stdout`      | `Binary`        |
    | `stderr`      | `Binary`        |

#### `capture_end_time_and_duration.rs`

*   **Input (Domain) Schema:**

    | Field Name   | Arrow Data Type           |
    | :----------- | :------------------------ |
    | `start_time` | `Timestamp(Nanosecond, None)` |

*   **Output (Range) Schema:**

    | Field Name | Arrow Data Type |
    | :--------- | :-------------- |
    | `duration` | `Int64`         |

#### `CommandExecutionResult` Struct Schema (Output of `create_command_execution_result.rs`)

| Field Name        | Arrow Data Type |
| :---------------- | :-------------- |
| `output.status_code` | `Int32`         |
| `output.stdout`   | `Binary`        |
| `output.stderr`   | `Binary`        |
| `duration`        | `Int64`         |

---

### Module: `parquet_reporter`

#### `prepare_record_batch_data.rs`

*   **Input (Domain) Schema:**

    | Field Name        | Arrow Data Type |
    | :---------------- | :-------------- |
    | `output.status_code` | `Int32`         |
    | `output.stdout`   | `Binary`        |
    | `output.stderr`   | `Binary`        |
    | `duration`        | `Int64`         |

*   **Output (Range) Schema (`PreparedBatchData` Struct):**

    | Field Name        | Arrow Data Type           |
    | :---------------- | :------------------------ |
    | `command_name`    | `Utf8`                    |
    | `command_status`  | `Utf8`                    |
    | `command_stdout`  | `Utf8`                    |
    | `command_stderr`  | `Utf8`                    |
    | `command_duration`| `Timestamp(Nanosecond, None)` |

#### `create_arrow_schema.rs`

*   **Input (Domain) Schema:** (None)

*   **Output (Range) Schema:** (Represents an Arrow `Schema` object itself, not a dataset of records)

    | Field Name          | Arrow Data Type           | Nullable |
    | :------------------ | :------------------------ | :------- |
    | `command_name`      | `Utf8`                    | No       |
    | `command_status`    | `Utf8`                    | No       |
    | `command_stdout`    | `Utf8`                    | No       |
    | `command_stderr`    | `Utf8`                    | No       |
    | `command_duration_ns` | `Timestamp(Nanosecond, None)` | No       |

#### `create_record_batch.rs`

*   **Input (Domain) Schema:**

    | Field Name        | Arrow Data Type           |
    | :---------------- | :------------------------ |
    | `schema`          | (Arrow `Schema` object)   |
    | `data`            | (from `PreparedBatchData` schema) |

*   **Output (Range) Schema:** (Represents an Arrow `RecordBatch` object itself)

    (Schema matches the output of `create_arrow_schema.rs`)

#### `write_to_parquet_file.rs`

*   **Input (Domain) Schema:**

    | Field Name  | Arrow Data Type           |
    | :---------- | :------------------------ |
    | `file_path` | `Utf8`                    |
    | `schema`    | (Arrow `Schema` object)   |
    | `batch`     | (Arrow `RecordBatch` object) |

*   **Output (Range) Schema:** (Represents an `ArrowWriter<File>` object - side effect: writes data to file)

#### `close_parquet_writer.rs`

*   **Input (Domain) Schema:**

    | Field Name | Arrow Data Type          |
    | :--------- | :----------------------- |
    | `writer`   | (`ArrowWriter<File>` object) |

*   **Output (Range) Schema:** (None - side effect: closes file)

#### `print_summary_header.rs`

*   **Input (Domain) Schema:** (None)
*   **Output (Range) Schema:** (None - pure side effect: prints to console)

#### `open_parquet_file.rs`

*   **Input (Domain) Schema:**

    | Field Name  | Arrow Data Type |
    | :---------- | :-------------- |
    | `file_path` | `Utf8`          |

*   **Output (Range) Schema:** (Represents a `File` handle)

#### `read_parquet_metadata.rs`

*   **Input (Domain) Schema:**

    | Field Name | Arrow Data Type |
    | :--------- | :-------------- |
    | `file`     | (`File` handle) |

*   **Output (Range) Schema:** (Represents a `ParquetMetaData` object)

#### `print_total_records.rs`

*   **Input (Domain) Schema:**

    | Field Name | Arrow Data Type |
    | :--------- | :-------------- |
    | `num_rows` | `Int64`         |

*   **Output (Range) Schema:** (None - pure side effect: prints to console)

#### `get_record_batch_reader.rs`

*   **Input (Domain) Schema:**

    | Field Name | Arrow Data Type |
    | :--------- | :-------------- |
    | `file`     | (`File` handle) |

*   **Output (Range) Schema:** (Represents an Arrow `RecordBatchReader` object)

#### `process_record_batches.rs`

*   **Input (Domain) Schema:**

    | Field Name     | Arrow Data Type          |
    | :------------- | :----------------------- |
    | `arrow_reader` | (Arrow `RecordBatchReader` object) |

*   **Output (Range) Schema:** (None - pure side effect: prints to console)

#### `print_summary_footer.rs`

*   **Input (Domain) Schema:** (None)
*   **Output (Range) Schema:** (None - pure side effect: prints to console)
