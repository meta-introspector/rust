use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

macro_rules! extract_string_codec {
    ($func_addr:expr, $strings:expr, $switch_patterns:expr) => {{
        println!("🔍 EXTRACTING STRING CODEC: 0x{:x}", $func_addr);
        StringCodec {
            function_address: $func_addr,
            string_domain: $strings,
            switch_patterns: $switch_patterns,
            enum_arithmetic: decode_enum_arithmetic(&$switch_patterns),
            codec_type: classify_codec_type(&$strings, &$switch_patterns),
        }
    }};
}

macro_rules! find_switch_functions {
    ($binary:expr, $elf:expr) => {{
        println!("🔍 FINDING SWITCH FUNCTIONS WITH STRING DOMAINS");
        let mut switch_functions = Vec::new();

        for sym in $elf.syms.iter().take(2000) {
            if let Some(name) = $elf.strtab.get_at(sym.st_name) {
                if sym.st_size > 50 && sym.st_value > 0 {
                    if let Some(codec) = analyze_function_for_codec($binary, $elf, &sym, name) {
                        switch_functions.push(codec);
                    }
                }
            }
        }

        switch_functions
    }};
}

macro_rules! decode_enum_arithmetic {
    ($patterns:expr) => {{
        let mut arithmetic = EnumArithmetic::new();

        for pattern in $patterns {
            // Extract numeric values from switch patterns
            if let Some(value) = extract_numeric_value(pattern) {
                arithmetic.add_case(value, pattern.clone());
            }
        }

        arithmetic
    }};
}

#[derive(Debug, Clone)]
struct StringCodec {
    function_address: u64,
    string_domain: Vec<String>,
    switch_patterns: Vec<String>,
    enum_arithmetic: EnumArithmetic,
    codec_type: CodecType,
}

#[derive(Debug, Clone)]
enum CodecType {
    StringToEnum,  // String input -> enum output
    EnumToString,  // Enum input -> string output
    Bidirectional, // Both directions
    Parser,        // Complex string parsing
    Formatter,     // String formatting
}

#[derive(Debug, Clone)]
struct EnumArithmetic {
    cases: HashMap<u32, String>,
    base_offset: u32,
    increment_pattern: u32,
    total_cases: usize,
}

impl EnumArithmetic {
    fn new() -> Self {
        Self { cases: HashMap::new(), base_offset: 0, increment_pattern: 1, total_cases: 0 }
    }

    fn add_case(&mut self, value: u32, pattern: String) {
        self.cases.insert(value, pattern);
        self.total_cases += 1;

        // Detect arithmetic patterns
        if self.total_cases == 1 {
            self.base_offset = value;
        } else if self.total_cases == 2 {
            let first_value = *self.cases.keys().min().unwrap();
            self.increment_pattern = value.saturating_sub(first_value);
        }
    }

    fn predict_next_case(&self) -> u32 {
        if let Some(&max_value) = self.cases.keys().max() {
            max_value + self.increment_pattern
        } else {
            self.base_offset
        }
    }
}

struct StringCodecExtractor {
    binary_path: String,
    discovered_codecs: Vec<StringCodec>,
}

impl StringCodecExtractor {
    fn new(binary_path: String) -> Self {
        Self { binary_path, discovered_codecs: Vec::new() }
    }

    fn extract_all_codecs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let binary = fs::read(&self.binary_path)?;
        let elf = Elf::parse(&binary)?;

        println!("🔍 Extracting string codecs from binary...");

        self.discovered_codecs = find_switch_functions!(&binary, &elf);

