use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanonicalDecl {
    pub hash: String,           // Content hash of canonical form
    pub minimal_form: String,   // Canonical minimal representation
    pub decl_type: String,      // "enum", "struct", "fn", etc.
    pub name: String,           // Declaration name
    pub variants: Vec<String>,  // For enums: variant names
    pub fields: Vec<String>,    // For structs: field names  
    pub signature: String,      // For functions: minimal signature
}

pub struct DeclCanonicalizer {
    cache: HashMap<String, CanonicalDecl>,
}

impl DeclCanonicalizer {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    pub fn canonicalize_enum(&mut self, name: &str, variants: &[&str]) -> CanonicalDecl {
        // Create canonical minimal form
        let mut sorted_variants: Vec<_> = variants.iter().map(|s| s.to_string()).collect();
        sorted_variants.sort();
        
        let minimal_form = format!("enum {}{{{}}}", name, sorted_variants.join(","));
        let hash = self.content_hash(&minimal_form);
        
        let canonical = CanonicalDecl {
            hash: hash.clone(),
            minimal_form,
            decl_type: "enum".to_string(),
            name: name.to_string(),
            variants: sorted_variants,
            fields: vec![],
            signature: String::new(),
        };
        
        self.cache.insert(hash, canonical.clone());
        canonical
    }
    
    pub fn canonicalize_struct(&mut self, name: &str, fields: &[&str]) -> CanonicalDecl {
        let mut sorted_fields: Vec<_> = fields.iter().map(|s| s.to_string()).collect();
        sorted_fields.sort();
        
        let minimal_form = format!("struct {}{{{}}}",  name, sorted_fields.join(","));
        let hash = self.content_hash(&minimal_form);
        
        let canonical = CanonicalDecl {
            hash: hash.clone(),
            minimal_form,
            decl_type: "struct".to_string(),
            name: name.to_string(),
            variants: vec![],
            fields: sorted_fields,
            signature: String::new(),
        };
        
        self.cache.insert(hash, canonical.clone());
        canonical
    }
    
    pub fn canonicalize_function(&mut self, name: &str, params: &[&str], return_type: &str) -> CanonicalDecl {
        let signature = format!("fn {}({}) -> {}", name, params.join(","), return_type);
        let minimal_form = self.minimize_signature(&signature);
        let hash = self.content_hash(&minimal_form);
        
        let canonical = CanonicalDecl {
            hash: hash.clone(),
            minimal_form,
            decl_type: "fn".to_string(),
            name: name.to_string(),
            variants: vec![],
            fields: vec![],
            signature,
        };
        
        self.cache.insert(hash, canonical.clone());
        canonical
    }
    
    fn content_hash(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string() // First 16 chars
    }
    
    fn minimize_signature(&self, sig: &str) -> String {
        // Remove whitespace, normalize types
        sig.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .replace("&str", "s")
            .replace("String", "S")
            .replace("usize", "u")
            .replace("i32", "i")
    }
    
    pub fn get_by_hash(&self, hash: &str) -> Option<&CanonicalDecl> {
        self.cache.get(hash)
    }
    
    pub fn save_cache(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.cache)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load_cache(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if std::path::Path::new(path).exists() {
            let json = std::fs::read_to_string(path)?;
            self.cache = serde_json::from_str(&json)?;
        }
        Ok(())
    }
}

// Integration with mklang! macro
#[macro_export]
macro_rules! canonical_hash {
    (enum $name:ident { $($variant:ident),* }) => {
        {
            let mut canonicalizer = DeclCanonicalizer::new();
            let variants = vec![$(stringify!($variant)),*];
            canonicalizer.canonicalize_enum(stringify!($name), &variants).hash
        }
    };
    
    (struct $name:ident { $($field:ident),* }) => {
        {
            let mut canonicalizer = DeclCanonicalizer::new();
            let fields = vec![$(stringify!($field)),*];
            canonicalizer.canonicalize_struct(stringify!($name), &fields).hash
        }
    };
}
