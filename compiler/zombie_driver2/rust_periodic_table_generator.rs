use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct RustElement {
    atomic_number: usize,
    symbol: String,
    name: String,
    element_type: ElementType,
    modular_key: String,
    weight: u32,
    level: u32,
    period: usize,
    group: usize,
    hotness: f64,
}

#[derive(Debug, Clone)]
enum ElementType {
    Function,
    Enum,
    Struct,
    Trait,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 PERIODIC TABLE OF RUST ELEMENTS");
    println!("==================================");

    // Core Rust elements with their properties
    let rust_elements = vec![
        // Period 1: Core Types (Hydrogen-like)
        ("Opt", "Option", ElementType::Enum, 11.0),
        ("Res", "Result", ElementType::Enum, 10.5),
        // Period 2: Comparison & Ordering (Alkali metals)
        ("Ord", "Ordering", ElementType::Enum, 8.5),
        ("Eq", "PartialEq", ElementType::Trait, 9.0),
        ("Cmp", "PartialOrd", ElementType::Trait, 8.0),
        ("Hash", "Hash", ElementType::Trait, 7.5),
        ("Clone", "Clone", ElementType::Trait, 9.5),
        ("Copy", "Copy", ElementType::Trait, 8.5),
        // Period 3: Collections (Transition metals)
        ("Vec", "Vec", ElementType::Struct, 9.0),
        ("Map", "HashMap", ElementType::Struct, 8.0),
        ("Set", "HashSet", ElementType::Struct, 7.0),
        ("Str", "String", ElementType::Struct, 9.5),
        ("Slice", "&[T]", ElementType::Struct, 8.5),
        ("Array", "[T; N]", ElementType::Struct, 7.5),
        ("Tuple", "(T, U)", ElementType::Struct, 8.0),
        ("Box", "Box", ElementType::Struct, 7.0),
        // Period 4: I/O & Error Handling (Lanthanides)
        ("Err", "ErrorKind", ElementType::Enum, 8.0),
        ("IO", "std::io", ElementType::Function, 7.5),
        ("File", "File", ElementType::Struct, 6.5),
        ("Path", "Path", ElementType::Struct, 6.0),
        ("Seek", "SeekFrom", ElementType::Enum, 4.5),
        ("Read", "Read", ElementType::Trait, 7.0),
        ("Write", "Write", ElementType::Trait, 7.0),
        ("BufRead", "BufRead", ElementType::Trait, 6.0),
        // Period 5: Network & Concurrency (Actinides)
        ("IP", "IpAddr", ElementType::Enum, 6.0),
        ("Sock", "SocketAddr", ElementType::Enum, 5.5),
        ("TCP", "TcpStream", ElementType::Struct, 5.0),
        ("UDP", "UdpSocket", ElementType::Struct, 4.5),
        ("Thread", "thread", ElementType::Function, 6.5),
        ("Mutex", "Mutex", ElementType::Struct, 6.0),
        ("Arc", "Arc", ElementType::Struct, 5.5),
        ("Chan", "channel", ElementType::Function, 5.0),
    ];

    println!("📊 Calculating periodic properties for {} Rust elements...", rust_elements.len());

    let mut periodic_table = Vec::new();

    for (i, (symbol, name, element_type, hotness)) in rust_elements.iter().enumerate() {
        let atomic_number = i + 1;

        // Calculate modular properties
        let modular_key = calculate_modular_key(name, atomic_number);
        let (level, weight) = parse_modular_key(&modular_key);

        // Calculate periodic position
        let period = calculate_period(atomic_number);
        let group = calculate_group(atomic_number, &element_type);

        let element = RustElement {
            atomic_number,
            symbol: symbol.to_string(),
            name: name.to_string(),
            element_type: element_type.clone(),
            modular_key,
            weight,
            level,
            period,
            group,
            hotness: *hotness,
        };

        periodic_table.push(element);
    }

    // Generate periodic table visualization
    println!("\n🧪 RUST PERIODIC TABLE:");
    println!("========================");

