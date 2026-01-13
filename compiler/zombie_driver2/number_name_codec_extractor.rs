use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

macro_rules! extract_number_to_name {
    ($constants:expr, $strings:expr, $execution_path:expr) => {{
        println!("🔢 EXTRACTING NUMBER-TO-NAME MAPPINGS");
        let mut mappings = HashMap::new();

        for (i, constant) in $constants.iter().enumerate() {
            if i < $strings.len() {
                mappings.insert(*constant, $strings[i].clone());
            }
        }

        NumberToNameMapping {
            constants: $constants,
            strings: $strings,
            execution_path: $execution_path,
            mappings,
            modulo_function: derive_modulo_function(&$constants, &$strings),
        }
    }};
}

macro_rules! derive_codec_function {
    ($mappings:expr) => {{
        println!("🧮 DERIVING CODEC FUNCTION f(input) mod n = output");

        let constants: Vec<u32> = $mappings.keys().cloned().collect();
        let strings: Vec<String> = $mappings.values().cloned().collect();

        // Find modulo base that maps constants to string indices
        let modulo_base = find_optimal_modulo(&constants, strings.len());

        CodecFunction {
            modulo_base,
            input_constants: constants,
            output_strings: strings,
            function_type: if modulo_base > 0 {
                FunctionType::Modulo
            } else {
                FunctionType::Direct
            },
        }
    }};
}

macro_rules! tokenize_strings {
    ($strings:expr, method: $method:expr) => {{
        println!("🎯 TOKENIZING STRINGS: {}", $method);

        match $method {
            "hash" => tokenize_by_hash($strings),
            "length" => tokenize_by_length($strings),
            "ascii" => tokenize_by_ascii($strings),
            "prefix" => tokenize_by_prefix($strings),
            _ => tokenize_by_hash($strings),
        }
    }};
}

#[derive(Debug, Clone)]
struct NumberToNameMapping {
    constants: Vec<u32>,
    strings: Vec<String>,
    execution_path: Vec<u64>,
    mappings: HashMap<u32, String>,
    modulo_function: Option<ModuloFunction>,
}

#[derive(Debug, Clone)]
struct ModuloFunction {
    base: u32,
    offset: u32,
    multiplier: u32,
}

#[derive(Debug, Clone)]
struct CodecFunction {
    modulo_base: u32,
    input_constants: Vec<u32>,
    output_strings: Vec<String>,
    function_type: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
enum FunctionType {
    Direct, // Direct mapping
    Modulo, // f(x) mod n
    Hash,   // Hash-based mapping
    Linear, // Linear transformation
}

#[derive(Debug, Clone)]
struct StringToken {
    original: String,
    token: u32,
    method: String,
}

struct NumberNameExtractor {
    binary_path: String,
    discovered_mappings: Vec<NumberToNameMapping>,
    codec_functions: Vec<CodecFunction>,
    string_tokens: Vec<StringToken>,
}

impl NumberNameExtractor {
    fn new(binary_path: String) -> Self {
        Self {
            binary_path,
            discovered_mappings: Vec::new(),
            codec_functions: Vec::new(),
            string_tokens: Vec::new(),
        }
    }

    fn extract_all_mappings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let binary = fs::read(&self.binary_path)?;
        let elf = Elf::parse(&binary)?;

        println!("🔍 Extracting number-to-name mappings from execution paths...");

        // Find functions with both constants and strings
        for sym in elf.syms.iter().take(1000) {
            if let Some(_name) = elf.strtab.get_at(sym.st_name) {
                if sym.st_size > 100 && sym.st_value > 0 {
                    if let Some(mapping) = self.analyze_function_for_mapping(&binary, &elf, &sym)? {
                        self.discovered_mappings.push(mapping);
                    }
                }
            }
        }

        println!("🎯 Discovered {} number-to-name mappings", self.discovered_mappings.len());

        // Derive codec functions
        for mapping in &self.discovered_mappings {
            let codec = derive_codec_function!(mapping.mappings);
            self.codec_functions.push(codec);
        }

        // Tokenize all strings
        let all_strings: Vec<String> =
            self.discovered_mappings.iter().flat_map(|m| m.strings.clone()).collect();

        self.string_tokens = tokenize_strings!(&all_strings, method: "hash");

        Ok(())
    }

    fn analyze_function_for_mapping(
        &self,
        binary: &[u8],
        elf: &Elf,
        sym: &goblin::elf::Sym,
    ) -> Result<Option<NumberToNameMapping>, Box<dyn std::error::Error>> {
        let text_section = elf
            .section_headers
            .iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
            .ok_or("No .text section found")?;

        let func_start = (sym.st_value - text_section.sh_addr) as usize;
        let func_size = sym.st_size as usize;
        let text_start = text_section.sh_offset as usize;
        let text_bytes = &binary[text_start..text_start + text_section.sh_size as usize];

        if func_start + func_size > text_bytes.len() {
            return Ok(None);
        }

        let func_bytes = &text_bytes[func_start..func_start + func_size];

        // Extract constants from function
        let constants = self.extract_constants(func_bytes);

        // Extract strings from function
        let strings = self.extract_strings(func_bytes);

        // Create execution path (simplified)
        let execution_path = self.trace_execution_path(func_bytes, sym.st_value);

        // Only create mapping if we have both constants and strings
        if !constants.is_empty() && !strings.is_empty() && constants.len() >= strings.len() {
            let mapping =
                extract_number_to_name!(constants.clone(), strings.clone(), execution_path);
            return Ok(Some(mapping));
        }

        Ok(None)
    }

