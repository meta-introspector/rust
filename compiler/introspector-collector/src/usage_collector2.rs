impl UsageCollector {
    fn new() -> Self {
        Self {
            module_data: HashMap::new(),
            enum_data: HashMap::new(),
            item_complexity: HashMap::new(),
        }
    }
    
    fn get_defkind_from_item<'tcx>(&self, tcx: TyCtxt<'tcx>, item: &rustc_hir::Item) -> String {
        match &item.kind {
            rustc_hir::ItemKind::Const(_, _, _, body_id) => {
                let body = tcx.hir_body(*body_id);
                match &body.value.kind {
                    rustc_hir::ExprKind::Lit(lit) => match lit.node {
                        rustc_ast::LitKind::Str(..) => "StringLiteral",
                        rustc_ast::LitKind::Int(..) => "IntLiteral", 
                        rustc_ast::LitKind::Float(..) => "FloatLiteral",
                        rustc_ast::LitKind::Bool(..) => "BoolLiteral",
                        _ => "Const"
                    },
                    _ => "Const"
                }
            },
            rustc_hir::ItemKind::Static(..) => "Static",
            rustc_hir::ItemKind::Struct(..) => "Struct",
            rustc_hir::ItemKind::Enum(..) => "Enum",
            rustc_hir::ItemKind::Fn(..) => "Fn",
            _ => "Other"
        }.to_string()
    }
    

    }
    
    fn classify_usage(&self, usage: &str, usage_type: &str, _used_def_id: &str) -> UsageClassification {
        let mut classification = UsageClassification::default();
        
        // String conversion patterns
        if usage.contains("to_string") || usage.contains("Display") || usage.contains("fmt") || 
           usage.contains("format!") || usage.contains("write!") {
            classification.string_conversion = 1;
        }
        // Pattern matching
        else if usage.contains("match") || usage.contains("if let") || usage_type == "PatternMatch" {
            classification.pattern_matching = 1;
        }
        // Construction
        else if usage.contains("::") && !usage.contains("(") {
            classification.construction = 1;
        }
        // Comparison
        else if usage.contains("==") || usage.contains("!=") || usage.contains("cmp") {
            classification.comparison = 1;
        }
        // Debug formatting
        else if usage.contains("Debug") || usage.contains("{:?}") || usage.contains("dbg!") {
            classification.debug_format = 1;
        }
        // Serialization
        else if usage.contains("serde") || usage.contains("serialize") || usage.contains("json") {
            classification.serialization = 1;
        }
        // General usage
        else {
            classification.general = 1;
        }
        
        classification
    }
    
    fn extract_enum_variant(&self, def_id: &str) -> Option<(String, String)> {
        // Parse DefId format to extract enum and variant names
        if let Some(path_part) = def_id.strip_prefix("DefId(").and_then(|s| s.split(" ~ ").nth(1)) {
            if let Some(clean_path) = path_part.strip_suffix(")") {
                let parts: Vec<&str> = clean_path.split("::").collect();
                if parts.len() >= 2 {
                    let variant = parts[parts.len() - 1].to_string();
                    let enum_name = parts[parts.len() - 2].to_string();
                    
                    // Check if this looks like an enum variant (not a function)
                    if !variant.contains("(") && !variant.contains("<") && 
                       (variant.chars().next().unwrap_or('a').is_uppercase() || 
                        variant == "true" || variant == "false") {
                        return Some((enum_name, variant));
                    }
                }
            }
        }
        None
    }
    
    fn update_enum_usage(&mut self, enum_name: String, variant_name: String, 
                        classification: UsageClassification, usage_type: &str, converter_fn: &str) {
        let enum_info = self.enum_data.entry(enum_name.clone()).or_insert_with(|| EnumInfo {
            name: enum_name.clone(),
            variants: Vec::new(),
            total_usage_classes: UsageClassification::default(),
        });
        
        // Find or create variant
        if let Some(variant) = enum_info.variants.iter_mut().find(|v| v.variant_name == variant_name) {
            // Update existing variant
            variant.usage_classes.string_conversion += classification.string_conversion;
            variant.usage_classes.pattern_matching += classification.pattern_matching;
            variant.usage_classes.construction += classification.construction;
            variant.usage_classes.comparison += classification.comparison;
            variant.usage_classes.debug_format += classification.debug_format;
            variant.usage_classes.serialization += classification.serialization;
            variant.usage_classes.general += classification.general;
            
            // Track top converter functions (keep top 3)
            let converters = variant.top_converters.entry(usage_type.to_string()).or_insert_with(Vec::new);
            if let Some(existing) = converters.iter_mut().find(|(fn_name, _)| fn_name == converter_fn) {
                existing.1 += 1;
            } else {
                converters.push((converter_fn.to_string(), 1));
            }
            converters.sort_by(|a, b| b.1.cmp(&a.1));
            converters.truncate(3); // Keep top 3
        } else {
            // Create new variant
            let mut top_converters = HashMap::new();
            top_converters.insert(usage_type.to_string(), vec![(converter_fn.to_string(), 1)]);
            
            enum_info.variants.push(EnumVariantUsage {
                enum_name: enum_name.clone(),
                variant_name: variant_name.clone(),
                usage_classes: classification.clone(),
                top_converters,
            });
        }
        
        // Update total usage for enum
        enum_info.total_usage_classes.string_conversion += classification.string_conversion;
        enum_info.total_usage_classes.pattern_matching += classification.pattern_matching;
        enum_info.total_usage_classes.construction += classification.construction;
        enum_info.total_usage_classes.comparison += classification.comparison;
        enum_info.total_usage_classes.debug_format += classification.debug_format;
        enum_info.total_usage_classes.serialization += classification.serialization;
        enum_info.total_usage_classes.general += classification.general;
    }
    
    fn save_to_files(&self, crate_name: &str) {
        // Use proper Cargo environment variables for target directory
        let output_dir = std::env::var("USAGE_OUTPUT_DIR")
            .or_else(|_| {
                // Try CARGO_TARGET_DIR first (user override)
                std::env::var("CARGO_TARGET_DIR")
                    .map(|target_dir| format!("{}/harmonic/usage", target_dir))
            })
            .or_else(|_| {
                // Fall back to calculated target path using CARGO_MANIFEST_DIR + PROFILE
                std::env::var("CARGO_MANIFEST_DIR")
                    .and_then(|manifest_dir| {
                        std::env::var("PROFILE")
                            .map(|profile| format!("{}/target/{}/harmonic/usage", manifest_dir, profile))
                    })
            })
            .unwrap_or_else(|_| "usage_data".to_string());
        
        std::fs::create_dir_all(&output_dir).unwrap();
        
        let mut total_usages = 0;
        let mut generated_files = Vec::new();
        
        for (module, usages) in &self.module_data {
            let module_data = ModuleData {
                crate_name: crate_name.to_string(),
                module: module.clone(),
                usages: usages.clone(),
            };
            
            // Clean filename by removing invalid characters
            let module_clean = module
                .replace("::", "_")
                .replace("<", "_")
                .replace(">", "_")
                .replace(" ", "_")
                .replace("/", "_")
                .replace("\\", "_")
                .replace("*", "_")
                .replace("?", "_")
                .replace("\"", "_")
                .replace("|", "_")
                .replace("'", "_")
                .replace("#", "_")
                .replace("{", "_")
                .replace("}", "_")
                .replace("(", "_")
                .replace(")", "_")
                .replace("[", "_")
                .replace("]", "_")
                .replace("&", "_")
                .replace("$", "_")
                .replace("@", "_")
                .replace("!", "_")
                .replace("%", "_")
                .replace("^", "_")
                .replace("+", "_")
                .replace("=", "_")
                .replace("~", "_")
                .replace("`", "_")
                .replace(";", "_")
                .replace(",", "_")
                .replace(".", "_");
            
            let filename = if module_clean.len() > 100 {
                let hash = std::collections::hash_map::DefaultHasher::new();
                use std::hash::{Hash, Hasher};
                let mut hasher = hash;
                module.hash(&mut hasher);
                format!("{}/{}_{:x}.json", output_dir, crate_name, hasher.finish())
            } else {
                format!("{}/{}_{}.json", output_dir, crate_name, module_clean)
            };
            
            let json = serde_json::to_string_pretty(&module_data).unwrap();
            std::fs::write(&filename, json).unwrap();
            generated_files.push(filename.clone());
            total_usages += usages.len();
        }
        
        // Save complexity data
        if !self.item_complexity.is_empty() {
            let complexity_filename = format!("{}/{}_complexity.json", output_dir, crate_name);
            let complexity_data = serde_json::json!({
                "crate": crate_name,
                "items": self.item_complexity.values().collect::<Vec<_>>()
            });
            let complexity_json = serde_json::to_string_pretty(&complexity_data).unwrap();
            std::fs::write(&complexity_filename, complexity_json).unwrap();
            generated_files.push(complexity_filename);
            eprintln!("=== SAVED {} COMPLEXITY ITEMS FOR CRATE: {} ===", self.item_complexity.len(), crate_name);
        }
        
        // Generate manifest for this crate
        self.generate_manifest(crate_name, &output_dir, &generated_files, total_usages);
        
        // eprintln!("=== COLLECTING USAGE DATA FOR CRATE: {} === ({} total usages)", crate_name, total_usages);
    }
    
    fn generate_manifest(&self, crate_name: &str, output_dir: &str, generated_files: &[String], total_usages: usize) {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        // Find other manifest files (dependencies)
        let mut dependency_manifests = Vec::new();
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with("_manifest.json") && !filename.starts_with(&format!("{}_", crate_name)) {
                        dependency_manifests.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
        
        let manifest = serde_json::json!({
            "crate_name": crate_name,
            "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "collector_version": "1.0.0",
            "rustc_version": std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
            "output_directory": output_dir,
            "total_usages": total_usages,
            "total_modules": self.module_data.len(),
            "total_enums": self.enum_data.len(),
            "generated_files": generated_files.iter().map(|f| {
                serde_json::json!({
                    "path": f,
                    "size_bytes": std::fs::metadata(f).map(|m| m.len()).unwrap_or(0),
                    "type": if f.contains("_enums_classified") { "enum_classification" } else { "usage_data" }
                })
            }).collect::<Vec<_>>(),
            "dependency_manifests": dependency_manifests,
            "environment": {
                "pwd": std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string()),
                "cargo_manifest_dir": std::env::var("CARGO_MANIFEST_DIR").ok(),
                "cargo_target_dir": std::env::var("CARGO_TARGET_DIR").ok(),
                "profile": std::env::var("PROFILE").ok()
            }
        });
        
        let manifest_path = format!("{}/{}_manifest.json", output_dir, crate_name);
        std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap()).unwrap();
        // eprintln!("=== MANIFEST SAVED: {} ===", manifest_path);
    }// generate manifest
