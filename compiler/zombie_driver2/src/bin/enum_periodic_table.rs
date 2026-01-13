use serde_json;
use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, Clone)]
struct EnumElement {
    atomic_number: usize,
    symbol: String,
    name: String,
    group_order: usize,
    generators: usize,
    ring_closure: bool,
    is_abelian: bool,
    is_cyclic: bool,
    period: usize,
    group: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 RUSTC ENUM PERIODIC TABLE GENERATOR");
    println!("=====================================");

    // Load lattice data
    let lattice_data = fs::read_to_string("rustc_enum_string_lattice.json")?;
    let lattice: serde_json::Value = serde_json::from_str(&lattice_data)?;

    let orbits = lattice["orbits"].as_array().unwrap();

    println!("📊 Building periodic table from {} orbits", orbits.len());

    let mut elements = Vec::new();

    // Convert orbits to periodic elements
    for (i, orbit) in orbits.iter().enumerate() {
        let length = orbit["length"].as_u64().unwrap() as usize;
        let group_order = orbit["group_order"].as_u64().unwrap() as usize;
        let generators = orbit["generators"].as_array().unwrap().len();
        let ring_closure = orbit["ring_closure"].as_bool().unwrap_or(false);

        // Determine mathematical properties
        let is_cyclic = generators == 1;
        let is_abelian = !orbit["sample_variants"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_str().unwrap_or("").contains("Mut"));

        // Calculate period (row) and group (column)
        let period = calculate_period(length);
        let group = calculate_group(generators, ring_closure);

        let element = EnumElement {
            atomic_number: i + 1,
            symbol: generate_symbol(length, generators),
            name: format!("Enum-{}", length),
            group_order,
            generators,
            ring_closure,
            is_abelian,
            is_cyclic,
            period,
            group,
        };

        elements.push(element);
    }

    // Sort by atomic number
    elements.sort_by_key(|e| e.atomic_number);

    println!("\n🧪 RUSTC ENUM PERIODIC TABLE");
    println!("============================");

    // Print table header
    println!(
        "     1    2    3    4    5    6    7    8    9   10   11   12   13   14   15   16   17   18"
    );
    println!(
        "   ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐"
    );

    // Group elements by period
    let mut periods: BTreeMap<usize, Vec<&EnumElement>> = BTreeMap::new();
    for element in &elements {
        periods.entry(element.period).or_insert(Vec::new()).push(element);
    }

    // Print each period
    for (period_num, period_elements) in &periods {
        print!("{:2} │", period_num);

        // Create 18-column grid
        let mut grid = vec!["    "; 18];

        for element in period_elements {
            let col = (element.group - 1).min(17);
            grid[col] = &element.symbol;
        }

        for cell in grid {
            print!("{:4}│", cell);
        }
        println!();

        // Print separator
        if *period_num < periods.len() {
            println!(
                "   ├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤"
            );
        }
    }

    println!(
        "   └────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘"
    );

    // Print legend and element details
    println!("\n📋 ELEMENT LEGEND:");
    println!("==================");

    for element in &elements {
        let properties = format!(
            "{}{}{}",
            if element.ring_closure { "R" } else { "-" },
            if element.is_abelian { "A" } else { "-" },
            if element.is_cyclic { "C" } else { "-" }
        );

        println!(
            "{:2} {:4} {:12} | Order:{:6} Gen:{:2} Props:{} Period:{} Group:{}",
            element.atomic_number,
            element.symbol,
            element.name,
            element.group_order,
            element.generators,
            properties,
            element.period,
            element.group
        );
    }

    // Analyze periodic trends
    analyze_periodic_trends(&elements);

    // Save periodic table
    let table_json = serde_json::json!({
        "total_elements": elements.len(),
        "periods": periods.len(),
        "elements": elements.iter().map(|e| {
            serde_json::json!({
                "atomic_number": e.atomic_number,
                "symbol": e.symbol,
                "name": e.name,
                "group_order": e.group_order,
                "generators": e.generators,
                "period": e.period,
                "group": e.group,
                "properties": {
                    "ring_closure": e.ring_closure,
                    "abelian": e.is_abelian,
                    "cyclic": e.is_cyclic
                }
            })
        }).collect::<Vec<_>>()
    });

