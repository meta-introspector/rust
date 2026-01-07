// Broken build.rs - commented out for now
/*
use std::{env, fs, path::PathBuf, collections::HashMap};
use syn::{parse_file, Item, ItemEnum, ItemConst, ItemStruct, ItemFn};
use quote::quote;

#[derive(Debug, Clone)]
struct FilterConfig {
    allow_consts: bool,
    allow_enums: bool,
    allow_structs: bool,
    allow_fns: bool,
    max_complexity: usize,
    enum_variant_limit: Option<usize>,
}

impl FilterConfig {
    fn from_env() -> Self {
        Self {
            allow_consts: env::var("FILTER_CONSTS").unwrap_or("true".to_string()) == "true",
            allow_enums: env::var("FILTER_ENUMS").unwrap_or("true".to_string()) == "true", 
            allow_structs: env::var("FILTER_STRUCTS").unwrap_or("true".to_string()) == "true",
            allow_fns: env::var("FILTER_FNS").unwrap_or("true".to_string()) == "true",
            max_complexity: env::var("MAX_COMPLEXITY").unwrap_or("100".to_string()).parse().unwrap_or(100),
            enum_variant_limit: env::var("ENUM_VARIANT_LIMIT").ok().and_then(|s| s.parse().ok()),
        }
    }
}

fn calculate_complexity(item: &Item) -> usize {
    match item {
        Item::Const(_) => 1,
        Item::Enum(e) => e.variants.len(),
        Item::Struct(s) => s.fields.len(),
        Item::Fn(f) => count_statements(&f.block.stmts),
        _ => 0,
    }
}

fn count_statements(stmts: &[syn::Stmt]) -> usize {
    stmts.len() + stmts.iter().map(|stmt| match stmt {
        syn::Stmt::Expr(syn::Expr::Block(block), _) => count_statements(&block.block.stmts),
        syn::Stmt::Expr(syn::Expr::If(if_expr), _) => {
            1 + count_statements(&if_expr.then_branch.stmts) +
            if_expr.else_branch.as_ref().map_or(0, |(_, else_expr)| match else_expr.as_ref() {
                syn::Expr::Block(block) => count_statements(&block.block.stmts),
                _ => 1,
            })
        },
        _ => 0,
    }).sum::<usize>()
}

fn should_include_item(item: &Item, config: &FilterConfig) -> bool {
    let complexity = calculate_complexity(item);
    if complexity > config.max_complexity {
        return false;
    }

    match item {
        Item::Const(_) => config.allow_consts,
        Item::Enum(e) => {
            config.allow_enums && 
            config.enum_variant_limit.map_or(true, |limit| e.variants.len() <= limit)
        },
        Item::Struct(_) => config.allow_structs,
        Item::Fn(_) => config.allow_fns,
        _ => true, // Allow other items by default
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=FILTER_CONSTS");
    println!("cargo:rerun-if-env-changed=FILTER_ENUMS");
    println!("cargo:rerun-if-env-changed=FILTER_STRUCTS");
    println!("cargo:rerun-if-env-changed=FILTER_FNS");
    println!("cargo:rerun-if-env-changed=MAX_COMPLEXITY");
    println!("cargo:rerun-if-env-changed=ENUM_VARIANT_LIMIT");

    let config = FilterConfig::from_env();
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let src_dir = manifest_dir.join("src");

    fs::create_dir_all(&out_dir)?;

    let mut filtered_content = String::new();
    let mut stats = HashMap::new();

    for entry in fs::read_dir(&src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            println!("cargo:rerun-if-changed={}", path.display());
            let code = fs::read_to_string(&path)?;
            let ast: syn::File = parse_file(&code)?;

            let mut filtered_items = Vec::new();
            let mut total_items = 0;
            let mut filtered_items_count = 0;

            for item in ast.items {
                total_items += 1;
                if should_include_item(&item, &config) {
                    filtered_items.push(item);
                    filtered_items_count += 1;
                } else {
                    let item_type = match &item {
                        Item::Const(_) => "const",
                        Item::Enum(_) => "enum", 
                        Item::Struct(_) => "struct",
                        Item::Fn(_) => "fn",
                        _ => "other",
                    };
                    *stats.entry(format!("filtered_{}", item_type)).or_insert(0) += 1;
                }
            }

            if !filtered_items.is_empty() {
                let file_stem = path.file_stem().unwrap().to_string_lossy();
                let filtered_file = quote! { #(#filtered_items)* };
                filtered_content.push_str(&format!("// Filtered {}: {}/{} items\n", 
                    file_stem, filtered_items_count, total_items));
                filtered_content.push_str(&filtered_file.to_string());
                filtered_content.push('\n');
            }
        }
    }

    // Write filtered code
    fs::write(out_dir.join("filtered.rs"), &filtered_content)?;

    // Write filter stats
    let stats_content = format!("// Filter Stats: {:?}\n// Config: {:?}\n", stats, config);
    fs::write(out_dir.join("filter_stats.rs"), stats_content)?;

    // Generate lib.rs that includes filtered code
    let lib_content = format!(r#"
// Bandwidth-filtered compilation
// Config: {:?}
include!(concat!(env!("OUT_DIR"), "/filtered.rs"));
"#, config);
    
    fs::write(out_dir.join("lib.rs"), lib_content)?;

    println!("Bandwidth filter applied: {:?}", config);
    for (key, value) in stats {
        println!("cargo:warning=Filtered {}: {}", key, value);
    }

    Ok(())
}
*/

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("Build script disabled");
}