    fn extract_constants(&self, func_bytes: &[u8]) -> Vec<u32> {
        let mut constants = Vec::new();

        // Look for 4-byte constants in the function
        for chunk in func_bytes.chunks_exact(4) {
            let value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);

            // Filter for reasonable constants (not addresses, not too large)
            if value > 0 && value < 100000 && value != 0xFFFFFFFF {
                constants.push(value);
            }
        }

        // Remove duplicates and sort
        constants.sort();
        constants.dedup();
        constants.into_iter().take(20).collect() // Limit to prevent overflow
    }

    fn extract_strings(&self, func_bytes: &[u8]) -> Vec<String> {
        let mut strings = Vec::new();
        let mut current_string = String::new();

        // Extract printable ASCII strings
        for &byte in func_bytes {
            if byte >= 32 && byte <= 126 {
                // Printable ASCII
                current_string.push(byte as char);
            } else {
                if current_string.len() >= 2 {
                    strings.push(current_string.clone());
                }
                current_string.clear();
            }
        }

        // Add final string if valid
        if current_string.len() >= 2 {
            strings.push(current_string);
        }

        strings.into_iter().take(10).collect() // Limit strings
    }

    fn trace_execution_path(&self, func_bytes: &[u8], base_addr: u64) -> Vec<u64> {
        let mut path = Vec::new();

        // Simplified execution path tracing
        for (i, chunk) in func_bytes.chunks_exact(4).enumerate() {
            let opcode = chunk[0];

            // Track control flow instructions
            if opcode == 0xe8 || opcode == 0xff || // CALL
               opcode == 0x74 || opcode == 0x75 || // JE/JNE
               opcode == 0xeb
            {
                // JMP
                path.push(base_addr + (i * 4) as u64);
            }
        }

        path.into_iter().take(10).collect() // Limit path length
    }

    fn generate_codec_enum(&self) -> String {
        let mut enum_code = String::new();

        enum_code.push_str("// Auto-generated number-to-name codecs\n");
        enum_code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        enum_code.push_str("pub enum NumberNameCodecs {\n");

        for (i, codec) in self.codec_functions.iter().enumerate() {
            enum_code.push_str(&format!(
                "    Codec_{} = {}, // {:?} with {} mappings\n",
                i,
                codec.modulo_base,
                codec.function_type,
                codec.input_constants.len()
            ));
        }

        enum_code.push_str("}\n\n");

        // Generate codec implementation
        enum_code.push_str("impl NumberNameCodecs {\n");
        enum_code
            .push_str("    pub fn decode_number(&self, input: u32) -> Option<&'static str> {\n");
        enum_code.push_str("        match self {\n");

        for (i, codec) in self.codec_functions.iter().enumerate() {
            enum_code.push_str(&format!("            NumberNameCodecs::Codec_{} => {{\n", i));

            if codec.function_type == FunctionType::Modulo && codec.modulo_base > 0 {
                enum_code.push_str(&format!(
                    "                let index = (input % {}) as usize;\n",
                    codec.modulo_base
                ));
                enum_code.push_str("                match index {\n");

                for (j, string) in codec.output_strings.iter().enumerate() {
                    let safe_string = string.replace("\"", "\\\"");
                    enum_code.push_str(&format!(
                        "                    {} => Some(\"{}\"),\n",
                        j, safe_string
                    ));
                }

                enum_code.push_str("                    _ => None,\n");
                enum_code.push_str("                }\n");
            } else {
                // Direct mapping
                for (constant, string) in codec.input_constants.iter().zip(&codec.output_strings) {
                    let safe_string = string.replace("\"", "\\\"");
                    enum_code.push_str(&format!(
                        "                if input == {} {{ return Some(\"{}\"); }}\n",
                        constant, safe_string
                    ));
                }
                enum_code.push_str("                None\n");
            }

            enum_code.push_str("            },\n");
        }

        enum_code.push_str("        }\n");
        enum_code.push_str("    }\n");
        enum_code.push_str("}\n");

        enum_code
    }

    fn print_analysis(&self) {
        println!("\n📊 NUMBER-TO-NAME ANALYSIS:");
        println!("============================");

        println!("   Discovered mappings: {}", self.discovered_mappings.len());
        println!("   Codec functions: {}", self.codec_functions.len());
        println!("   String tokens: {}", self.string_tokens.len());

        println!("\n🔢 TOP MAPPINGS:");
        for (i, mapping) in self.discovered_mappings.iter().take(3).enumerate() {
            println!(
                "   Mapping {}: {} constants -> {} strings",
                i,
                mapping.constants.len(),
                mapping.strings.len()
            );

            if let Some(ref modulo_func) = mapping.modulo_function {
                println!(
                    "      Modulo function: f(x) = (x * {} + {}) mod {}",
                    modulo_func.multiplier, modulo_func.offset, modulo_func.base
                );
            }

            // Show sample mappings
            for (constant, string) in mapping.mappings.iter().take(3) {
                println!("      {} -> \"{}\"", constant, string);
            }
        }

        println!("\n🧮 CODEC FUNCTIONS:");
        for (i, codec) in self.codec_functions.iter().take(3).enumerate() {
            println!("   Codec {}: {:?}", i, codec.function_type);
            if codec.function_type == FunctionType::Modulo {
                println!("      f(input) mod {} = output_index", codec.modulo_base);
            }
            println!(
                "      {} inputs -> {} outputs",
                codec.input_constants.len(),
                codec.output_strings.len()
            );
        }

        println!("\n🎯 STRING TOKENS:");
        for token in self.string_tokens.iter().take(5) {
            println!("   \"{}\" -> {} ({})", token.original, token.token, token.method);
        }
    }
}

