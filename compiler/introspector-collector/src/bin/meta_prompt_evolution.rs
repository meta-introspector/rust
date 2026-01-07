use introspector_collector::meta_prompt_system::*;

fn main() {
    println!("🧬 META-PROMPT EVOLUTION SYSTEM");
    println!("🤖 Wrapping LLMs in macro system with self-contained Nix experiments");
    println!("🌐 Network-isolated execution with results captured to Nix store");
    
    let mut meta_system = MetaPromptSystem::new();
    
    // Sample code snippets to evolve prompts for
    let code_snippets = vec![
        r#"enum Color { Red, Green, Blue }"#.to_string(),
        r#"enum Option<T> { None, Some(T) }"#.to_string(),
        r#"enum Result<T, E> { Ok(T), Err(E) }"#.to_string(),
        r#"enum Expr { 
            Literal(i32), 
            Variable(String), 
            Add(Box<Expr>, Box<Expr>) 
        }"#.to_string(),
    ];
    
    println!("\n🚀 Starting meta-prompt evolution for {} code snippets...", code_snippets.len());
    
    // Evolve prompts and create Nix experiments
    let experiments = meta_system.evolve_meta_prompts(code_snippets);
    
    println!("\n📊 Evolution Results:");
    for (i, experiment) in experiments.iter().enumerate() {
        println!("  🧪 Experiment {}: {}", i + 1, experiment.id);
        println!("     📁 Path: experiments/{}", experiment.id);
        println!("     🌐 Network: {}", if experiment.network_allowed { "Allowed (single port)" } else { "Disabled" });
        println!("     📦 Store: {}", experiment.output_path);
    }
    
    // Show evolved prompts
    println!("\n🔄 Evolved Prompts:");
    for prompt in &meta_system.evolved_prompts {
        println!("  📝 {}: Generation {}, Fitness {:.3}", 
                 prompt.id, prompt.generation, prompt.fitness_score);
    }
    
    // Show LLM wrappers
    println!("\n🤖 LLM Wrappers:");
    for wrapper in &meta_system.llm_wrappers {
        println!("  🔧 {}: {} → {}", 
                 wrapper.name, wrapper.input_schema, wrapper.output_schema);
    }
    
    println!("\n{}", meta_system.generate_experiment_summary());
    
    println!("\n🎯 Next Steps:");
    println!("1. Run individual experiments: ./experiments/nix_experiment_*/doit.sh");
    println!("2. Build all Nix derivations: nix-build experiments/*/default.nix");
    println!("3. Inspect results in Nix store: ls /nix/store/*experiment*");
    println!("4. Evolve prompts further based on results");
    
    println!("\n✨ Meta-Prompt Evolution System Ready!");
    println!("🧬 LLMs wrapped in macros, containerized in Nix, ready for evolution!");
}
