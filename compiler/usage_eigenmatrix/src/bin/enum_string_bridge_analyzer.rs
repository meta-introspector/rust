use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Enum-to-String Bridge Analyzer
/// Identifies functions F: Enum → String (Label Set 1)

#[derive(Debug, Clone)]
struct EnumToStringFunction {
    function_name: String,
    enum_type: String,
    string_patterns: Vec<String>,
    file_path: String,
    line_number: usize,
    monster_cell: u32,
}

#[derive(Debug)]
struct EnumStringBridgeAnalyzer {
    label_set_1: Vec<EnumToStringFunction>,
    enum_types: HashSet<String>,
    string_functions: HashMap<String, Vec<String>>,
    prime_generators: [u8; 8],
}

impl EnumStringBridgeAnalyzer {
    fn new() -> Self {
        Self {
            label_set_1: Vec::new(),
            enum_types: HashSet::new(),
            string_functions: HashMap::new(),
            prime_generators: [2, 3, 5, 7, 11, 13, 17, 19],
        }
    }
    
    fn hash_to_monster_cell(&self, data: &str) -> u32 {
        let mut hash = 1u64;
        for (i, byte) in data.bytes().enumerate() {
            let prime_idx = i % 8;
            hash = hash.wrapping_mul(self.prime_generators[prime_idx] as u64)
                      .wrapping_add(byte as u64);
        }
        (hash % (1u64 << 24)) as u32
    }
    
    fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        // Find enum definitions
        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().starts_with("enum ") {
                if let Some(enum_name) = self.extract_enum_name(line) {
                    self.enum_types.insert(enum_name);
                }
            }
        }
        
        // Find functions that map enums to strings
        for (line_num, line) in lines.iter().enumerate() {
            if self.is_enum_to_string_function(line, &lines, line_num) {
                if let Some(function) = self.parse_enum_string_function(line, &lines, line_num, file_path) {
                    self.label_set_1.push(function);
                }
            }
        }
        
        Ok(())
    }
    
    fn extract_enum_name(&self, line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "enum" {
            Some(parts[1].trim_end_matches('{').to_string())
        } else {
            None
        }
    }
    
    fn is_enum_to_string_function(&self, line: &str, lines: &[&str], line_num: usize) -> bool {
        // Look for function signatures that return String/&str and take enum parameters
        if line.contains("fn ") && (line.contains("-> String") || line.contains("-> &str")) {
            // Check if function body contains enum matching
            for i in line_num..std::cmp::min(line_num + 20, lines.len()) {
                if lines[i].contains("match ") || lines[i].contains("=> \"") {
                    return true;
                }
            }
        }
        false
    }
    
    fn parse_enum_string_function(&self, line: &str, lines: &[&str], line_num: usize, file_path: &Path) -> Option<EnumToStringFunction> {
        // Extract function name
        let function_name = self.extract_function_name(line)?;
        
        // Extract enum type from parameters
        let enum_type = self.extract_enum_parameter(line)?;
        
        // Extract string patterns from match arms
        let string_patterns = self.extract_string_patterns(lines, line_num);
        
        let monster_cell = self.hash_to_monster_cell(&format!("{}::{}", enum_type, function_name));
        
        Some(EnumToStringFunction {
            function_name,
            enum_type,
            string_patterns,
            file_path: file_path.to_string_lossy().to_string(),
            line_number: line_num + 1,
            monster_cell,
        })
    }
    
    fn extract_function_name(&self, line: &str) -> Option<String> {
        if let Some(fn_pos) = line.find("fn ") {
            let after_fn = &line[fn_pos + 3..];
            if let Some(paren_pos) = after_fn.find('(') {
                return Some(after_fn[..paren_pos].trim().to_string());
            }
        }
        None
    }
    
    fn extract_enum_parameter(&self, line: &str) -> Option<String> {
        // Look for enum types in function parameters
        for enum_type in &self.enum_types {
            if line.contains(enum_type) {
                return Some(enum_type.clone());
            }
        }
        
        // Common enum patterns
        if line.contains("self") && line.contains("&self") {
            return Some("Self".to_string());
        }
        
        None
    }
    
    fn extract_string_patterns(&self, lines: &[&str], start_line: usize) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for i in start_line..std::cmp::min(start_line + 30, lines.len()) {
            let line = lines[i];
            
            // Look for string literals in match arms
            if line.contains("=> \"") {
                if let Some(start) = line.find("=> \"") {
                    let after_arrow = &line[start + 4..];
                    if let Some(end) = after_arrow.find('"') {
                        patterns.push(after_arrow[..end].to_string());
                    }
                }
            }
            
            // Look for format! macros
            if line.contains("format!") {
                patterns.push("format_macro".to_string());
            }
            
            // Stop at function end
            if line.trim() == "}" && line.len() == 1 {
                break;
            }
        }
        
        patterns
    }
    
    fn analyze_directory(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let dir_name = path.file_name().unwrap().to_string_lossy();
                if !dir_name.starts_with('.') && dir_name != "target" {
                    self.analyze_directory(&path)?;
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Err(e) = self.analyze_file(&path) {
                    eprintln!("⚠️  Error analyzing {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(())
    }
    
    fn generate_label_set_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Enum-to-String Bridge Analysis: Label Set 1\n\n");
        report.push_str("## Functions F: Enum → String Domain Mapping\n\n");
        
        report.push_str(&format!("### Summary\n"));
        report.push_str(&format!("- **Total Functions**: {}\n", self.label_set_1.len()));
        report.push_str(&format!("- **Enum Types**: {}\n", self.enum_types.len()));
        report.push_str(&format!("- **Unique Monster Cells**: {}\n", 
            self.label_set_1.iter().map(|f| f.monster_cell).collect::<HashSet<_>>().len()));
        
        report.push_str("\n### Label Set 1: Enum → String Functions\n");
        report.push_str("| Function | Enum Type | Monster Cell | String Patterns | File |\n");
        report.push_str("|----------|-----------|--------------|-----------------|------|\n");
        
        for func in &self.label_set_1 {
            let patterns = func.string_patterns.join(", ");
            let patterns_display = if patterns.len() > 50 {
                format!("{}...", &patterns[..47])
            } else {
                patterns
            };
            
            report.push_str(&format!(
                "| `{}` | `{}` | `0x{:06X}` | {} | {} |\n",
                func.function_name,
                func.enum_type,
                func.monster_cell,
                patterns_display,
                Path::new(&func.file_path).file_name().unwrap().to_string_lossy()
            ));
        }
        
        report.push_str("\n### Enum Types Discovered\n");
        for enum_type in &self.enum_types {
            report.push_str(&format!("- `{}`\n", enum_type));
        }
        
        report.push_str("\n### Monster Group Mapping\n");
        report.push_str("Each enum-to-string function is mapped to a unique 24-bit Monster cell:\n\n");
        
        let mut cell_distribution = HashMap::new();
        for func in &self.label_set_1 {
            let prime_idx = (func.monster_cell % 8) as usize;
            let prime = self.prime_generators[prime_idx];
            *cell_distribution.entry(prime).or_insert(0) += 1;
        }
        
        for (prime, count) in cell_distribution {
            report.push_str(&format!("- **Prime {}**: {} functions\n", prime, count));
        }
        
        // Calculate label set signature
        let mut signature = 1u128;
        for func in &self.label_set_1 {
            signature = signature.wrapping_mul(func.monster_cell as u128);
        }
        
        report.push_str(&format!("\n### Label Set 1 Signature\n"));
        report.push_str(&format!("**0x{:032X}**\n", signature));
        
        report.push_str("\n### Mathematical Foundation\n");
        report.push_str("Label Set 1 establishes the foundational mapping:\n");
        report.push_str("- **Domain**: Enum values (discrete algebraic types)\n");
        report.push_str("- **Range**: String values (textual representations)\n");
        report.push_str("- **Functions**: F: Enum → String (bijective where possible)\n");
        report.push_str("- **Monster Cells**: Each function mapped to 24-bit prime space\n\n");
        report.push_str("This creates the basis for all subsequent label set analysis.\n");
        
        report
    }
}

fn main() {
    println!("🍄 Enum-to-String Bridge Analyzer");
    println!("=================================");
    println!("Identifying Label Set 1: Functions F: Enum → String");
    
    let mut analyzer = EnumStringBridgeAnalyzer::new();
    
    // Analyze current directory and rustc_driver
    let paths = [".", "../rustc_driver"];
    
    for path in &paths {
        println!("🔍 Analyzing: {}", path);
        
        match analyzer.analyze_directory(Path::new(path)) {
            Ok(()) => println!("✅ Analysis complete for {}", path),
            Err(e) => eprintln!("❌ Error analyzing {}: {}", path, e),
        }
    }
    
    // Generate report
    let report = analyzer.generate_label_set_report();
    
    match fs::write("enum_string_bridge_analysis.md", &report) {
        Ok(()) => println!("📊 Report saved: enum_string_bridge_analysis.md"),
        Err(e) => eprintln!("❌ Error saving report: {}", e),
    }
    
    println!("\n🎯 LABEL SET 1 ANALYSIS COMPLETE!");
    println!("=================================");
    println!("Functions found: {}", analyzer.label_set_1.len());
    println!("Enum types: {}", analyzer.enum_types.len());
    println!("Monster cells: {}", 
        analyzer.label_set_1.iter().map(|f| f.monster_cell).collect::<HashSet<_>>().len());
    
    println!("\n🧬 Label Set 1 established: F: Enum → String domain mapping complete!");
}
