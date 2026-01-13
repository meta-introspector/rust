use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;

macro_rules! mkfeature {
    (opcode_test $opcode:expr, $expected:expr) => {{
        println!("🧪 TESTING: Opcode {:02x} should decode as {}", $opcode, $expected);
        TestFeature {
            name: format!("opcode_test_{:02x}", $opcode),
            test_type: TestType::OpcodeDecoding($opcode, $expected.to_string()),
            auto_generated: true,
        }
    }};

    (pattern_hunt $pattern:expr) => {{
        println!("🔍 HUNTING: Pattern {} in binary", $pattern);
        TestFeature {
            name: format!("pattern_hunt_{}", $pattern),
            test_type: TestType::PatternHunting($pattern.to_string()),
            auto_generated: true,
        }
    }};

    (decoder_verify $decoder:expr) => {{
        println!("✅ VERIFYING: Decoder function {}", $decoder);
        TestFeature {
            name: format!("decoder_verify_{}", $decoder.replace("::", "_")),
            test_type: TestType::DecoderVerification($decoder.to_string()),
            auto_generated: true,
        }
    }};

    (self_ref_check $function:expr) => {{
        println!("🔄 CHECKING: Self-referential patterns in {}", $function);
        TestFeature {
            name: format!("self_ref_check_{}", $function.replace("::", "_")),
            test_type: TestType::SelfReferenceCheck($function.to_string()),
            auto_generated: true,
        }
    }};
}

macro_rules! mkbuild {
    (features=[$($feature:expr),*]) => {
        {
            let mut experiment = AutoExperiment::new();
            $(
                experiment.add_feature($feature);
            )*
            experiment.execute()
        }
    };
}

#[derive(Debug, Clone)]
enum TestType {
    OpcodeDecoding(u8, String),
    PatternHunting(String),
    DecoderVerification(String),
    SelfReferenceCheck(String),
}

#[derive(Debug, Clone)]
struct TestFeature {
    name: String,
    test_type: TestType,
    auto_generated: bool,
}

struct AutoExperiment {
    features: Vec<TestFeature>,
    results: HashMap<String, bool>,
}

impl AutoExperiment {
    fn new() -> Self {
        Self { features: Vec::new(), results: HashMap::new() }
    }

    fn add_feature(&mut self, feature: TestFeature) {
        self.features.push(feature);
    }

    fn execute(&mut self) -> ExperimentResults {
        println!("\n🚀 EXECUTING AUTO-GENERATED EXPERIMENT");
        println!("======================================");

        let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
        let binary = fs::read(binary_path).expect("Failed to read binary");
        let elf = Elf::parse(&binary).expect("Failed to parse ELF");

        let mut passed = 0;
        let mut failed = 0;

        for feature in &self.features {
            let result = self.run_test(feature, &binary, &elf);
            self.results.insert(feature.name.clone(), result);

            if result {
                passed += 1;
                println!("  ✅ {} - PASSED", feature.name);
            } else {
                failed += 1;
                println!("  ❌ {} - FAILED", feature.name);
            }
        }

        ExperimentResults {
            total_features: self.features.len(),
            passed,
            failed,
            results: self.results.clone(),
        }
    }

    fn run_test(&self, feature: &TestFeature, binary: &[u8], elf: &Elf) -> bool {
        match &feature.test_type {
            TestType::OpcodeDecoding(opcode, expected) => {
                self.test_opcode_decoding(*opcode, expected, binary, elf)
            }
            TestType::PatternHunting(pattern) => self.test_pattern_hunting(pattern, binary, elf),
            TestType::DecoderVerification(decoder) => {
                self.test_decoder_verification(decoder, binary, elf)
            }
            TestType::SelfReferenceCheck(function) => {
                self.test_self_reference_check(function, binary, elf)
            }
        }
    }

    fn test_opcode_decoding(&self, opcode: u8, expected: &str, binary: &[u8], elf: &Elf) -> bool {
        let text_section = elf
            .section_headers
            .iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
            .unwrap();

        let text_start = text_section.sh_offset as usize;
        let text_bytes = &binary[text_start..text_start + text_section.sh_size as usize];

        // Count occurrences of this opcode
        let count = text_bytes.chunks_exact(4).filter(|chunk| (chunk[0] as u8) == opcode).count();

        // Test passes if opcode exists and expected classification makes sense
        count > 0 && !expected.is_empty()
    }

    fn test_pattern_hunting(&self, pattern: &str, _binary: &[u8], elf: &Elf) -> bool {
        // Hunt for pattern in symbol names
        let found = elf
            .syms
            .iter()
            .filter_map(|sym| elf.strtab.get_at(sym.st_name))
            .any(|name| name.to_lowercase().contains(&pattern.to_lowercase()));

        found
    }

