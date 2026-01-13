use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct TypeCatalog {
    catalog: Vec<TypeEntry>,
}

#[derive(Debug, Deserialize)]
struct TypeEntry {
    name: String,
    emoji: String,
    category: String,
    frequency: u32,
    complexity_score: f64,
    curve_class: String,
    mathematical_properties: String,
}

#[derive(Debug, Serialize)]
struct HuggingFaceDataset {
    dataset_info: DatasetInfo,
    features: HashMap<String, FeatureType>,
    splits: HashMap<String, SplitInfo>,
    data: Vec<DataRow>,
}

#[derive(Debug, Serialize)]
struct DatasetInfo {
    description: String,
    citation: String,
    homepage: String,
    license: String,
    features: HashMap<String, FeatureSpec>,
}

#[derive(Debug, Serialize)]
struct FeatureSpec {
    dtype: String,
    description: String,
}

#[derive(Debug, Serialize)]
struct FeatureType {
    dtype: String,
}

#[derive(Debug, Serialize)]
struct SplitInfo {
    name: String,
    num_bytes: u64,
    num_examples: usize,
}

#[derive(Debug, Serialize)]
struct DataRow {
    type_name: String,
    emoji: String,
    category: String,
    frequency: u32,
    complexity_score: f64,
    curve_class: String,
    genus: u32,
    rank: u32,
    torsion: String,
    mathematical_beauty: f64,
}

fn parse_mathematical_properties(props: &str) -> (u32, u32, String) {
    let mut genus = 0;
    let mut rank = 0;
    let mut torsion = "unknown".to_string();

    for part in props.split(", ") {
        if let Some(val) = part.strip_prefix("genus:") {
            genus = val.parse().unwrap_or(0);
        } else if let Some(val) = part.strip_prefix("rank:") {
            rank = val.parse().unwrap_or(0);
        } else if let Some(val) = part.strip_prefix("torsion:") {
            torsion = val.to_string();
        }
    }

    (genus, rank, torsion)
}

