use introspector_collector::mklang;
use introspector_collector::language_meta_system::*;

fn main() {
    println!("🌍 Universal Language Meta-System");
    println!("🔧 Creating decorated tree structures for all programming languages");
    
    let mut registry = LanguageRegistry::new();
    
    // Create language trees using mklang! macro
    println!("\n🚀 Creating language trees...");
    
    // Rust with full specification
    let mut rust_tree = mklang!("rust", {
        paradigm: "systems,functional,imperative",
        memory_safety: "guaranteed",
        concurrency: "fearless"
    }, wikidata: "Q575650", repo: "https://github.com/rust-lang/rust");
    
    rust_tree.decorate("compiler", "rustc")
           .decorate("package_manager", "cargo")
           .decorate("macro_system", "proc_macro");
    
    // Nix with properties
    let mut nix_tree = mklang!("nix", {
        purity: "functional",
        reproducibility: "guaranteed",
        evaluation: "lazy"
    }, wikidata: "Q7041568", repo: "https://github.com/NixOS/nix");
    
    nix_tree.decorate("package_manager", "nixpkgs")
           .decorate("build_system", "derivations");
    
    // Bash
    let bash_tree = mklang!("bash", wikidata: "Q189248", repo: "https://git.savannah.gnu.org/cgit/bash.git");
    
    // Haskell
    let haskell_tree = mklang!("haskell", {
        purity: "pure",
        evaluation: "lazy",
        type_system: "hindley_milner"
    }, wikidata: "Q34010", repo: "https://github.com/ghc/ghc");
    
    // Lean4 for theorem proving
    let lean4_tree = mklang!("lean4", {
        paradigm: "functional,theorem_proving",
        type_system: "dependent",
        proof_assistant: "true"
    }, wikidata: "Q28865", repo: "https://github.com/leanprover/lean4");
    
    // OCaml
    let ocaml_tree = mklang!("ocaml", wikidata: "Q178804", repo: "https://github.com/ocaml/ocaml");
    
    // Coq
    let coq_tree = mklang!("coq", wikidata: "Q1136376", repo: "https://github.com/coq/coq");
    
    // Register all languages
    registry.register(rust_tree);
    registry.register(nix_tree);
    registry.register(bash_tree);
    registry.register(haskell_tree);
    registry.register(lean4_tree);
    registry.register(ocaml_tree);
    registry.register(coq_tree);
    
    println!("✅ Created {} language trees", registry.languages.len());
    
    // Show language properties
    println!("\n📊 Language Properties:");
    for (name, tree) in &registry.languages {
        println!("  🔧 {}: {} paradigm, {} typing", 
                 name, 
                 tree.properties.paradigm.join("+"), 
                 tree.properties.typing);
        println!("     📚 Wikidata: {:?}", tree.wikidata_entry);
        println!("     🏠 Repo: {:?}", tree.official_repo);
        if !tree.decorations.is_empty() {
            println!("     🎨 Decorations: {:?}", 
                     tree.decorations.iter().map(|d| &d.decorator_type).collect::<Vec<_>>());
        }
        println!();
    }
    
    // Generate code examples
    println!("🔧 Code Generation Examples:");
    
    if let Some(rust) = registry.get("rust") {
        let rust_code = rust.generate(
            "fn {function_name}() -> {return_type} {{\n    {body}\n}}",
            [
                ("function_name".to_string(), "hello_world".to_string()),
                ("return_type".to_string(), "String".to_string()),
                ("body".to_string(), "\"Hello, World!\".to_string()".to_string()),
            ].iter().cloned().collect()
        );
        println!("\n🦀 Rust:");
        println!("{}", rust_code);
    }
    
    if let Some(nix) = registry.get("nix") {
        let nix_code = nix.generate(
            "pkgs.stdenv.mkDerivation {{\n  name = \"{package_name}\";\n  src = {src};\n}}",
            [
                ("package_name".to_string(), "my-package".to_string()),
                ("src".to_string(), "./src".to_string()),
            ].iter().cloned().collect()
        );
        println!("\n❄️  Nix:");
        println!("{}", nix_code);
    }
    
    // Show interop examples
    println!("\n🔗 Language Interop:");
    if let (Some(nix), Some(rust)) = (registry.get("nix"), registry.get("rust")) {
        println!("Nix ↔ Rust:");
        println!("{}", nix.interop_with(rust));
    }
    
    if let (Some(bash), Some(nix)) = (registry.get("bash"), registry.get("nix")) {
        println!("\nBash ↔ Nix:");
        println!("{}", bash.interop_with(nix));
    }
    
    // Create polyglot program
    println!("\n🌐 Polyglot Program:");
    let polyglot = registry.create_polyglot_program(vec!["rust", "nix", "bash", "haskell", "lean4"]);
    println!("{}", polyglot);
    
    println!("✨ Universal Language Meta-System Complete!");
    println!("🎯 All languages are now decorated tree structures with:");
    println!("  📊 Properties and metadata");
    println!("  🔗 Wikidata entries for semantic linking");
    println!("  🏠 Official repositories for source access");
    println!("  🎨 Decorations for extended functionality");
    println!("  🔧 Code generation capabilities");
    println!("  🌐 Cross-language interop support");
}