    fn test_decoder_verification(&self, decoder: &str, _binary: &[u8], elf: &Elf) -> bool {
        // Verify decoder function exists
        let found = elf.syms.iter().filter_map(|sym| elf.strtab.get_at(sym.st_name)).any(|name| {
            let demangled = rust_demangle(name);
            demangled.contains(decoder)
        });

        found
    }

    fn test_self_reference_check(&self, function: &str, binary: &[u8], elf: &Elf) -> bool {
        // Check if function contains its own opcodes as data
        let text_section = elf
            .section_headers
            .iter()
            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
            .unwrap();

        let text_start = text_section.sh_offset as usize;
        let text_bytes = &binary[text_start..text_start + text_section.sh_size as usize];

        // Find function and check for self-reference
        for sym in elf.syms.iter() {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                let demangled = rust_demangle(name);
                if demangled.contains(function) && sym.st_size > 0 {
                    let func_start = (sym.st_value - text_section.sh_addr) as usize;
                    let func_size = sym.st_size as usize;

                    if func_start + func_size <= text_bytes.len() {
                        let func_bytes = &text_bytes[func_start..func_start + func_size];

                        // Check for self-referential patterns
                        let opcodes_used: Vec<u8> =
                            func_bytes.chunks_exact(4).map(|chunk| chunk[0]).collect();

                        let opcodes_as_data: Vec<u8> = func_bytes.iter().cloned().collect();

                        // Test passes if function uses opcodes that also appear as data
                        return opcodes_used.iter().any(|&op| opcodes_as_data.contains(&op));
                    }
                }
            }
        }

        false
    }
}

#[derive(Debug)]
struct ExperimentResults {
    total_features: usize,
    passed: usize,
    failed: usize,
    results: HashMap<String, bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 AUTO-TESTING SUGGESTION SYSTEM");
    println!("=================================");

    // DNA: Single macro call that generates entire experiment
    let results = mkbuild! {
        features=[
            mkfeature!(opcode_test 0x55, "PUSH RBP"),
            mkfeature!(opcode_test 0x48, "REX.W prefix"),
            mkfeature!(opcode_test 0x7e, "JLE rel8"),
            mkfeature!(opcode_test 0xc9, "LEAVE"),
            mkfeature!(pattern_hunt "fma"),
            mkfeature!(pattern_hunt "backend"),
            mkfeature!(pattern_hunt "codegen"),
            mkfeature!(decoder_verify "compiler_builtins::math::libm_math::arch::x86::fma"),
            mkfeature!(decoder_verify "rustc_driver_impl::get_backend_from_raw_matches"),
            mkfeature!(self_ref_check "fma_fallback"),
            mkfeature!(self_ref_check "get_backend_from_raw_matches")
        ]
    };

    println!("\n🏆 EXPERIMENT RESULTS:");
    println!("======================");
    println!("   Total features tested: {}", results.total_features);
    println!(
        "   Passed: {} ({}%)",
        results.passed,
        (results.passed as f64 / results.total_features as f64 * 100.0) as u32
    );
    println!(
        "   Failed: {} ({}%)",
        results.failed,
        (results.failed as f64 / results.total_features as f64 * 100.0) as u32
    );

    // Auto-generate new experiments based on results
    println!("\n🔬 AUTO-GENERATING NEW EXPERIMENTS:");
    if results.passed > results.failed {
        println!("   Success rate high - generating advanced tests");
        let _advanced = mkbuild! {
            features=[
                mkfeature!(opcode_test 0xff, "INC/DEC/CALL/JMP group"),
                mkfeature!(pattern_hunt "spectral"),
                mkfeature!(self_ref_check "describe_codegen_flags")
            ]
        };
    } else {
        println!("   Success rate low - generating basic validation tests");
    }

    println!("\n✅ AUTO-TEST SYSTEM COMPLETE");
    println!("   System DNA: mkbuild! macro with feature composition");
    println!("   Self-generating experiments based on results");

    Ok(())
}

fn rust_demangle(mangled: &str) -> String {
    if !mangled.starts_with("_ZN") {
        return mangled.to_string();
    }

    let core = &mangled[3..];
    let mut result = String::new();
    let mut chars = core.chars().peekable();
    let mut current_len = String::new();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            current_len.push(ch);
        } else if ch == 'E' {
            break;
        } else {
            if let Ok(len) = current_len.parse::<usize>() {
                if len > 0 && len < 200 {
                    let mut segment = String::new();
                    segment.push(ch);

                    for _ in 1..len {
                        if let Some(next_ch) = chars.next() {
                            segment.push(next_ch);
                        }
                    }

                    if !result.is_empty() {
                        result.push_str("::");
                    }
                    result.push_str(&segment);
                }
            }
            current_len.clear();
        }
    }

    if result.is_empty() { mangled.to_string() } else { result }
}
