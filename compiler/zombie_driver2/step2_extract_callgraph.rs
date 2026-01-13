use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 STEP 2: EXTRACT PARSER CALL GRAPH");
    println!("====================================");

    let mut parser_functions = HashSet::new();

    // Try to read from rustc output log first
    if let Ok(rustc_log) = fs::read_to_string("rustc_output.log") {
        println!("🔍 Analyzing rustc self-profile output...");

        // Look for function names in the output
        for line in rustc_log.lines() {
            if line.contains("parse") || line.contains("Parse") {
                // Extract function names from the line
                if let Some(func_name) = extract_function_name(line) {
                    parser_functions.insert(func_name);
                }
            }
        }
    }

    // Try to read from perf trace
    if let Ok(perf_trace) = fs::read_to_string("perf_trace.txt") {
        println!("🔍 Analyzing perf trace output...");

        for line in perf_trace.lines() {
            // Look for rustc parser function calls in perf output
            if line.contains("rustc") && (line.contains("parse") || line.contains("Parse")) {
                if let Some(func_name) = extract_perf_function_name(line) {
                    parser_functions.insert(func_name);
                }
            }
        }
    }

    // Add known parser functions from rustc source
    let known_parser_functions = vec![
        "rustc_parse::parser::Parser::parse_crate_mod",
        "rustc_parse::parser::Parser::parse_item",
        "rustc_parse::parser::Parser::parse_stmt",
        "rustc_parse::parser::Parser::parse_expr",
        "rustc_parse::parser::Parser::parse_block",
        "rustc_parse::parser::Parser::parse_fn",
        "rustc_parse::lexer::StringReader::next_token",
        "rustc_ast::tokenstream::TokenStream::parse",
        "rustc_driver_impl::parse_crate_attrs",
    ];

    for func in known_parser_functions {
        parser_functions.insert(func.to_string());
    }

    println!("📋 Found {} parser functions:", parser_functions.len());

    // Write target function list
    let target_list: Vec<String> = Vec::from_iter(parser_functions);
    let mut sorted_list = target_list;
    sorted_list.sort();

    // Simple JSON serialization without serde_json
    let mut json_content = String::from("[\n");
    for (i, func) in sorted_list.iter().enumerate() {
        json_content.push_str(&format!("  \"{}\"", func));
        if i < sorted_list.len() - 1 {
            json_content.push(',');
        }
        json_content.push('\n');
    }
    json_content.push_str("]\n");

    fs::write("parser_target_functions.json", json_content)?;

    for (i, func) in sorted_list.iter().enumerate() {
        println!("  {}: {}", i + 1, func);
    }

    println!("✅ Target function list saved to parser_target_functions.json");

    Ok(())
}

fn extract_function_name(line: &str) -> Option<String> {
    // Extract function names from rustc self-profile output
    // This is a simple heuristic - adjust based on actual output format
    if let Some(start) = line.find("rustc_parse::") {
        if let Some(end) = line[start..].find(' ') {
            return Some(line[start..start + end].to_string());
        }
    }
    None
}

fn extract_perf_function_name(line: &str) -> Option<String> {
    // Extract function names from perf script output
    // Perf format: timestamp process [address] function_name
    let parts: Vec<&str> = line.split_whitespace().collect();
    for part in parts {
        if part.contains("rustc_parse::") || part.contains("parse") {
            return Some(part.to_string());
        }
    }
    None
}