        println!("🎯 Discovered {} string codecs", self.discovered_codecs.len());
        Ok(())
    }

    fn generate_codec_enum(&self) -> String {
        let mut enum_code = String::new();

        enum_code.push_str("// Auto-generated string codecs\n");
        enum_code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        enum_code.push_str("pub enum StringCodecs {\n");

        for (i, codec) in self.discovered_codecs.iter().enumerate() {
            let codec_name = format!("Codec_{}", i);
            enum_code.push_str(&format!(
                "    {} = 0x{:x}, // {:?} with {} strings\n",
                codec_name,
                codec.function_address,
                codec.codec_type,
                codec.string_domain.len()
            ));
        }

        enum_code.push_str("}\n\n");

        // Generate codec implementations
        enum_code.push_str("impl StringCodecs {\n");
        enum_code.push_str("    pub fn address(&self) -> u64 { *self as u64 }\n");
        enum_code.push_str("    \n");
        enum_code.push_str("    pub fn decode_string(&self, input: &str) -> Option<u32> {\n");
        enum_code.push_str("        match self {\n");

        for (i, codec) in self.discovered_codecs.iter().enumerate() {
            let codec_name = format!("Codec_{}", i);
            enum_code.push_str(&format!("            StringCodecs::{} => {{\n", codec_name));

            // Generate string matching logic
            for (value, string) in &codec.enum_arithmetic.cases {
                let safe_string = string.replace("\"", "\\\"");
                enum_code.push_str(&format!(
                    "                if input == \"{}\" {{ return Some({}); }}\n",
                    safe_string, value
                ));
            }

            enum_code.push_str("                None\n");
            enum_code.push_str("            },\n");
        }

        enum_code.push_str("        }\n");
        enum_code.push_str("    }\n");
        enum_code.push_str("    \n");
        enum_code.push_str("    pub fn encode_enum(&self, value: u32) -> Option<&'static str> {\n");
        enum_code.push_str("        match self {\n");

        for (i, codec) in self.discovered_codecs.iter().enumerate() {
            let codec_name = format!("Codec_{}", i);
            enum_code.push_str(&format!("            StringCodecs::{} => {{\n", codec_name));
            enum_code.push_str("                match value {\n");

            for (value, string) in &codec.enum_arithmetic.cases {
                let safe_string = string.replace("\"", "\\\"");
                enum_code.push_str(&format!(
                    "                    {} => Some(\"{}\"),\n",
                    value, safe_string
                ));
            }

            enum_code.push_str("                    _ => None,\n");
            enum_code.push_str("                }\n");
            enum_code.push_str("            },\n");
        }

        enum_code.push_str("        }\n");
        enum_code.push_str("    }\n");
        enum_code.push_str("}\n");

        enum_code
    }

    fn print_codec_analysis(&self) {
        println!("\n📊 STRING CODEC ANALYSIS:");
        println!("=========================");

        let mut type_counts = HashMap::new();
        for codec in &self.discovered_codecs {
            let type_name = format!("{:?}", codec.codec_type);
            *type_counts.entry(type_name).or_insert(0) += 1;
        }

        for (codec_type, count) in &type_counts {
            println!("   {}: {} codecs", codec_type, count);
        }

        println!("\n🎯 TOP STRING CODECS:");
        for (i, codec) in self.discovered_codecs.iter().take(5).enumerate() {
            println!("   Codec {}: 0x{:x} ({:?})", i, codec.function_address, codec.codec_type);
            println!("      String domain: {} entries", codec.string_domain.len());
            println!("      Switch patterns: {} cases", codec.switch_patterns.len());
            println!(
                "      Enum arithmetic: base={}, increment={}",
                codec.enum_arithmetic.base_offset, codec.enum_arithmetic.increment_pattern
            );

            if !codec.string_domain.is_empty() {
                println!(
                    "      Sample strings: {:?}",
                    &codec.string_domain[..3.min(codec.string_domain.len())]
                );
            }
        }
    }
}

