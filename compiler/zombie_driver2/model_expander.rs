use std::collections::{HashMap, HashSet};
use std::fs;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, ItemFn, ItemStruct, ItemEnum, ItemConst};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExpandingModel {
    concepts: HashMap<String, ConceptNode>,
    type_graph: HashMap<String, HashSet<String>>,
    pattern_library: HashMap<String, u32>,
    total_files_processed: u32,
    model_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConceptNode {
    name: String,
    frequency: u32,
    connections: HashSet<String>,
    semantic_weight: f64,
    last_seen: String,
}

struct ModelExpander {
    model: ExpandingModel,
    batch_size: usize,
}

impl ModelExpander {
    fn new() -> Self {
        let model = Self::load_or_create_model();
        Self { model, batch_size: 100 }
    }

    fn load_or_create_model() -> ExpandingModel {
        if let Ok(data) = fs::read_to_string("expanding_model.json") {
            serde_json::from_str(&data).unwrap_or_else(|_| Self::create_empty_model())
        } else {
            Self::create_empty_model()
        }
    }

    fn create_empty_model() -> ExpandingModel {
        ExpandingModel {
            concepts: HashMap::new(),
            type_graph: HashMap::new(),
            pattern_library: HashMap::new(),
            total_files_processed: 0,
            model_version: 1,
        }
    }

    fn ingest_directory(&mut self, dir_path: &str) {
        let mut batch = Vec::new();
        
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                if let Ok(path) = entry.path().canonicalize() {
                    if path.extension().map_or(false, |ext| ext == "rs") {
                        batch.push(path);
                        if batch.len() >= self.batch_size {
                            self.process_batch(&batch);
                            batch.clear();
                            self.save_model();
                        }
                    }
                }
            }
        }

        if !batch.is_empty() {
            self.process_batch(&batch);
            self.save_model();
        }
    }

    fn process_batch(&mut self, files: &[std::path::PathBuf]) {
        for file in files {
            if let Ok(content) = fs::read_to_string(file) {
                if let Ok(ast) = parse_file(&content) {
                    let mut visitor = ConceptVisitor::new();
                    visitor.visit_file(&ast);
                    self.integrate_concepts(visitor.concepts);
                    self.model.total_files_processed += 1;
                }
            }
        }
        self.model.model_version += 1;
    }

    fn integrate_concepts(&mut self, new_concepts: Vec<(String, String)>) {
        for (name, concept_type) in new_concepts {
            let entry = self.model.concepts.entry(name.clone()).or_insert_with(|| {
                ConceptNode {
                    name: name.clone(),
                    frequency: 0,
                    connections: HashSet::new(),
                    semantic_weight: 1.0,
                    last_seen: chrono::Utc::now().to_rfc3339(),
                }
            });
            
            entry.frequency += 1;
            entry.semantic_weight = (entry.frequency as f64).ln() + 1.0;
            entry.last_seen = chrono::Utc::now().to_rfc3339();

            self.model.pattern_library.entry(concept_type).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    fn save_model(&self) {
        let json = serde_json::to_string_pretty(&self.model).unwrap();
        fs::write("expanding_model.json", json).unwrap();
        println!("📊 Model saved: {} concepts, {} files processed, version {}", 
                 self.model.concepts.len(), 
                 self.model.total_files_processed,
                 self.model.model_version);
    }

    fn expand_to_new_sources(&mut self, sources: Vec<&str>) {
        for source in sources {
            println!("🔍 Ingesting: {}", source);
            self.ingest_directory(source);
        }
    }
}

struct ConceptVisitor {
    concepts: Vec<(String, String)>,
}

impl ConceptVisitor {
    fn new() -> Self {
        Self { concepts: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ConceptVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.concepts.push((node.sig.ident.to_string(), "Function".to_string()));
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.concepts.push((node.ident.to_string(), "Struct".to_string()));
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        self.concepts.push((node.ident.to_string(), "Enum".to_string()));
        syn::visit::visit_item_enum(self, node);
    }

    fn visit_item_const(&mut self, node: &'ast ItemConst) {
        self.concepts.push((node.ident.to_string(), "Const".to_string()));
        syn::visit::visit_item_const(self, node);
    }
}

fn main() {
    println!("🚀 MODEL EXPANDER - Continuous Code Ingestion");
    println!("============================================");

    let mut expander = ModelExpander::new();
    
    // Expand to active repositories from our analysis
    let sources = vec![
        "/mnt/data1/nix/vendor/rust",
        "/mnt/data1/meta-introspector", 
        ".",
        // Add the 69 recently active repos from personal_repo_identifier
        "/mnt/data1/meta-introspector/com/github",
        "/mnt/data1/meta-introspector/io/github",
    ];

    expander.expand_to_new_sources(sources);

    println!("\n📈 EXPANSION COMPLETE");
    println!("Final model: {} concepts across {} files", 
             expander.model.concepts.len(),
             expander.model.total_files_processed);
}
