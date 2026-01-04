
// MINIMAL CONSTANT-ONLY COMPILER
// Generated from rustc subgraph extraction
// Lattice composition of constant-handling parts only

#[derive(Debug, Clone)]
enum ConstantValue {
    Integer(i64),
    String(String),
    Boolean(bool),
    Float(f64),
}


// Generated from: const builtin_macros_tests_not_support
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff_number_activities
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:2287 ~ rustc_data_structures[1ab4]::sync::parallel::par_slice::par_rec)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_TIMING
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_help
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_naked_attribute
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const EXIT_SIGNALED_SIGABRT
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SKIP_SERIALIZING
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const BITS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_START_BRK
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_bytes_invalid
fn validate_constant(input: &str) -> bool {
    input.parse::<i64>().is_ok() ||
    input == "true" || input == "false" ||
    (input.starts_with('"') && input.ends_with('"'))
}


// Generated from: DefId(0:18041 ~ rustc_query_impl[4791]::query_impl::constness::dynamic_query::{closure#2}::__CALLSITE::META)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_label1
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_coerce_pointee_too_many_pointees
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const BOUND
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_remove_raw_ident
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_unused_args
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_TIMING_TIMESTAMP
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const DEFAULT
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_clobber_outputs
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const OPTLEN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_byte_str
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B1200
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_explicit_register_name
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const NUM_MASK
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_CHILD_SUBREAPER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B500000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_SPECULATION_CTRL
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_MAP
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_bytes_non_u8
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_FP_MODE_FRE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_additional
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_MCE_KILL_EARLY
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B2500000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const TRY_FROM
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:163 ~ gix_testtools[ec58]::to_bstr_err)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_derive_from_wrong_target
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B19200
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_proc_macro
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff_mode_activity
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cfg_accessible_indeterminate
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_non_generic_pointee
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_assert_missing_comma
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_UNALIGN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_naked_functions_testing_attribute
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B150
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_ENDIAN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B4800
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_unused_arg
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_derive_path_args_value
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static HWCAP2
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:27932 ~ rustc_query_impl[4791]::query_impl::proc_macro_decls_static::dynamic_query::{closure#2}::__CALLSITE::META)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_TASK_PERF_EVENTS_DISABLE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_string_invalid
fn validate_constant(input: &str) -> bool {
    input.parse::<i64>().is_ok() ||
    input == "true" || input == "false" ||
    (input.starts_with('"') && input.ends_with('"'))
}


// Generated from: const NAME
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_pos_mismatch
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const VARIANT_IDENTIFIER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_use_positional
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B4000000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_attribute_not_supported
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_AUXV
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B1500000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_pure_combine
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_bytes_array
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B9600
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_second_label
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B576000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const infer_hidden_type
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_opt_already_provided
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_pure_no_output
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const GETTER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_TIMING_STATISTICAL
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B38400
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B230400
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_expected_comma_in_list
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SKIP_DESERIALIZING
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_PDEATHSIG
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const label
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_PDEATHSIG
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const WRITE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B3000000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_env_not_unicode
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_derive_from_usage_note
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_test_case_non_item
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const DIR_BITS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_EXE_FILE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cfg_accessible_multiple_paths
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const INITIAL_WRITER_CAPACITY
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_derive_macro_call
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SMALL_PATH_BUFFER_SIZE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_ENDIAN_BIG
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_DUMPABLE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B134
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_coerce_pointee_requires_one_generic
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const CLOCKFD
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_clobber_abi
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:12232 ~ clippy_lints[d854]::methods::manual_c_str_literals::is_c_str_function)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const DESERIALIZE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff_unknown_activity
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const GROUP_MASK
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:12236 ~ clippy_lints[d854]::methods::manual_c_str_literals::rewrite_as_cstr)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_SPECULATION_CTRL
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const ENOBUFS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static PAGE_SIZE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const BORROW
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SUID_DUMP_USER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cfg_accessible_has_args
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_derive_from_wrong_field_count
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B115200
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_c_str
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_expected_other
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_str_lit
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B3500000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_note2
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_TSC_SIGSEGV
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cfg_select_no_matches
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B2000000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B50
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B110
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_coerce_pointee_requires_transparent
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_ENV_START
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_FPEMU
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const EXIT_FAILURE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_unknown_trait
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_trace_macros
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_DUMPABLE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_bad_derive_target
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SERDE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_takes_no_arguments
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_TSC
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const CONTENT
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff_missing_config
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const RENAME_ALL
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_bytes_missing_literal
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SPEC_L1D_FLUSH
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_reorder_format_parameter
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_explicit
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const infer_opaque_hidden_type
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_non_exhaustive_default
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B1800
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const GROUP_BITS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const DIR_MASK
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_FPEMU
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const FIELD_IDENTIFIER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff_ty_activity
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_assert_requires_expression
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_ARG_END
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_unsupported_option
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const OTHER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_IO_FLUSHER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_ARG_START
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B0
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_named
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SERIALIZE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const BUF_LEN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_export_macro_rules
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const infer_opaque_type
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_bytes_oob
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_label2
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_non_unit_default
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const WITH
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const help
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_cfg
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_test_runner_nargs
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const REMOTE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_proc_macro_attribute_only_usable_with_crate_type
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_other
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_BRK
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const RENAME_ALL_FIELDS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_IO_FLUSHER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_only_one_argument
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_bench_sig
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const DENY_UNKNOWN_FIELDS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_VMA
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff_width
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_mutually_exclusive
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SUID_DUMP_DISABLE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:6275 ~ clippy_lints[d854]::inherent_to_string::show_lint)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B75
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_test_bad_fn
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_positional_after_named
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const CUTOFF
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_FPEXC
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_MAP_SIZE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_PAC_GET_ENABLED_KEYS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_note
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const TRANSPARENT
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const EXPECTING
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B57600
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SUID_DUMP_ROOT
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_FP_MODE_FR
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B300
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_pos_after
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_TASK_PERF_EVENTS_ENABLE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_alloc_must_statics
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_no_default_variant
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_START_STACK
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_FP_MODE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_arg
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B200
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static DEFAULT_LOCALE_RESOURCE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const FROM
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cfg_select_unreachable
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B1000000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_FP_MODE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:8875 ~ rustc_trait_selection[c701]::solve::normalize::{impl#1}::try_fold_const::__CALLSITE::META)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_env_takes_args
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_coerce_pointee_requires_one_field
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B600
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_MCE_KILL
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_test_runner_invalid
fn validate_constant(input: &str) -> bool {
    input.parse::<i64>().is_ok() ||
    input == "true" || input == "false" ||
    (input.starts_with('"') && input.ends_with('"'))
}


// Generated from: const SKIP_SERIALIZING_IF
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_noreturn
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_PAC_SET_ENABLED_KEYS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const U64_MAX_STR_LEN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const EXIT_SUCCESS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff_not_build
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:8889 ~ rustc_trait_selection[c701]::solve::normalize::{impl#1}::try_fold_const::__CALLSITE#1::META)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static HWCAP
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_unexpected_lit
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_MCE_KILL_DEFAULT
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static RENAME_RULES
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SIZE_MASK
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SPEC_STORE_BYPASS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const REPR
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const _
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const note
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static _DECLS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const UNTAGGED
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const suggestion
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const warn
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_duplicate_arg
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cannot_derive_union
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const ALIAS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_multiple_defaults
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static MINSIGSTKSZ
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_number_array
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_TSC_ENABLE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_missing_literal
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_env_not_defined
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_duplicate_arg
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: rustc_data_structures
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_c_str_note
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static STATX_STATE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cfg_accessible_literal_path
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_label
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_expected_one_cfg_pattern
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_GET_ENDIAN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_named_args
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B921600
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SIZE_BITS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:2327 ~ rustc_borrowck[5558]::polonius::dump::emit_mermaid_constraint_graph)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_TSC
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const infer_label
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_ENDIAN_PPC_LITTLE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_coerce_pointee_requires_one_pointee
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:15924 ~ rustc_query_impl[4791]::query_impl::thir_abstract_const::get_query_incr::__rust_end_short_backtrace::__CALLSITE::META)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_custom
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_requires_cfg_pattern
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_byte_char
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const TAG
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_START_DATA
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: DefId(0:1086 ~ rustc_pattern_analysis[c1a8]::errors::verify_pattern_analysis_mixed_deref_pattern_constructors_2#1)
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_assert_requires_boolean
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const INTO
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const FLATTEN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const CRATE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_TIMING
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_MCE_KILL_LATE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_ENV_END
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_MCE_KILL_CLEAR
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_autodiff_ret_activity
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_UNALIGN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const NUM_BITS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_PTRACER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const INITIAL_READER_CAPACITY
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const I64_MAX_STR_LEN
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_no_matched_argument_name
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_END_CODE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B460800
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_pos
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_modifier_invalid
fn validate_constant(input: &str) -> bool {
    input.parse::<i64>().is_ok() ||
    input == "true" || input == "false" ||
    (input.starts_with('"') && input.ends_with('"'))
}


// Generated from: const builtin_macros_multiple_default_attrs
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_CHILD_SUBREAPER
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const RENAME
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cargo
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_c_str_lit
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_ENDIAN_LITTLE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_requires_string
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_alloc_error_must_be_fn
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const NUM_SHIFT
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_coerce_pointee_requires_maybe_sized
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_label_again
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_bytes_bad_repeat
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_cfg_accessible_unspecified_path
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B2400
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SKIP
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_suggestion
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static META
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_no_arg_named
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SPEC_INDIRECT_BRANCH
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_unsupported_clobber_abi
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_concat_bytestr
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const NONE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_mayunwind
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const SERIALIZE_WITH
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_source_uitls_expected_item
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const DESERIALIZE_WITH
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const AVERAGE_NUM_ATTRS
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const NON_EXHAUSTIVE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_default_arg
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_format_redundant_args
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_AUXV
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_FPEXC
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_END_DATA
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: static CLOCK_TICKS_PER_SECOND
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const B1152000
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_asm_clobber_no_reg
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_MCE_KILL_GET
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const READ
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_MM_START_CODE
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_SET_VMA_ANON_NAME
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const PR_MCE_KILL_SET
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}


// Generated from: const builtin_macros_derive_path_args_list
fn parse_constant(input: &str) -> Option<ConstantValue> {
    if let Ok(i) = input.parse::<i64>() {
        Some(ConstantValue::Integer(i))
    } else if input == "true" || input == "false" {
        Some(ConstantValue::Boolean(input == "true"))
    } else if input.starts_with('"') && input.ends_with('"') {
        Some(ConstantValue::String(input[1..input.len()-1].to_string()))
    } else {
        None
    }
}

