use std::process::{Command, Stdio};
use std::fs;
use std::collections::HashMap;
use serde_json::json;

pub struct EnhancedTracer {
    output_dir: String,
}

impl EnhancedTracer {
    pub fn new() -> Self {
        Self {
            output_dir: "enhanced_traces".to_string(),
        }
    }

    pub fn trace_with_registers(&self, source_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.output_dir)?;
        
        println!("🔍 Enhanced tracing with register capture...");
        
        // Use perf to capture register states
        let perf_data = format!("{}/enhanced_perf.data", self.output_dir);
        
        let status = Command::new("perf")
            .args(&[
                "record",
                "-e", "cycles:u",
                "-g", "--call-graph", "dwarf",
                "--user-regs", "AX,BX,CX,DX,SI,DI,SP,BP,R8,R9,R10,R11,R12,R13,R14,R15",
                "-F", "999",
                "-o", &perf_data,
                "--",
                "rustc",
                source_file,
                "--emit=llvm-ir",
                "-O",
            ])
            .status()?;
        
        if !status.success() {
            return Err("Enhanced perf record failed".into());
        }

        // Extract detailed script with register info
        let script_output = Command::new("perf")
            .args(&[
                "script",
                "-i", &perf_data,
                "--fields", "comm,pid,tid,time,event,ip,sym,iregs",
            ])
            .output()?;

        let script_content = String::from_utf8(script_output.stdout)?;
        fs::write(format!("{}/enhanced_script.txt", self.output_dir), &script_content)?;

        // Parse and convert to structured format
        self.parse_enhanced_script(&script_content)?;
        
        Ok(())
    }

    fn parse_enhanced_script(&self, script: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut traces = Vec::new();
        let mut call_depth = 0u32;

        for line in script.lines() {
            if let Some(trace) = self.parse_perf_line(line, &mut call_depth) {
                traces.push(trace);
            }
        }

        // Save as JSONL for ABI extractor
        let trace_file = format!("{}/function_traces.jsonl", self.output_dir);
        let mut output = String::new();
        
        for trace in traces {
            output.push_str(&serde_json::to_string(&trace)?);
            output.push('\n');
        }
        
        fs::write(trace_file, output)?;
        println!("✅ Enhanced traces saved: {}/function_traces.jsonl", self.output_dir);
        
        Ok(())
    }

    fn parse_perf_line(&self, line: &str, call_depth: &mut u32) -> Option<serde_json::Value> {
        // Parse perf script line with register info
        // Format: rustc 12345 [000] 123.456: cycles:u: 7f1234abcd symbol AX:0x123 BX:0x456 ...
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }

        let timestamp = parts[3].trim_end_matches(':').parse::<f64>().ok()? as u64;
        let symbol = parts.get(5)?.to_string();

        // Extract register values
        let mut registers = HashMap::new();
        for part in &parts[6..] {
            if let Some((reg, val)) = part.split_once(':') {
                if let Ok(value) = u64::from_str_radix(val.trim_start_matches("0x"), 16) {
                    registers.insert(reg.to_string(), value);
                }
            }
        }

        // Simulate memory accesses (in real implementation, use Intel PT or similar)
        let memory_accesses = vec![
            json!({
                "address": registers.get("DI").unwrap_or(&0),
                "size": 8,
                "access_type": "Read",
                "data_preview": []
            })
        ];

        Some(json!({
            "name": symbol,
            "timestamp": timestamp,
            "registers": {
                "rdi": registers.get("DI").unwrap_or(&0),
                "rsi": registers.get("SI").unwrap_or(&0),
                "rdx": registers.get("DX").unwrap_or(&0),
                "rcx": registers.get("CX").unwrap_or(&0),
                "r8": registers.get("R8").unwrap_or(&0),
                "r9": registers.get("R9").unwrap_or(&0),
                "rax": registers.get("AX").unwrap_or(&0),
                "rsp": registers.get("SP").unwrap_or(&0)
            },
            "memory_accesses": memory_accesses,
            "call_depth": *call_depth,
            "return_value": None
        }))
    }

    pub fn trace_with_gdb(&self, source_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🐛 GDB-based tracing for detailed ABI capture...");
        
        // Create GDB script for automated tracing
        let gdb_script = format!("{}/trace_script.gdb", self.output_dir);
        fs::write(&gdb_script, r#"
set logging file gdb_trace.log
set logging on
set pagination off

# Break on main rustc functions
break rustc_driver::main
break rustc_parse
break rustc_expand
break rustc_resolve

# Define function to log registers and memory
define log_call
    printf "CALL: %s\n", $arg0
    printf "RDI: 0x%lx RSI: 0x%lx RDX: 0x%lx\n", $rdi, $rsi, $rdx
    printf "RCX: 0x%lx R8: 0x%lx R9: 0x%lx\n", $rcx, $r8, $r9
    if $rdi != 0
        printf "MEM[RDI]: "
        x/8bx $rdi
    end
    if $rsi != 0
        printf "MEM[RSI]: "
        x/8bx $rsi
    end
    continue
end

# Set up breakpoint commands
commands 1
    log_call "rustc_driver::main"
end

commands 2
    log_call "rustc_parse"
end

commands 3
    log_call "rustc_expand"
end

commands 4
    log_call "rustc_resolve"
end

run
quit
"#)?;

        // Run rustc under GDB
        let status = Command::new("gdb")
            .args(&[
                "--batch",
                "--command", &gdb_script,
                "--args",
                "rustc", source_file,
            ])
            .current_dir(&self.output_dir)
            .status()?;

        if status.success() {
            println!("✅ GDB trace completed: {}/gdb_trace.log", self.output_dir);
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tracer = EnhancedTracer::new();
    
    // Create test file
    if !std::path::Path::new("test_abi.rs").exists() {
        fs::write("test_abi.rs", r#"
use std::collections::HashMap;

struct Parser {
    tokens: Vec<String>,
    position: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            tokens: input.split_whitespace().map(|s| s.to_string()).collect(),
            position: 0,
        }
    }
    
    fn parse_expression(&mut self) -> Option<String> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
}

fn main() {
    let mut parser = Parser::new("hello world rust");
    
    while let Some(expr) = parser.parse_expression() {
        println!("Parsed: {}", expr);
    }
    
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("Map: {:?}", map);
}
"#)?;
    }

    println!("=== Enhanced ABI Tracing ===");
    
    // Method 1: Enhanced perf with register capture
    if let Err(e) = tracer.trace_with_registers("test_abi.rs") {
        println!("⚠️  Enhanced perf failed: {}", e);
        println!("💡 Trying alternative methods...");
    }

    // Method 2: GDB-based tracing
    if let Err(e) = tracer.trace_with_gdb("test_abi.rs") {
        println!("⚠️  GDB tracing failed: {}", e);
    }

    println!("\n=== Running ABI Analysis ===");
    
    // Run the ABI extractor on collected traces
    let status = Command::new("cargo")
        .args(&["run", "--bin", "abi_signature_extractor"])
        .status()?;

    if status.success() {
        println!("✅ ABI analysis complete!");
        println!("📄 Check generated_bindings.rs for auto-generated FFI");
    }

    Ok(())
}