fn analyze_function_for_codec(
    binary: &[u8],
    elf: &Elf,
    sym: &goblin::elf::Sym,
    _name: &str,
) -> Option<StringCodec> {
    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")?;

    let func_start = (sym.st_value - text_section.sh_addr) as usize;
    let func_size = sym.st_size as usize;
    let text_start = text_section.sh_offset as usize;
    let text_bytes = &binary[text_start..text_start + text_section.sh_size as usize];

    if func_start + func_size > text_bytes.len() {
        return None;
    }

    let func_bytes = &text_bytes[func_start..func_start + func_size];

    // Look for switch patterns (simplified detection)
    let switch_count = func_bytes
        .chunks_exact(4)
        .filter(|chunk| {
            let opcode = chunk[0];
            opcode == 0x3c || opcode == 0x83 || // CMP instructions
            opcode == 0x74 || opcode == 0x75 // JE/JNE instructions
        })
        .count();

    // Look for string references (simplified)
    let string_refs = func_bytes
        .windows(4)
        .filter(|window| {
            // Look for patterns that might be string pointers
            let value = u32::from_le_bytes([window[0], window[1], window[2], window[3]]);
            value > 0x400000 && value < 0x800000 // Typical string address range
        })
        .count();

    // Only consider functions with both switch patterns and string references
    if switch_count >= 3 && string_refs >= 2 {
        let strings = extract_string_domain(func_bytes);
        let switch_patterns = extract_switch_patterns(func_bytes);

        if !strings.is_empty() && !switch_patterns.is_empty() {
            return Some(extract_string_codec!(
                sym.st_value,
                strings.clone(),
                switch_patterns.clone()
            ));
        }
    }

    None
}

fn extract_string_domain(func_bytes: &[u8]) -> Vec<String> {
    let mut strings = Vec::new();

    // Simplified string extraction - look for printable ASCII sequences
    let mut current_string = String::new();

    for &byte in func_bytes {
        if byte >= 32 && byte <= 126 {
            // Printable ASCII
            current_string.push(byte as char);
        } else {
            if current_string.len() >= 3 {
                strings.push(current_string.clone());
            }
            current_string.clear();
        }
    }

    // Add final string if valid
    if current_string.len() >= 3 {
        strings.push(current_string);
    }

    strings.into_iter().take(10).collect() // Limit to prevent overflow
}

fn extract_switch_patterns(func_bytes: &[u8]) -> Vec<String> {
    let mut patterns = Vec::new();

    for (i, chunk) in func_bytes.chunks_exact(4).enumerate() {
        let opcode = chunk[0];

        if opcode == 0x3c || opcode == 0x83 {
            // CMP instructions
            let immediate = chunk[1];
            patterns.push(format!("cmp_case_{}", immediate));
        } else if opcode == 0x74 || opcode == 0x75 {
            // JE/JNE
            patterns.push(format!("branch_case_{}", i));
        }
    }

    patterns.into_iter().take(20).collect() // Limit patterns
}

fn decode_enum_arithmetic(patterns: &[String]) -> EnumArithmetic {
    let mut arithmetic = EnumArithmetic::new();

    for (i, pattern) in patterns.iter().enumerate() {
        arithmetic.add_case(i as u32, pattern.clone());
    }

    arithmetic
}

fn extract_numeric_value(pattern: &str) -> Option<u32> {
    // Extract numeric values from pattern strings
    pattern.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().ok()
}

fn classify_codec_type(strings: &[String], patterns: &[String]) -> CodecType {
    let string_count = strings.len();
    let pattern_count = patterns.len();

    if string_count > pattern_count {
        CodecType::EnumToString
    } else if pattern_count > string_count {
        CodecType::StringToEnum
    } else if string_count > 5 && pattern_count > 5 {
        CodecType::Parser
    } else if string_count == pattern_count && string_count > 0 {
        CodecType::Bidirectional
    } else {
        CodecType::Formatter
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 STRING CODEC EXTRACTOR");
    println!("=========================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";

    let mut extractor = StringCodecExtractor::new(binary_path.to_string());

    // Extract all string codecs
    extractor.extract_all_codecs()?;

    // Generate codec enum
    let codec_enum = extractor.generate_codec_enum();
    fs::write("string_codecs.rs", &codec_enum)?;

    // Print analysis
    extractor.print_codec_analysis();

    println!("\n✅ STRING CODEC EXTRACTION COMPLETE:");
    println!("====================================");
    println!("   Discovered codecs: {}", extractor.discovered_codecs.len());
    println!("   💾 Saved codec enum to: string_codecs.rs");

    println!("\n🔍 CODEC CAPABILITIES:");
    println!("   • String domain extraction");
    println!("   • Switch pattern detection");
    println!("   • Enum arithmetic decoding");
    println!("   • Bidirectional string/enum conversion");
    println!("   • Automatic codec type classification");

    Ok(())
}
