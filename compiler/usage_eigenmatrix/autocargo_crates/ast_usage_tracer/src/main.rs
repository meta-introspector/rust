use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use syn::{self, visit::Visit, Item, Expr};
use serde_json;

struct AstVisitor {
    pattern_counts: HashMap<String, u64>,
    context_stack: Vec<String>,
}

impl AstVisitor {
    fn new() -> Self {
        Self {
            pattern_counts: HashMap::new(),
            context_stack: Vec::new(),
        }
    }

    fn sample_stack(&mut self) {
        for depth in 1..=8 {
            if self.context_stack.len() >= depth {
                let pattern = self.context_stack[self.context_stack.len()-depth..].join("::");
                *self.pattern_counts.entry(pattern).or_insert(0) += 1;
            }
        }
    }

    fn push_symbol(&mut self, symbol: String) {
        self.context_stack.push(symbol);
        self.sample_stack();
    }

    fn pop_symbol(&mut self) {
        self.context_stack.pop();
    }
}

impl<'ast> Visit<'ast> for AstVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Fn(f) => {
                self.push_symbol(f.sig.ident.to_string());
                syn::visit::visit_item_fn(self, f);
                self.pop_symbol();
            }
            Item::Struct(s) => {
                self.push_symbol(s.ident.to_string());
                syn::visit::visit_item_struct(self, s);
                self.pop_symbol();
            }
            Item::Enum(e) => {
                self.push_symbol(e.ident.to_string());
                syn::visit::visit_item_enum(self, e);
                self.pop_symbol();
            }
            _ => syn::visit::visit_item(self, item),
        }
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Path(p) => {
                if let Some(ident) = p.path.get_ident() {
                    self.push_symbol(ident.to_string());
                    self.pop_symbol();
                }
            }
            Expr::Call(c) => {
                self.push_symbol("call".to_string());
                syn::visit::visit_expr_call(self, c);
                self.pop_symbol();
            }
            Expr::MethodCall(m) => {
                self.push_symbol(m.method.to_string());
                syn::visit::visit_expr_method_call(self, m);
                self.pop_symbol();
            }
            _ => syn::visit::visit_expr(self, expr),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Single-Pass AST Pattern Sampling");
    
    let mut visitor = AstVisitor::new();
    let mut file_count = 0;
    
    for entry in WalkDir::new("submodules/rust")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .take(100)
    {
        let content = fs::read_to_string(entry.path())?;
        if let Ok(ast) = syn::parse_file(&content) {
            visitor.visit_file(&ast);
            file_count += 1;
            
            if file_count % 10 == 0 {
                println!("  📊 Processed {} files, {} patterns found", 
                    file_count, visitor.pattern_counts.len());
            }
        }
    }
    
    println!("✅ Analyzed {} files", file_count);
    println!("📈 Found {} unique patterns", visitor.pattern_counts.len());
    
    // Cache results
    let json_data = serde_json::to_string_pretty(&visitor.pattern_counts)?;
    fs::write("ast_patterns.json", json_data)?;
    println!("💾 Cached patterns to ast_patterns.json");
    
    // Generate reports
    generate_reports(&visitor.pattern_counts)?;
    
    Ok(())
}

fn generate_reports(patterns: &HashMap<String, u64>) -> Result<(), Box<dyn std::error::Error>> {
    let mut sorted: Vec<_> = patterns.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    // Report 1: Top 20 overall
    println!("\n🔥 Top 20 AST Usage Patterns:");
    for (pattern, count) in sorted.iter().take(20) {
        println!("  {} → {}", pattern, count);
    }
    
    // Report 2: Top 10 length-8 expressions
    let length8: Vec<_> = sorted.iter()
        .filter(|(pattern, _)| pattern.split("::").count() == 8)
        .take(10)
        .collect();
    
    println!("\n🎯 Top 10 Length-8 AST Expressions:");
    for (pattern, count) in length8 {
        println!("  {} → {}", pattern, count);
    }
    
    // Report 3: Pattern length distribution
    let mut length_dist: HashMap<usize, u64> = HashMap::new();
    for (pattern, count) in patterns {
        let len = pattern.split("::").count();
        *length_dist.entry(len).or_insert(0) += count;
    }
    
    println!("\n📊 Pattern Length Distribution:");
    let mut dist: Vec<_> = length_dist.iter().collect();
    dist.sort_by_key(|&(len, _)| len);
    for (len, total) in dist {
        println!("  Length {} → {} occurrences", len, total);
    }
    
    // Report 4: Most common symbols by depth
    for depth in 1..=8 {
        let depth_patterns: Vec<_> = sorted.iter()
            .filter(|(pattern, _)| pattern.split("::").count() == depth)
            .take(5)
            .collect();
        
        if !depth_patterns.is_empty() {
            println!("\n🔍 Top 5 Depth-{} Patterns:", depth);
            for (pattern, count) in depth_patterns {
                println!("  {} → {}", pattern, count);
            }
        }
    }
    
    Ok(())
}