    // Group by periods
    let mut periods: HashMap<usize, Vec<&RustElement>> = HashMap::new();
    for element in &periodic_table {
        periods.entry(element.period).or_insert_with(Vec::new).push(element);
    }

    for period_num in 1..=5 {
        if let Some(period_elements) = periods.get(&period_num) {
            println!("\nPeriod {}: {} elements", period_num, period_elements.len());

            // Sort by group within period
            let mut sorted_elements = period_elements.clone();
            sorted_elements.sort_by_key(|e| e.group);

            // Print elements in groups
            for element in sorted_elements {
                let type_symbol = match element.element_type {
                    ElementType::Function => "Fn",
                    ElementType::Enum => "En",
                    ElementType::Struct => "St",
                    ElementType::Trait => "Tr",
                };

                println!(
                    "  [{:2}] {:>6} ({}) - {} | L:{} W:{} | G:{} | H:{:.1}",
                    element.atomic_number,
                    element.symbol,
                    type_symbol,
                    element.modular_key,
                    element.level,
                    element.weight,
                    element.group,
                    element.hotness
                );
            }
        }
    }

    // Generate periodic table code
    println!("\n💾 GENERATING PERIODIC TABLE MODULE:");

    let mut table_code = String::new();
    table_code.push_str("// Auto-generated Rust Periodic Table\n");
    table_code.push_str("use std::collections::HashMap;\n\n");

    table_code.push_str("#[derive(Debug, Clone)]\n");
    table_code.push_str("pub struct RustElement {\n");
    table_code.push_str("    pub atomic_number: usize,\n");
    table_code.push_str("    pub symbol: String,\n");
    table_code.push_str("    pub name: String,\n");
    table_code.push_str("    pub element_type: String,\n");
    table_code.push_str("    pub modular_key: String,\n");
    table_code.push_str("    pub weight: u32,\n");
    table_code.push_str("    pub level: u32,\n");
    table_code.push_str("    pub period: usize,\n");
    table_code.push_str("    pub group: usize,\n");
    table_code.push_str("    pub hotness: f64,\n");
    table_code.push_str("}\n\n");

    table_code.push_str("pub struct RustPeriodicTable {\n");
    table_code.push_str("    elements: HashMap<usize, RustElement>,\n");
    table_code.push_str("}\n\n");

    table_code.push_str("impl RustPeriodicTable {\n");
    table_code.push_str("    pub fn new() -> Self {\n");
    table_code.push_str("        let mut elements = HashMap::new();\n");

    for element in &periodic_table {
        let type_str = match element.element_type {
            ElementType::Function => "Function",
            ElementType::Enum => "Enum",
            ElementType::Struct => "Struct",
            ElementType::Trait => "Trait",
        };

        table_code.push_str(&format!(
            "        elements.insert({}, RustElement {{\n",
            element.atomic_number
        ));
        table_code.push_str(&format!("            atomic_number: {},\n", element.atomic_number));
        table_code.push_str(&format!("            symbol: \"{}\".to_string(),\n", element.symbol));
        table_code.push_str(&format!("            name: \"{}\".to_string(),\n", element.name));
        table_code.push_str(&format!("            element_type: \"{}\".to_string(),\n", type_str));
        table_code.push_str(&format!(
            "            modular_key: \"{}\".to_string(),\n",
            element.modular_key
        ));
        table_code.push_str(&format!("            weight: {},\n", element.weight));
        table_code.push_str(&format!("            level: {},\n", element.level));
        table_code.push_str(&format!("            period: {},\n", element.period));
        table_code.push_str(&format!("            group: {},\n", element.group));
        table_code.push_str(&format!("            hotness: {:.1},\n", element.hotness));
        table_code.push_str("        });\n");
    }

    table_code.push_str("        Self { elements }\n");
    table_code.push_str("    }\n\n");

    table_code.push_str(
        "    pub fn get_element(&self, atomic_number: usize) -> Option<&RustElement> {\n",
    );
    table_code.push_str("        self.elements.get(&atomic_number)\n");
    table_code.push_str("    }\n\n");

