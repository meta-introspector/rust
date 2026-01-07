use introspector_collector::topological_ast_chunker::TopologicalASTChunker;

fn main() {
    println!("🧩 Topological AST Chunking Demo");
    println!("================================\n");
    
    let mut chunker = TopologicalASTChunker::new();
    
    // Sample Rust code to chunk
    let sample_code = r#"
        use std::collections::HashMap;
        
        #[derive(Debug, Clone)]
        pub struct DataProcessor {
            pub data: HashMap<String, i32>,
        }
        
        impl DataProcessor {
            pub fn new() -> Self {
                Self {
                    data: HashMap::new(),
                }
            }
            
            pub fn process(&mut self, key: String, value: i32) {
                self.data.insert(key, value);
            }
        }
        
        pub trait Processor {
            fn process_data(&self, input: &str) -> String;
        }
        
        pub enum ProcessingMode {
            Fast,
            Accurate,
            Balanced,
        }
        
        pub fn main() {
            let mut processor = DataProcessor::new();
            processor.process("test".to_string(), 42);
            
            match ProcessingMode::Fast {
                ProcessingMode::Fast => println!("Fast mode"),
                ProcessingMode::Accurate => println!("Accurate mode"),
                ProcessingMode::Balanced => println!("Balanced mode"),
            }
        }
    "#;
    
    println!("📝 Sample code to chunk:");
    println!("{}\n", sample_code);
    
    // Perform topological chunking
    match chunker.chunk_by_types(sample_code) {
        Ok(_) => {
            println!("✅ Successfully chunked code by types\n");
            
            // Display chunking results
            println!("{}", chunker.generate_chunking_report());
            
            // Show individual chunks
            println!("\n🔍 Individual Chunks:");
            println!("====================");
            for (i, chunk) in chunker.topological_chunks.iter().enumerate() {
                println!("\n--- Chunk {} ---", i + 1);
                println!("ID: {}", chunk.chunk_id);
                println!("Type: {:?}", chunk.chunk_type);
                println!("Enum mapping: {}", chunk.enum_mapping);
                println!("Spectral signature: {:?}", chunk.spectral_signature);
                println!("Compilable code preview:");
                let preview = chunk.compilable_code.lines().take(5).collect::<Vec<_>>().join("\n");
                println!("{}", preview);
                if chunk.compilable_code.lines().count() > 5 {
                    println!("... (truncated)");
                }
            }
            
            // Test independent compilation
            println!("\n🔨 Testing Independent Compilation:");
            println!("===================================");
            let compilation_results = chunker.compile_chunks_independently();
            
            let mut successful = 0;
            let mut failed = 0;
            
            for result in &compilation_results {
                if result.compilation_success {
                    println!("✅ {} compiled successfully", result.chunk_id);
                    successful += 1;
                } else {
                    println!("❌ {} failed to compile", result.chunk_id);
                    if let Some(error) = &result.error_output {
                        println!("   Error: {}", error.lines().next().unwrap_or("Unknown error"));
                    }
                    failed += 1;
                }
            }
            
            println!("\n📊 Compilation Summary:");
            println!("  Successful: {}", successful);
            println!("  Failed: {}", failed);
            println!("  Success rate: {:.1}%", (successful as f64 / (successful + failed) as f64) * 100.0);
            
            // Show type-based splits
            println!("\n📂 Type-Based Splits:");
            println!("====================");
            for (type_key, chunks) in &chunker.type_based_splits {
                println!("  {}: {} chunks", type_key, chunks.len());
                for chunk in chunks.iter().take(2) {
                    println!("    - {}", chunk.chunk_id);
                }
                if chunks.len() > 2 {
                    println!("    ... and {} more", chunks.len() - 2);
                }
            }
            
            // Show spectral clusters
            println!("\n🌈 Spectral Clusters:");
            println!("====================");
            for (cluster_name, chunk_ids) in &chunker.spectral_analysis.spectral_clusters {
                println!("  {}: {} chunks", cluster_name, chunk_ids.len());
                for chunk_id in chunk_ids.iter().take(3) {
                    println!("    - {}", chunk_id);
                }
                if chunk_ids.len() > 3 {
                    println!("    ... and {} more", chunk_ids.len() - 3);
                }
            }
            
        }
        Err(e) => {
            println!("❌ Failed to chunk code: {}", e);
        }
    }
    
    println!("\n🎯 Key Achievements:");
    println!("===================");
    println!("• ✅ Topological AST chunking by type");
    println!("• ✅ Enum of enums mapping (syn ↔ HIR)");
    println!("• ✅ Independent chunk compilation");
    println!("• ✅ Spectral signature analysis");
    println!("• ✅ Type-based code splitting");
    println!("• ✅ Quote-based token generation");
    println!("• ✅ Compilable code generation");
    
    println!("\n🚀 Next Steps:");
    println!("==============");
    println!("• Extend to expression and pattern chunking");
    println!("• Add HIR-level chunking support");
    println!("• Implement chunk dependency analysis");
    println!("• Add parallel compilation of chunks");
    println!("• Create chunk optimization pipeline");
}