fn derive_modulo_function(constants: &[u32], strings: &[String]) -> Option<ModuloFunction> {
    if constants.is_empty() || strings.is_empty() {
        return None;
    }

    // Try to find a modulo base that maps constants to string indices
    let base = find_optimal_modulo(constants, strings.len());

    if base > 0 { Some(ModuloFunction { base, offset: 0, multiplier: 1 }) } else { None }
}

fn find_optimal_modulo(constants: &[u32], string_count: usize) -> u32 {
    // Try different modulo bases to find one that maps well
    for base in 2..=string_count as u32 * 2 {
        let mut indices = Vec::new();
        for &constant in constants {
            indices.push((constant % base) as usize);
        }

        // Check if indices are within string range and reasonably distributed
        if indices.iter().all(|&i| i < string_count) {
            let unique_indices: std::collections::HashSet<_> = indices.iter().collect();
            if unique_indices.len() >= string_count.min(constants.len()) / 2 {
                return base;
            }
        }
    }

    0 // No good modulo found
}

fn tokenize_by_hash(strings: &[String]) -> Vec<StringToken> {
    strings
        .iter()
        .map(|s| {
            let mut hash = 0u32;
            for byte in s.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
            }

            StringToken { original: s.clone(), token: hash, method: "hash".to_string() }
        })
        .collect()
}

fn tokenize_by_length(strings: &[String]) -> Vec<StringToken> {
    strings
        .iter()
        .map(|s| StringToken {
            original: s.clone(),
            token: s.len() as u32,
            method: "length".to_string(),
        })
        .collect()
}

fn tokenize_by_ascii(strings: &[String]) -> Vec<StringToken> {
    strings
        .iter()
        .map(|s| {
            let ascii_sum: u32 = s.bytes().map(|b| b as u32).sum();
            StringToken { original: s.clone(), token: ascii_sum, method: "ascii".to_string() }
        })
        .collect()
}

fn tokenize_by_prefix(strings: &[String]) -> Vec<StringToken> {
    strings
        .iter()
        .map(|s| {
            let prefix = s.chars().take(4).collect::<String>();
            let mut token = 0u32;
            for (i, ch) in prefix.chars().enumerate() {
                token |= ((ch as u32) & 0xFF) << (i * 8);
            }

            StringToken { original: s.clone(), token, method: "prefix".to_string() }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 NUMBER-TO-NAME CODEC EXTRACTOR");
    println!("==================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";

    let mut extractor = NumberNameExtractor::new(binary_path.to_string());

    // Extract all number-to-name mappings
    extractor.extract_all_mappings()?;

    // Generate codec enum
    let codec_enum = extractor.generate_codec_enum();
    fs::write("number_name_codecs.rs", &codec_enum)?;

    // Print analysis
    extractor.print_analysis();

    println!("\n✅ NUMBER-TO-NAME EXTRACTION COMPLETE:");
    println!("======================================");
    println!("   Discovered mappings: {}", extractor.discovered_mappings.len());
    println!("   Codec functions: {}", extractor.codec_functions.len());
    println!("   String tokens: {}", extractor.string_tokens.len());
    println!("   💾 Saved codec enum to: number_name_codecs.rs");

    println!("\n🧮 CODEC CAPABILITIES:");
    println!("   • Extract constants from execution paths");
    println!("   • Map numbers to string names");
    println!("   • Derive modulo functions f(input) mod n = output");
    println!("   • Tokenize strings for later processing");
    println!("   • Generate bidirectional codec enums");

    Ok(())
}
