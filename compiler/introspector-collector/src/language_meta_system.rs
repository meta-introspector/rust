/// Universal Language Meta-System
/// mklang!("language", properties) creates decorated tree structures for any language

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LanguageTree {
    pub name: String,
    pub properties: LanguageProperties,
    pub syntax_tree: SyntaxTree,
    pub wikidata_entry: Option<String>,
    pub official_repo: Option<String>,
    pub decorations: Vec<Decoration>,
}

#[derive(Debug, Clone)]
pub struct LanguageProperties {
    pub paradigm: Vec<String>,
    pub typing: String,
    pub compilation: String,
    pub runtime: String,
    pub ecosystem: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub root: SyntaxNode,
    pub nodes: HashMap<String, SyntaxNode>,
}

#[derive(Debug, Clone)]
pub struct SyntaxNode {
    pub id: String,
    pub node_type: String,
    pub children: Vec<String>,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Decoration {
    pub decorator_type: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
}

/// The mklang! macro system
#[macro_export]
macro_rules! mklang {
    // Basic language creation
    ($lang:literal) => {
        mklang!($lang, {})
    };
    
    // Language with properties
    ($lang:literal, { $($key:ident: $value:expr),* }) => {{
        let mut properties = std::collections::HashMap::new();
        $(
            properties.insert(stringify!($key).to_string(), $value.to_string());
        )*
        
        $crate::language_meta_system::create_language_tree($lang, properties)
    }};
    
    // Language with wikidata and repo
    ($lang:literal, wikidata: $wd:literal, repo: $repo:literal) => {{
        let mut tree = mklang!($lang);
        tree.wikidata_entry = Some($wd.to_string());
        tree.official_repo = Some($repo.to_string());
        tree
    }};
    
    // Full specification
    ($lang:literal, { $($key:ident: $value:expr),* }, wikidata: $wd:literal, repo: $repo:literal) => {{
        let mut tree = mklang!($lang, { $($key: $value),* });
        tree.wikidata_entry = Some($wd.to_string());
        tree.official_repo = Some($repo.to_string());
        tree
    }};
}

pub fn create_language_tree(name: &str, properties: HashMap<String, String>) -> LanguageTree {
    let lang_properties = match name {
        "rust" => LanguageProperties {
            paradigm: vec!["systems".to_string(), "functional".to_string(), "imperative".to_string()],
            typing: "static".to_string(),
            compilation: "ahead-of-time".to_string(),
            runtime: "native".to_string(),
            ecosystem: "cargo".to_string(),
            version: "1.91.1".to_string(),
        },
        "nix" => LanguageProperties {
            paradigm: vec!["functional".to_string(), "declarative".to_string()],
            typing: "dynamic".to_string(),
            compilation: "interpreted".to_string(),
            runtime: "nix-evaluator".to_string(),
            ecosystem: "nixpkgs".to_string(),
            version: "2.18".to_string(),
        },
        "bash" => LanguageProperties {
            paradigm: vec!["scripting".to_string(), "imperative".to_string()],
            typing: "untyped".to_string(),
            compilation: "interpreted".to_string(),
            runtime: "shell".to_string(),
            ecosystem: "unix".to_string(),
            version: "5.1".to_string(),
        },
        "haskell" => LanguageProperties {
            paradigm: vec!["functional".to_string(), "lazy".to_string()],
            typing: "static".to_string(),
            compilation: "ahead-of-time".to_string(),
            runtime: "ghc-runtime".to_string(),
            ecosystem: "hackage".to_string(),
            version: "9.4".to_string(),
        },
        "lean4" => LanguageProperties {
            paradigm: vec!["functional".to_string(), "theorem-proving".to_string()],
            typing: "dependent".to_string(),
            compilation: "ahead-of-time".to_string(),
            runtime: "lean-runtime".to_string(),
            ecosystem: "mathlib".to_string(),
            version: "4.0".to_string(),
        },
        _ => LanguageProperties {
            paradigm: vec!["unknown".to_string()],
            typing: "unknown".to_string(),
            compilation: "unknown".to_string(),
            runtime: "unknown".to_string(),
            ecosystem: "unknown".to_string(),
            version: "unknown".to_string(),
        },
    };
    
    let syntax_tree = create_syntax_tree(name);
    
    LanguageTree {
        name: name.to_string(),
        properties: lang_properties,
        syntax_tree,
        wikidata_entry: get_wikidata_entry(name),
        official_repo: get_official_repo(name),
        decorations: vec![],
    }
}

fn create_syntax_tree(lang: &str) -> SyntaxTree {
    let mut nodes = HashMap::new();
    
    let root = match lang {
        "rust" => SyntaxNode {
            id: "rust_root".to_string(),
            node_type: "crate".to_string(),
            children: vec!["items".to_string(), "attributes".to_string()],
            properties: HashMap::from([
                ("edition".to_string(), "2021".to_string()),
                ("crate_type".to_string(), "bin".to_string()),
            ]),
        },
        "nix" => SyntaxNode {
            id: "nix_root".to_string(),
            node_type: "expression".to_string(),
            children: vec!["derivation".to_string(), "function".to_string(), "attrset".to_string()],
            properties: HashMap::from([
                ("purity".to_string(), "pure".to_string()),
                ("evaluation".to_string(), "lazy".to_string()),
            ]),
        },
        "bash" => SyntaxNode {
            id: "bash_root".to_string(),
            node_type: "script".to_string(),
            children: vec!["commands".to_string(), "functions".to_string(), "variables".to_string()],
            properties: HashMap::from([
                ("shebang".to_string(), "#!/bin/bash".to_string()),
                ("set".to_string(), "-e".to_string()),
            ]),
        },
        _ => SyntaxNode {
            id: format!("{}_root", lang),
            node_type: "program".to_string(),
            children: vec!["statements".to_string()],
            properties: HashMap::new(),
        },
    };
    
    nodes.insert(root.id.clone(), root.clone());
    
    SyntaxTree { root, nodes }
}

fn get_wikidata_entry(lang: &str) -> Option<String> {
    match lang {
        "rust" => Some("Q575650".to_string()),
        "nix" => Some("Q7041568".to_string()),
        "bash" => Some("Q189248".to_string()),
        "haskell" => Some("Q34010".to_string()),
        "ocaml" => Some("Q178804".to_string()),
        "coq" => Some("Q1136376".to_string()),
        "lean4" => Some("Q28865".to_string()),
        "regex" => Some("Q185612".to_string()),
        "sed" => Some("Q1194858".to_string(),
        "awk" => Some("Q217595".to_string()),
        _ => None,
    }
}

fn get_official_repo(lang: &str) -> Option<String> {
    match lang {
        "rust" => Some("https://github.com/rust-lang/rust".to_string()),
        "nix" => Some("https://github.com/NixOS/nix".to_string()),
        "bash" => Some("https://git.savannah.gnu.org/cgit/bash.git".to_string()),
        "haskell" => Some("https://github.com/ghc/ghc".to_string()),
        "ocaml" => Some("https://github.com/ocaml/ocaml".to_string()),
        "coq" => Some("https://github.com/coq/coq".to_string()),
        "lean4" => Some("https://github.com/leanprover/lean4".to_string()),
        _ => None,
    }
}

impl LanguageTree {
    /// Add decoration to the language tree
    pub fn decorate(&mut self, decorator_type: &str, content: &str) -> &mut Self {
        self.decorations.push(Decoration {
            decorator_type: decorator_type.to_string(),
            content: content.to_string(),
            metadata: HashMap::new(),
        });
        self
    }
    
    /// Generate code in this language
    pub fn generate(&self, template: &str, context: HashMap<String, String>) -> String {
        let mut result = template.to_string();
        
        for (key, value) in context {
            result = result.replace(&format!("{{{}}}", key), &value);
        }
        
        // Add language-specific formatting
        match self.name.as_str() {
            "nix" => format!("# Generated Nix expression\n{{\n{}\n}}", result),
            "rust" => format!("// Generated Rust code\n{}", result),
            "bash" => format!("#!/bin/bash\n# Generated bash script\nset -e\n\n{}", result),
            _ => result,
        }
    }
    
    /// Create interop with another language
    pub fn interop_with(&self, other: &LanguageTree) -> String {
        match (self.name.as_str(), other.name.as_str()) {
            ("nix", "rust") => format!(
                "# Nix calling Rust\nrustPlatform.buildRustPackage {{\n  # Rust project configuration\n}}"
            ),
            ("bash", "nix") => format!(
                "#!/bin/bash\n# Bash calling Nix\nnix-build -E '{}' -o result",
                "with import <nixpkgs> {}; ..."
            ),
            ("rust", "bash") => format!(
                "// Rust calling Bash\nstd::process::Command::new(\"bash\")\n    .arg(\"-c\")\n    .arg(\"{}\")\n    .output()",
                "echo 'Hello from bash'"
            ),
            _ => format!("# Interop between {} and {} not implemented", self.name, other.name),
        }
    }
}

/// Language registry for managing all created languages
pub struct LanguageRegistry {
    pub languages: HashMap<String, LanguageTree>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, tree: LanguageTree) {
        self.languages.insert(tree.name.clone(), tree);
    }
    
    pub fn get(&self, name: &str) -> Option<&LanguageTree> {
        self.languages.get(name)
    }
    
    pub fn create_polyglot_program(&self, languages: Vec<&str>) -> String {
        let mut program = String::new();
        program.push_str("# Polyglot program using multiple languages\n\n");
        
        for lang in languages {
            if let Some(tree) = self.get(lang) {
                program.push_str(&format!("## {} ({})\n", tree.name, tree.properties.paradigm.join(", ")));
                program.push_str(&format!("# Wikidata: {:?}\n", tree.wikidata_entry));
                program.push_str(&format!("# Repo: {:?}\n\n", tree.official_repo));
            }
        }
        
        program
    }
}
