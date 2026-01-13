// Function Body Instruction Lattice Decoder
use goblin::elf::Elf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

const MONSTER_PRIMES: [u64; 35] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71, 37, 43, 53, 61, 67, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
];

#[derive(Debug, Serialize, Deserialize)]
struct InstructionLattice {
    function_name: String,
    function_address: u64,
    instructions: Vec<InstructionNode>,
    lattice_layers: Vec<LatticeLayer>,
    control_flow: Vec<ControlFlow>,
    monster_instruction_map: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstructionNode {
    address: u64,
    bytes: Vec<u8>,
    opcode: String,
    operands: Vec<String>,
    instruction_type: String,
    monster_score: f64,
    lattice_position: (u32, u32), // (layer, position_in_layer)
    semantic_label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LatticeLayer {
    layer_id: u32,
    layer_type: String,     // "entry", "computation", "control", "exit"
    instructions: Vec<u64>, // addresses
    monster_correlation: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ControlFlow {
    from_address: u64,
    to_address: u64,
    flow_type: String, // "sequential", "jump", "call", "return"
    monster_flow_strength: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 FUNCTION BODY INSTRUCTION LATTICE DECODER");
    println!("============================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    println!("📦 Loaded binary: {} bytes, {} symbols", binary.len(), elf.syms.len());

    // Analyze first 20 functions with significant size
    let mut lattices = Vec::new();
    let mut analyzed = 0;

    for sym in &elf.syms {
        if sym.st_size > 50 && sym.st_value > 0 && analyzed < 20 {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if let Some(lattice) = decode_function_lattice(&binary, &sym, name) {
                    lattices.push(lattice);
                    analyzed += 1;
                    println!(
                        "   ✅ Decoded function {}: {}",
                        analyzed,
                        if name.len() > 40 { &name[..40] } else { name }
                    );
                }
            }
        }
    }

    // Analyze instruction patterns
    analyze_instruction_patterns(&lattices);

    // Save results
    save_instruction_lattices(&lattices)?;

    Ok(())
}

fn decode_function_lattice(
    binary: &[u8],
    sym: &goblin::elf::Sym,
    name: &str,
) -> Option<InstructionLattice> {
    let start = sym.st_value as usize;
    let size = sym.st_size as usize;

    if start + size > binary.len() {
        return None;
    }

    let function_bytes = &binary[start..start + size];
    let mut instructions = Vec::new();
    let mut offset = 0;

    // Decode instructions
    while offset < function_bytes.len() {
        if let Some(instruction) = decode_instruction(function_bytes, offset, start as u64) {
            let instruction_len = instruction.bytes.len();
            instructions.push(instruction);
            offset += instruction_len.max(1);
        } else {
            offset += 1;
        }

        if instructions.len() >= 100 {
            // Limit instructions per function
            break;
        }
    }

    // Build lattice layers
    let lattice_layers = build_lattice_layers(&instructions);

    // Analyze control flow
    let control_flow = analyze_control_flow(&instructions);

    // Create Monster instruction mapping
    let monster_instruction_map = create_monster_instruction_map(&instructions);

    Some(InstructionLattice {
        function_name: name.to_string(),
        function_address: sym.st_value,
        instructions,
        lattice_layers,
        control_flow,
        monster_instruction_map,
    })
}

fn decode_instruction(bytes: &[u8], offset: usize, base_address: u64) -> Option<InstructionNode> {
    if offset >= bytes.len() {
        return None;
    }

    let address = base_address + offset as u64;
    let byte = bytes[offset];

    // Simple x86-64 instruction decoding
    let (opcode, operands, instruction_type, instruction_bytes) = match byte {
        0x48 => {
            // REX prefix - look at next byte
            if offset + 1 < bytes.len() {
                match bytes[offset + 1] {
                    0x89 => (
                        "mov".to_string(),
                        vec!["reg64".to_string(), "reg64".to_string()],
                        "data_movement".to_string(),
                        bytes[offset..offset.min(bytes.len()).min(offset + 3)].to_vec(),
                    ),
                    0x8B => (
                        "mov".to_string(),
                        vec!["reg64".to_string(), "mem64".to_string()],
                        "data_movement".to_string(),
                        bytes[offset..offset.min(bytes.len()).min(offset + 3)].to_vec(),
                    ),
                    0x83 => (
                        "add/sub".to_string(),
                        vec!["reg64".to_string(), "imm8".to_string()],
                        "arithmetic".to_string(),
                        bytes[offset..offset.min(bytes.len()).min(offset + 4)].to_vec(),
                    ),
                    _ => ("rex_prefix".to_string(), vec![], "prefix".to_string(), vec![byte]),
                }
            } else {
                ("rex_prefix".to_string(), vec![], "prefix".to_string(), vec![byte])
            }
        }
        0xE8 => (
            "call".to_string(),
            vec!["rel32".to_string()],
            "control_flow".to_string(),
            bytes[offset..offset.min(bytes.len()).min(offset + 5)].to_vec(),
        ),
        0xE9 => (
            "jmp".to_string(),
            vec!["rel32".to_string()],
            "control_flow".to_string(),
            bytes[offset..offset.min(bytes.len()).min(offset + 5)].to_vec(),
        ),
        0xC3 => ("ret".to_string(), vec![], "control_flow".to_string(), vec![byte]),
        0x50..=0x57 => (
            "push".to_string(),
            vec![format!("reg{}", byte - 0x50)],
            "stack_operation".to_string(),
            vec![byte],
        ),
        0x58..=0x5F => (
            "pop".to_string(),
            vec![format!("reg{}", byte - 0x58)],
            "stack_operation".to_string(),
            vec![byte],
        ),
        0x90 => ("nop".to_string(), vec![], "no_operation".to_string(), vec![byte]),
        0x74..=0x7F => (
            "jcc".to_string(),
            vec!["rel8".to_string()],
            "conditional_jump".to_string(),
            bytes[offset..offset.min(bytes.len()).min(offset + 2)].to_vec(),
        ),
        _ => ("unknown".to_string(), vec![], "unknown".to_string(), vec![byte]),
    };

    let monster_score = calculate_instruction_monster_score(&instruction_bytes);
    let semantic_label = generate_semantic_label(&opcode, &instruction_type, monster_score);

    Some(InstructionNode {
        address,
        bytes: instruction_bytes,
        opcode,
        operands,
        instruction_type,
        monster_score,
        lattice_position: (0, 0), // Will be set by lattice builder
        semantic_label,
    })
}

fn calculate_instruction_monster_score(bytes: &[u8]) -> f64 {
    let mut score = 0.0;

    for &byte in bytes {
        for &prime in &MONSTER_PRIMES[..10] {
            // Use first 10 primes for performance
            if byte as u64 % prime == 0 {
                score += 1.0 / prime as f64;
            }
        }
    }

    score / bytes.len() as f64
}

fn generate_semantic_label(opcode: &str, instruction_type: &str, monster_score: f64) -> String {
    let monster_level = if monster_score > 1.0 {
        "high_monster"
    } else if monster_score > 0.5 {
        "med_monster"
    } else {
        "low_monster"
    };

    format!("{}_{}__{}", instruction_type, opcode, monster_level)
}

fn build_lattice_layers(instructions: &[InstructionNode]) -> Vec<LatticeLayer> {
    let mut layers = Vec::new();
    let mut current_layer = 0;
    let mut layer_instructions = Vec::new();
    let mut layer_type = "entry";

    for (i, instruction) in instructions.iter().enumerate() {
        layer_instructions.push(instruction.address);

        // Determine layer transitions
        let should_transition = match instruction.instruction_type.as_str() {
            "control_flow" => true,
            "conditional_jump" => true,
            _ => i > 0 && i % 10 == 0, // New layer every 10 instructions
        };

        if should_transition || i == instructions.len() - 1 {
            let monster_correlation = layer_instructions
                .iter()
                .filter_map(|&addr| instructions.iter().find(|inst| inst.address == addr))
                .map(|inst| inst.monster_score)
                .sum::<f64>()
                / layer_instructions.len().max(1) as f64;

            layers.push(LatticeLayer {
                layer_id: current_layer,
                layer_type: layer_type.to_string(),
                instructions: layer_instructions.clone(),
                monster_correlation,
            });

            current_layer += 1;
            layer_instructions.clear();

            // Update layer type based on instruction patterns
            layer_type = match instruction.instruction_type.as_str() {
                "control_flow" => "control",
                "arithmetic" => "computation",
                "data_movement" => "computation",
                _ => "computation",
            };
        }
    }

    layers
}

fn analyze_control_flow(instructions: &[InstructionNode]) -> Vec<ControlFlow> {
    let mut flows = Vec::new();

    for (i, instruction) in instructions.iter().enumerate() {
        let flow_type = match instruction.instruction_type.as_str() {
            "control_flow" => {
                if instruction.opcode == "call" {
                    "call"
                } else if instruction.opcode == "ret" {
                    "return"
                } else {
                    "jump"
                }
            }
            "conditional_jump" => "conditional_jump",
            _ => "sequential",
        };

        // Find target address (simplified)
        let to_address = if i + 1 < instructions.len() {
            instructions[i + 1].address
        } else {
            instruction.address + instruction.bytes.len() as u64
        };

        let monster_flow_strength = (instruction.monster_score
            + instructions.get(i + 1).map(|next| next.monster_score).unwrap_or(0.0))
            / 2.0;

        flows.push(ControlFlow {
            from_address: instruction.address,
            to_address,
            flow_type: flow_type.to_string(),
            monster_flow_strength,
        });
    }

    flows
}

fn create_monster_instruction_map(instructions: &[InstructionNode]) -> HashMap<String, f64> {
    let mut map = HashMap::new();

    for instruction in instructions {
        let key = format!("{}_{}", instruction.opcode, instruction.instruction_type);
        let entry = map.entry(key).or_insert(0.0);
        *entry += instruction.monster_score;
    }

    map
}

fn analyze_instruction_patterns(lattices: &[InstructionLattice]) {
    println!("\n🔧 INSTRUCTION LATTICE ANALYSIS:");
    println!("================================");

    // Aggregate statistics
    let total_instructions: usize = lattices.iter().map(|l| l.instructions.len()).sum();
    let total_layers: usize = lattices.iter().map(|l| l.lattice_layers.len()).sum();

    println!("   📊 Overall Statistics:");
    println!("     Functions analyzed: {}", lattices.len());
    println!("     Total instructions: {}", total_instructions);
    println!("     Total lattice layers: {}", total_layers);
    println!(
        "     Avg instructions per function: {:.1}",
        total_instructions as f64 / lattices.len() as f64
    );

    // Most common opcodes
    let mut opcode_counts = HashMap::new();
    for lattice in lattices {
        for instruction in &lattice.instructions {
            *opcode_counts.entry(instruction.opcode.clone()).or_insert(0) += 1;
        }
    }

    let mut sorted_opcodes: Vec<_> = opcode_counts.into_iter().collect();
    sorted_opcodes.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n   🔧 Most Common Opcodes:");
    for (opcode, count) in sorted_opcodes.iter().take(10) {
        println!("     {}: {} occurrences", opcode, count);
    }

    // Instruction type distribution
    let mut type_counts = HashMap::new();
    for lattice in lattices {
        for instruction in &lattice.instructions {
            *type_counts.entry(instruction.instruction_type.clone()).or_insert(0) += 1;
        }
    }

    println!("\n   📂 Instruction Type Distribution:");
    for (inst_type, count) in type_counts {
        println!("     {}: {} instructions", inst_type, count);
    }

    // Monster correlation analysis
    let avg_monster_score: f64 =
        lattices.iter().flat_map(|l| &l.instructions).map(|i| i.monster_score).sum::<f64>()
            / total_instructions as f64;

    println!("\n   🧬 Monster Correlation:");
    println!("     Average instruction Monster score: {:.4}", avg_monster_score);

    // Top Monster instructions
    let mut monster_instructions: Vec<_> = lattices.iter().flat_map(|l| &l.instructions).collect();
    monster_instructions.sort_by(|a, b| b.monster_score.partial_cmp(&a.monster_score).unwrap());

    println!("\n   🎭 Top Monster Instructions:");
    for (i, instruction) in monster_instructions.iter().take(5).enumerate() {
        println!(
            "     {}: {} {} (score: {:.4})",
            i + 1,
            instruction.opcode,
            instruction.instruction_type,
            instruction.monster_score
        );
    }
}

fn save_instruction_lattices(
    lattices: &[InstructionLattice],
) -> Result<(), Box<dyn std::error::Error>> {
    // Save complete lattices
    let json = serde_json::to_string_pretty(lattices)?;
    fs::write("instruction_lattices.json", json)?;

    // Save instruction summary CSV
    let mut csv_content =
        String::from("function,address,opcode,instruction_type,monster_score,semantic_label\n");
    for lattice in lattices {
        for instruction in &lattice.instructions {
            csv_content.push_str(&format!(
                "{},{:x},{},{},{:.4},{}\n",
                lattice.function_name.replace(',', ";"),
                instruction.address,
                instruction.opcode,
                instruction.instruction_type,
                instruction.monster_score,
                instruction.semantic_label
            ));
        }
    }
    fs::write("instruction_analysis.csv", csv_content)?;

    println!("\n💾 INSTRUCTION LATTICES SAVED:");
    println!("==============================");
    println!("   Complete lattices: instruction_lattices.json");
    println!("   Analysis CSV: instruction_analysis.csv");
    println!("   Functions decoded: {}", lattices.len());

    Ok(())
}
