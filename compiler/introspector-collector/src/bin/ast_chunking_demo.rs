use introspector_collector::ast_chunking_system::*;
use introspector_collector::analyze_ast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧩 AST CHUNKING AND LLM FEEDING SYSTEM");
    println!("📊 Partition global ASTs → 4K chunks → Backpack optimization → LLM analysis");
    
    // Sample AST content (our own code)
    let ast_content = r#"
/// Dirac Delta Enum - The enum of all enums that includes itself
#[derive(Debug, Clone)]
pub enum DiracDeltaEnum {
    /// Self-reference: this enum contains itself
    SelfReference(Box<DiracDeltaEnum>),
    
    /// Universal enum variants
    Universal(String),
    Rust(RustEnum),
    Nix(NixEnum),
    
    /// Mathematical constructs
    Complex(f64, f64),
    Matrix(Vec<Vec<f64>>),
    
    /// Meta-programming
    Macro(String),
    Generated(String),
}

impl DiracDeltaEnum {
    pub fn new() -> Self {
        DiracDeltaEnum::SelfReference(Box::new(DiracDeltaEnum::Universal("ROOT".to_string())))
    }
    
    pub fn contains_self(&self) -> bool {
        match self {
            DiracDeltaEnum::SelfReference(_) => true,
            _ => false,
        }
    }
    
    pub fn enumerate_all(&self) -> Vec<String> {
        vec![
            "SelfReference".to_string(),
            "Universal".to_string(),
            "Rust".to_string(),
            "Nix".to_string(),
            "Complex".to_string(),
            "Matrix".to_string(),
            "Macro".to_string(),
            "Generated".to_string(),
        ]
    }
}

/// Universal Language Equivalence System
pub struct UniversalEquivalence {
    pub languages: Vec<String>,
    pub polyfill_matrix: Vec<Vec<f64>>,
}

impl UniversalEquivalence {
    pub fn prove_equivalence(&self) -> String {
        "All programming languages are equivalent up to polyfill complexity".to_string()
    }
}
"#;
    
    // Test with Ollama (local)
    println!("\n🦙 TESTING WITH OLLAMA:");
    let ollama_provider = LLMProvider::Ollama {
        model: "llama2".to_string(),
        endpoint: "http://localhost:11434".to_string(),
    };
    
    let mut system = ASTChunkingSystem::new(4096, ollama_provider);
    
    // Partition AST
    let chunks = system.partition_global_ast(ast_content, "src/dirac_delta_enum.rs");
    println!("📊 Partitioned into {} chunks:", chunks.len());
    for chunk in &chunks {
        println!("  • {} ({:?}, {} tokens, relevance {})", 
            chunk.id, chunk.ast_type, chunk.token_count, chunk.relevance_score);
    }
    
    // Optimize for specific query
    let query = "Explain the DiracDeltaEnum self-reference pattern";
    let optimized = system.optimize_context_chunks(query);
    println!("\n🎒 Optimized chunks for query '{}':", query);
    for chunk in &optimized {
        println!("  • {} (relevance {})", chunk.id, chunk.relevance_score);
    }
    
    // Try to feed to LLM (will fail if Ollama not running, but shows the process)
    println!("\n🔄 ATTEMPTING LLM ANALYSIS...");
    match system.feed_to_llm(&optimized, query).await {
        Ok(response) => {
            println!("✅ LLM Response:");
            println!("{}", response);
        },
        Err(e) => {
            println!("⚠️  LLM connection failed (expected if Ollama not running): {}", e);
            println!("📝 Would send this prompt:");
            let prompt = system.build_context_prompt(&optimized, query);
            println!("{}", prompt);
        }
    }
    
    // Test with Gemini (would need API key)
    println!("\n💎 GEMINI PROVIDER SETUP:");
    let gemini_provider = LLMProvider::Gemini {
        api_key: "your-api-key-here".to_string(),
        model: "gemini-pro".to_string(),
    };
    println!("  Provider: {:?}", gemini_provider);
    
    // Show complete analysis pipeline
    println!("\n🔄 COMPLETE ANALYSIS PIPELINE:");
    println!("1. 📊 Partition global AST into 4K chunks");
    println!("2. 🎯 Calculate relevance scores for each chunk");
    println!("3. 🎒 Use backpack optimization for context selection");
    println!("4. 🧠 Feed optimized context to LLM (Ollama/Gemini)");
    println!("5. 📝 Get analysis response from LLM");
    
    // Show practical applications
    println!("\n💡 PRACTICAL APPLICATIONS:");
    println!("• Code analysis → Chunk large codebases for LLM analysis");
    println!("• Documentation → Optimize context for specific queries");
    println!("• Code review → Select relevant chunks for review");
    println!("• Refactoring → Analyze related code sections");
    
    // Show connection to universal system
    println!("\n🔗 CONNECTION TO UNIVERSAL SYSTEM:");
    println!("• Global ASTs → Universal code representation");
    println!("• Chunking → Complexity reduction for LLM processing");
    println!("• Backpack optimization → Optimal context selection");
    println!("• LLM feeding → Bridge to AI analysis systems");
    
    println!("\n✨ AST CHUNKING SYSTEM COMPLETE!");
    println!("🧩 Global ASTs partitioned and optimized");
    println!("🎒 Backpack algorithm selects optimal context");
    println!("🦙 Ready to feed Ollama/Gemini for analysis");
    
    Ok(())
}