fn calculate_mathematical_beauty(complexity: f64, genus: u32, rank: u32) -> f64 {
    // Beauty = inverse complexity + genus elegance + rank sophistication
    let base_beauty = 100.0 / (1.0 + complexity);
    let genus_bonus = genus as f64 * 10.0;
    let rank_bonus = rank as f64 * 5.0;
    base_beauty + genus_bonus + rank_bonus
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog_json = fs::read_to_string("rust_type_emoji_catalog.json")?;
    let catalog: TypeCatalog = serde_json::from_str(&catalog_json)?;

    let mut data_rows = Vec::new();

    for entry in catalog.catalog {
        let (genus, rank, torsion) = parse_mathematical_properties(&entry.mathematical_properties);
        let beauty = calculate_mathematical_beauty(entry.complexity_score, genus, rank);

        data_rows.push(DataRow {
            type_name: entry.name,
            emoji: entry.emoji,
            category: entry.category,
            frequency: entry.frequency,
            complexity_score: entry.complexity_score,
            curve_class: entry.curve_class,
            genus,
            rank,
            torsion,
            mathematical_beauty: beauty,
        });
    }

    // Sort by mathematical beauty (most beautiful first)
    data_rows.sort_by(|a, b| b.mathematical_beauty.partial_cmp(&a.mathematical_beauty).unwrap());

    let mut features = HashMap::new();
    features.insert("type_name".to_string(), FeatureType { dtype: "string".to_string() });
    features.insert("emoji".to_string(), FeatureType { dtype: "string".to_string() });
    features.insert("category".to_string(), FeatureType { dtype: "string".to_string() });
    features.insert("frequency".to_string(), FeatureType { dtype: "int32".to_string() });
    features.insert("complexity_score".to_string(), FeatureType { dtype: "float64".to_string() });
    features.insert("curve_class".to_string(), FeatureType { dtype: "string".to_string() });
    features.insert("genus".to_string(), FeatureType { dtype: "int32".to_string() });
    features.insert("rank".to_string(), FeatureType { dtype: "int32".to_string() });
    features.insert("torsion".to_string(), FeatureType { dtype: "string".to_string() });
    features
        .insert("mathematical_beauty".to_string(), FeatureType { dtype: "float64".to_string() });

    let mut feature_specs = HashMap::new();
    feature_specs.insert(
        "type_name".to_string(),
        FeatureSpec {
            dtype: "string".to_string(),
            description: "Rust type name extracted from rustc_driver.so".to_string(),
        },
    );
    feature_specs.insert(
        "emoji".to_string(),
        FeatureSpec {
            dtype: "string".to_string(),
            description: "Visual emoji representation based on mathematical properties".to_string(),
        },
    );
    feature_specs.insert(
        "category".to_string(),
        FeatureSpec {
            dtype: "string".to_string(),
            description: "Type category classification".to_string(),
        },
    );
    feature_specs.insert(
        "frequency".to_string(),
        FeatureSpec {
            dtype: "int32".to_string(),
            description: "Usage frequency in rustc compiler binary".to_string(),
        },
    );
    feature_specs.insert(
        "complexity_score".to_string(),
        FeatureSpec {
            dtype: "float64".to_string(),
            description: "Mathematical complexity score (linear to elliptic progression)"
                .to_string(),
        },
    );
    feature_specs.insert(
        "curve_class".to_string(),
        FeatureSpec {
            dtype: "string".to_string(),
            description: "Algebraic curve classification from LMFDB integration".to_string(),
        },
    );
    feature_specs.insert(
        "genus".to_string(),
        FeatureSpec {
            dtype: "int32".to_string(),
            description: "Algebraic geometry genus from LMFDB mapping".to_string(),
        },
    );
    feature_specs.insert(
        "rank".to_string(),
        FeatureSpec {
            dtype: "int32".to_string(),
            description: "Elliptic curve rank from LMFDB classification".to_string(),
        },
    );
    feature_specs.insert(
        "torsion".to_string(),
        FeatureSpec {
            dtype: "string".to_string(),
            description: "Torsion group structure from algebraic geometry".to_string(),
        },
    );
    feature_specs.insert(
        "mathematical_beauty".to_string(),
        FeatureSpec {
            dtype: "float64".to_string(),
            description: "Computed mathematical beauty score combining complexity, genus, and rank"
                .to_string(),
        },
    );

    let mut splits = HashMap::new();
    splits.insert(
        "train".to_string(),
        SplitInfo {
            name: "train".to_string(),
            num_bytes: (catalog_json.len() as u64 * 2), // Estimate
            num_examples: data_rows.len(),
        },
    );

    let dataset = HuggingFaceDataset {
        dataset_info: DatasetInfo {
            description: "Complete mathematical analysis of 5,724 Rust types extracted from rustc_driver.so binary. First mathematical classification of an entire programming language compiler using algebraic geometry, complexity theory, and visual emoji representations.".to_string(),
            citation: "@misc{rust_mathematical_compiler_2026, title={Mathematical Programming Language Analysis: Complete Classification of Rust Compiler Types}, author={James Mike Dupont <h4@solfunmeme.com>}, year={2026}}".to_string(),
            homepage: "https://github.com/rust-lang/rust".to_string(),
            license: "MIT/Apache-2.0".to_string(),
            features: feature_specs,
        },
        features,
        splits,
        data: data_rows,
    };

    let output = serde_json::to_string_pretty(&dataset)?;
    fs::write("rust_types_huggingface_dataset.json", output)?;

    println!("🤗 Generated Hugging Face dataset with {} types", dataset.data.len());
    println!("📊 Top 5 most mathematically beautiful types:");
    for (i, row) in dataset.data.iter().take(5).enumerate() {
        println!(
            "{}. {} {} - Beauty: {:.2}",
            i + 1,
            row.emoji,
            row.type_name,
            row.mathematical_beauty
        );
    }

    Ok(())
}