    table_code
        .push_str("    pub fn get_by_symbol(&self, symbol: &str) -> Option<&RustElement> {\n");
    table_code.push_str("        self.elements.values().find(|e| e.symbol == symbol)\n");
    table_code.push_str("    }\n\n");

    table_code.push_str("    pub fn get_period(&self, period: usize) -> Vec<&RustElement> {\n");
    table_code
        .push_str("        self.elements.values().filter(|e| e.period == period).collect()\n");
    table_code.push_str("    }\n");
    table_code.push_str("}\n");

    fs::write("rust_periodic_table.rs", table_code)?;
    println!("   💾 Saved periodic table to rust_periodic_table.rs");

    // Analysis
    println!("\n🔬 PERIODIC TABLE ANALYSIS:");

    let mut type_counts: HashMap<String, usize> = HashMap::new();
    let mut period_counts: HashMap<usize, usize> = HashMap::new();

    for element in &periodic_table {
        let type_name = match element.element_type {
            ElementType::Function => "Functions",
            ElementType::Enum => "Enums",
            ElementType::Struct => "Structs",
            ElementType::Trait => "Traits",
        };
        *type_counts.entry(type_name.to_string()).or_insert(0) += 1;
        *period_counts.entry(element.period).or_insert(0) += 1;
    }

    println!("   📊 Element distribution:");
    for (element_type, count) in type_counts {
        println!("      {}: {}", element_type, count);
    }

    println!("\n   📊 Period distribution:");
    for (period, count) in period_counts {
        println!("      Period {}: {} elements", period, count);
    }

    // Find noble gases (most stable elements)
    let noble_gases: Vec<_> = periodic_table.iter().filter(|e| e.group == 18).collect();

    if !noble_gases.is_empty() {
        println!("\n   🌟 Noble Gases (Most Stable):");
        for element in noble_gases {
            println!(
                "      {}: {} (hotness: {:.1})",
                element.symbol, element.name, element.hotness
            );
        }
    }

    println!("\n🎯 PERIODIC TABLE COMPLETE!");
    println!("   {} Rust elements organized by mathematical properties", periodic_table.len());
    println!("   Periods represent complexity levels");
    println!("   Groups represent functional families");
    println!("   Modular keys provide mathematical classification");

    Ok(())
}

fn calculate_modular_key(name: &str, atomic_number: usize) -> String {
    let name_hash = name.bytes().map(|b| b as u64).sum::<u64>();
    let combined = name_hash.wrapping_add(atomic_number as u64);

    let level = (combined % 37) + 1;
    let weight = if combined % 3 == 0 {
        2
    } else if combined % 3 == 1 {
        4
    } else {
        6
    };
    let character = if combined % 2 == 0 { "12" } else { "11" };
    let orbit = ((combined % 26) as u8 + b'a') as char;

    format!("{}.{}.{}.{}", level, weight, character, orbit)
}

fn parse_modular_key(key: &str) -> (u32, u32) {
    let parts: Vec<&str> = key.split('.').collect();
    if parts.len() >= 2 {
        (parts[0].parse().unwrap_or(1), parts[1].parse().unwrap_or(2))
    } else {
        (1, 2)
    }
}

fn calculate_period(atomic_number: usize) -> usize {
    match atomic_number {
        1..=2 => 1,   // Core types
        3..=10 => 2,  // Traits & basic operations
        11..=18 => 3, // Collections & data structures
        19..=26 => 4, // I/O & error handling
        27..=32 => 5, // Network & concurrency
        _ => 6,       // Advanced/experimental
    }
}

fn calculate_group(atomic_number: usize, element_type: &ElementType) -> usize {
    let base_group = match element_type {
        ElementType::Enum => 1,      // Group 1: Alkali metals (reactive)
        ElementType::Trait => 2,     // Group 2: Alkaline earth metals
        ElementType::Struct => 13,   // Group 13: Boron group (versatile)
        ElementType::Function => 18, // Group 18: Noble gases (stable)
    };

    // Adjust based on atomic number for variety
    base_group + (atomic_number % 3)
}
