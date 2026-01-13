use std::collections::{HashMap, BTreeMap};
use std::fs;
use serde::{Deserialize, Serialize};
use goblin::elf::Elf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionTrace {
    pub name: String,
    pub timestamp: u64,
    pub registers: RegisterState,
    pub memory_accesses: Vec<MemoryAccess>,
    pub call_depth: u32,
    pub return_value: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterState {
    pub rdi: u64, // First argument (System V ABI)
    pub rsi: u64, // Second argument
    pub rdx: u64, // Third argument
    pub rcx: u64, // Fourth argument
    pub r8: u64,  // Fifth argument
    pub r9: u64,  // Sixth argument
    pub rax: u64, // Return value
    pub rsp: u64, // Stack pointer
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAccess {
    pub address: u64,
    pub size: usize,
    pub access_type: AccessType,
    pub data_preview: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
    Execute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub args: Vec<ArgumentType>,
    pub return_type: ReturnType,
    pub calling_convention: CallingConvention,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgumentType {
    Pointer { target_type: Box<ArgumentType> },
    Integer { size: usize, signed: bool },
    Float { size: usize },
    Struct { size: usize, fields: Vec<ArgumentType> },
    String { encoding: StringEncoding },
    Unknown { size: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StringEncoding {
    Utf8,
    Ascii,
    CString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReturnType {
    Void,
    Value(ArgumentType),
    Pointer(Box<ArgumentType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallingConvention {
    SystemV,
    Windows,
    Rust,
    C,
}

pub struct ABIExtractor {
    traces: Vec<FunctionTrace>,
    memory_snapshots: BTreeMap<u64, Vec<u8>>,
    function_patterns: HashMap<String, Vec<FunctionTrace>>,
}

impl ABIExtractor {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            memory_snapshots: BTreeMap::new(),
            function_patterns: HashMap::new(),
        }
    }

    pub fn load_traces(&mut self, trace_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(trace_file)?;
        
        for line in content.lines() {
            if let Ok(trace) = serde_json::from_str::<FunctionTrace>(line) {
                self.function_patterns
                    .entry(trace.name.clone())
                    .or_insert_with(Vec::new)
                    .push(trace.clone());
                self.traces.push(trace);
            }
        }
        
        println!("Loaded {} traces for {} unique functions", 
                 self.traces.len(), 
                 self.function_patterns.len());
        Ok(())
    }

    pub fn infer_signature(&self, func_name: &str) -> Option<FunctionSignature> {
        let traces = self.function_patterns.get(func_name)?;
        if traces.is_empty() {
            return None;
        }

        println!("Analyzing {} traces for function: {}", traces.len(), func_name);

        let args = self.infer_arguments(traces);
        let return_type = self.infer_return_type(traces);
        let calling_conv = self.infer_calling_convention(traces);
        let confidence = self.calculate_confidence(traces, &args);

        Some(FunctionSignature {
            name: func_name.to_string(),
            args,
            return_type,
            calling_convention: calling_conv,
            confidence,
        })
    }

    fn infer_arguments(&self, traces: &[FunctionTrace]) -> Vec<ArgumentType> {
        let mut args = Vec::new();
        
        // Analyze register usage patterns
        let registers = [
            ("rdi", |r: &RegisterState| r.rdi),
            ("rsi", |r: &RegisterState| r.rsi),
            ("rdx", |r: &RegisterState| r.rdx),
            ("rcx", |r: &RegisterState| r.rcx),
            ("r8", |r: &RegisterState| r.r8),
            ("r9", |r: &RegisterState| r.r9),
        ];

        for (reg_name, reg_accessor) in registers.iter() {
            let values: Vec<u64> = traces.iter()
                .map(|t| reg_accessor(&t.registers))
                .collect();
            
            if let Some(arg_type) = self.analyze_argument_pattern(&values, traces) {
                println!("  {} -> {:?}", reg_name, arg_type);
                args.push(arg_type);
            } else {
                break; // No more arguments
            }
        }

        args
    }

    fn analyze_argument_pattern(&self, values: &[u64], traces: &[FunctionTrace]) -> Option<ArgumentType> {
        if values.iter().all(|&v| v == 0) {
            return None; // Unused register
        }

        // Check if values look like pointers
        if self.looks_like_pointers(values) {
            return Some(self.infer_pointer_type(values, traces));
        }

        // Check if values are small integers
        if values.iter().all(|&v| v < 0x10000) {
            return Some(ArgumentType::Integer { size: 8, signed: false });
        }

        // Check for string patterns
        if let Some(string_type) = self.check_string_patterns(values, traces) {
            return Some(string_type);
        }

        // Default to unknown
        Some(ArgumentType::Unknown { size: 8 })
    }

    fn looks_like_pointers(&self, values: &[u64]) -> bool {
        values.iter().any(|&v| {
            // Typical userspace address range
            v > 0x400000 && v < 0x800000000000
        })
    }

    fn infer_pointer_type(&self, values: &[u64], traces: &[FunctionTrace]) -> ArgumentType {
        // Try to determine what the pointer points to
        for &addr in values {
            if let Some(data) = self.get_memory_at(addr, traces) {
                if self.looks_like_string(&data) {
                    return ArgumentType::Pointer {
                        target_type: Box::new(ArgumentType::String {
                            encoding: StringEncoding::CString
                        })
                    };
                }
                
                if self.looks_like_struct(&data) {
                    return ArgumentType::Pointer {
                        target_type: Box::new(ArgumentType::Struct {
                            size: data.len(),
                            fields: vec![] // TODO: Infer struct fields
                        })
                    };
                }
            }
        }

        ArgumentType::Pointer {
            target_type: Box::new(ArgumentType::Unknown { size: 8 })
        }
    }

    fn check_string_patterns(&self, values: &[u64], traces: &[FunctionTrace]) -> Option<ArgumentType> {
        for &addr in values {
            if let Some(data) = self.get_memory_at(addr, traces) {
                if self.looks_like_string(&data) {
                    return Some(ArgumentType::String {
                        encoding: StringEncoding::CString
                    });
                }
            }
        }
        None
    }

    fn get_memory_at(&self, addr: u64, traces: &[FunctionTrace]) -> Option<Vec<u8>> {
        // Look for memory accesses at this address
        for trace in traces {
            for access in &trace.memory_accesses {
                if access.address == addr && !access.data_preview.is_empty() {
                    return Some(access.data_preview.clone());
                }
            }
        }
        None
    }

    fn looks_like_string(&self, data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }

        // Check for null termination
        if data.last() == Some(&0) {
            let string_part = &data[..data.len()-1];
            return string_part.iter().all(|&b| b.is_ascii_graphic() || b.is_ascii_whitespace());
        }

        // Check for UTF-8
        std::str::from_utf8(data).is_ok()
    }

    fn looks_like_struct(&self, data: &[u8]) -> bool {
        // Heuristic: structs often have aligned sizes
        data.len() % 8 == 0 && data.len() >= 16
    }

    fn infer_return_type(&self, traces: &[FunctionTrace]) -> ReturnType {
        let return_values: Vec<u64> = traces.iter()
            .filter_map(|t| t.return_value)
            .collect();

        if return_values.is_empty() {
            return ReturnType::Void;
        }

        if return_values.iter().all(|&v| v == 0) {
            return ReturnType::Void;
        }

        if self.looks_like_pointers(&return_values) {
            return ReturnType::Pointer(Box::new(ArgumentType::Unknown { size: 8 }));
        }

        ReturnType::Value(ArgumentType::Integer { size: 8, signed: false })
    }

    fn infer_calling_convention(&self, _traces: &[FunctionTrace]) -> CallingConvention {
        // For now, assume System V ABI (Linux x86_64)
        CallingConvention::SystemV
    }

    fn calculate_confidence(&self, traces: &[FunctionTrace], args: &[ArgumentType]) -> f64 {
        let base_confidence = if traces.len() >= 10 { 0.8 } else { 0.5 };
        
        // Increase confidence if we found consistent patterns
        let pattern_bonus = args.iter()
            .filter(|arg| !matches!(arg, ArgumentType::Unknown { .. }))
            .count() as f64 * 0.1;

        (base_confidence + pattern_bonus).min(1.0)
    }

    pub fn generate_rust_bindings(&self, signatures: &[FunctionSignature]) -> String {
        let mut output = String::new();
        
        output.push_str("// Auto-generated FFI bindings from ABI analysis\n");
        output.push_str("// Generated by zombie_driver2 ABI extractor\n\n");
        output.push_str("use std::os::raw::{c_void, c_char};\n\n");
        output.push_str("#[link(name = \"rustc_driver\")]\n");
        output.push_str("extern \"C\" {\n");

        for sig in signatures {
            if sig.confidence < 0.6 {
                output.push_str(&format!("    // Low confidence ({:.2}): {}\n", sig.confidence, sig.name));
                continue;
            }

            let args_str = sig.args.iter()
                .enumerate()
                .map(|(i, arg)| format!("arg{}: {}", i, self.type_to_rust_string(arg)))
                .collect::<Vec<_>>()
                .join(", ");

            let return_str = match &sig.return_type {
                ReturnType::Void => "()".to_string(),
                ReturnType::Value(t) => self.type_to_rust_string(t),
                ReturnType::Pointer(t) => format!("*const {}", self.type_to_rust_string(t)),
            };

            output.push_str(&format!(
                "    // Confidence: {:.2}\n    pub fn {}({}) -> {};\n\n",
                sig.confidence, sig.name, args_str, return_str
            ));
        }

        output.push_str("}\n");
        output
    }

    fn type_to_rust_string(&self, arg_type: &ArgumentType) -> String {
        match arg_type {
            ArgumentType::Pointer { target_type } => {
                format!("*const {}", self.type_to_rust_string(target_type))
            }
            ArgumentType::Integer { size: 8, signed: false } => "u64".to_string(),
            ArgumentType::Integer { size: 4, signed: false } => "u32".to_string(),
            ArgumentType::Integer { size: 8, signed: true } => "i64".to_string(),
            ArgumentType::Integer { size: 4, signed: true } => "i32".to_string(),
            ArgumentType::String { .. } => "*const c_char".to_string(),
            ArgumentType::Struct { .. } => "c_void".to_string(),
            ArgumentType::Float { size: 8 } => "f64".to_string(),
            ArgumentType::Float { size: 4 } => "f32".to_string(),
            ArgumentType::Unknown { .. } => "c_void".to_string(),
            _ => "c_void".to_string(),
        }
    }

    pub fn save_analysis(&self, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut signatures = Vec::new();
        
        for func_name in self.function_patterns.keys() {
            if let Some(sig) = self.infer_signature(func_name) {
                signatures.push(sig);
            }
        }

        // Sort by confidence
        signatures.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        let analysis = serde_json::json!({
            "total_functions": signatures.len(),
            "high_confidence": signatures.iter().filter(|s| s.confidence > 0.8).count(),
            "medium_confidence": signatures.iter().filter(|s| s.confidence > 0.6 && s.confidence <= 0.8).count(),
            "low_confidence": signatures.iter().filter(|s| s.confidence <= 0.6).count(),
            "signatures": signatures
        });

        fs::write(output_file, serde_json::to_string_pretty(&analysis)?)?;
        
        // Generate Rust bindings
        let bindings = self.generate_rust_bindings(&signatures);
        fs::write("generated_bindings.rs", bindings)?;

        println!("Analysis saved to: {}", output_file);
        println!("Rust bindings saved to: generated_bindings.rs");
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut extractor = ABIExtractor::new();
    
    // Load traces from interception
    if let Err(_) = extractor.load_traces("live_parser_capture.jsonl") {
        println!("No trace file found. Run the parser interception first.");
        return Ok(());
    }

    println!("=== ABI Signature Inference ===");
    
    // Analyze all discovered functions
    extractor.save_analysis("abi_analysis.json")?;
    
    println!("\n=== Summary ===");
    println!("✅ ABI analysis complete");
    println!("📄 Results: abi_analysis.json");
    println!("🦀 Bindings: generated_bindings.rs");
    println!("\nNext steps:");
    println!("  1. Review generated bindings for accuracy");
    println!("  2. Test bindings with actual rustc calls");
    println!("  3. Refine type inference based on results");
    
    Ok(())
}