    fs::write("rustc_enum_periodic_table.json", serde_json::to_string_pretty(&table_json)?)?;
    println!("\n💾 Saved periodic table to rustc_enum_periodic_table.json");

    Ok(())
}

fn calculate_period(length: usize) -> usize {
    // Period based on string length (electron shells analogy)
    match length {
        1..=2 => 1,   // K shell
        3..=10 => 2,  // L shell
        11..=18 => 3, // M shell
        19..=36 => 4, // N shell
        37..=54 => 5, // O shell
        _ => 6,       // P shell and beyond
    }
}

fn calculate_group(generators: usize, ring_closure: bool) -> usize {
    // Group based on mathematical properties
    match (generators, ring_closure) {
        (1, true) => 1,  // Alkali metals (highly reactive, single generator)
        (1, false) => 2, // Alkaline earth metals
        (2, true) => 3,  // Transition metals start
        (2, false) => 4,
        (3, true) => 5, // Transition metals
        (3, false) => 6,
        (4, _) => 7,
        (5, _) => 8,
        _ => (generators % 10) + 9, // Higher groups
    }
}

fn generate_symbol(length: usize, generators: usize) -> String {
    // Generate chemical-like symbols
    let length_char = match length {
        1..=2 => 'H',   // Hydrogen-like
        3..=4 => 'L',   // Lithium-like
        5..=6 => 'C',   // Carbon-like
        7..=8 => 'N',   // Nitrogen-like
        9..=10 => 'O',  // Oxygen-like
        11..=12 => 'F', // Fluorine-like
        13..=18 => 'S', // Sulfur-like
        19..=25 => 'K', // Potassium-like
        26..=30 => 'I', // Iron-like
        31..=36 => 'G', // Gallium-like
        37..=48 => 'R', // Rubidium-like
        _ => 'U',       // Unknown
    };

    let gen_char = match generators {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        _ => 'x',
    };

    format!("{}{}", length_char, gen_char)
}

fn analyze_periodic_trends(elements: &[EnumElement]) {
    println!("\n🔬 PERIODIC TRENDS ANALYSIS:");
    println!("============================");

    // Group order trends
    let avg_order_by_period: BTreeMap<usize, f64> = elements
        .iter()
        .fold(BTreeMap::new(), |mut acc, e| {
            let entry = acc.entry(e.period).or_insert((0.0, 0));
            entry.0 += e.group_order as f64;
            entry.1 += 1;
            acc
        })
        .into_iter()
        .map(|(period, (sum, count))| (period, sum / count as f64))
        .collect();

    println!("📊 Average Group Order by Period:");
    for (period, avg_order) in avg_order_by_period {
        println!("   Period {}: {:.1}", period, avg_order);
    }

    // Generator trends
    let generator_distribution: BTreeMap<usize, usize> =
        elements.iter().fold(BTreeMap::new(), |mut acc, e| {
            *acc.entry(e.generators).or_insert(0) += 1;
            acc
        });

    println!("\n🧮 Generator Distribution:");
    for (gen_count, element_count) in generator_distribution {
        println!("   {} generators: {} elements", gen_count, element_count);
    }

    // Mathematical property statistics
    let ring_elements = elements.iter().filter(|e| e.ring_closure).count();
    let abelian_elements = elements.iter().filter(|e| e.is_abelian).count();
    let cyclic_elements = elements.iter().filter(|e| e.is_cyclic).count();

    println!("\n💍 Mathematical Properties:");
    println!(
        "   Ring Closure: {}/{} ({:.1}%)",
        ring_elements,
        elements.len(),
        100.0 * ring_elements as f64 / elements.len() as f64
    );
    println!(
        "   Abelian: {}/{} ({:.1}%)",
        abelian_elements,
        elements.len(),
        100.0 * abelian_elements as f64 / elements.len() as f64
    );
    println!(
        "   Cyclic: {}/{} ({:.1}%)",
        cyclic_elements,
        elements.len(),
        100.0 * cyclic_elements as f64 / elements.len() as f64
    );
}
