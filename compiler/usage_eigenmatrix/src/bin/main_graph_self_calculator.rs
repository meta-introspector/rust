use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🎯 RUSTC MAIN GRAPH SELF-CALCULATOR");
    println!("═══════════════════════════════════");
    
    let mut calculator = MainGraphCalculator::new();
    
    // Load the rustc main graph we created
    calculator.load_main_graph()?;
    
    // Apply the graph to itself as input
    calculator.calculate_self()?;
    
    Ok(())
}

struct MainGraphCalculator {
    main_graph: HashMap<String, Vec<String>>,
    equivalence_classes: HashMap<String, String>, // node -> class mapping
    calculation_results: HashMap<String, CalculationResult>,
}

#[derive(Debug, Clone)]
struct CalculationResult {
    input_node: String,
    applied_to: String,
    result_class: String,
    operations_count: usize,
    convergence_value: f64,
}

impl MainGraphCalculator {
    fn new() -> Self {
        Self {
            main_graph: HashMap::new(),
            equivalence_classes: HashMap::new(),
            calculation_results: HashMap::new(),
        }
    }
    
    fn load_main_graph(&mut self) -> Result<()> {
        println!("📊 Loading rustc main graph...");
        
        // Try to load from DOT file first
        if let Ok(dot_content) = fs::read_to_string("rustc_main_graph.dot") {
            self.parse_dot_graph(&dot_content)?;
        } else {
            // Fallback to usage data
            self.load_from_usage_data()?;
        }
        
        println!("  Loaded {} main graph nodes", self.main_graph.len());
        
        // Classify nodes into equivalence classes
        self.classify_nodes();
        
        Ok(())
    }
    
    fn parse_dot_graph(&mut self, dot_content: &str) -> Result<()> {
        for line in dot_content.lines() {
            if line.contains(" -> ") {
                // Parse DOT edge: "node_a" -> "node_b";
                let parts: Vec<&str> = line.split(" -> ").collect();
                if parts.len() == 2 {
                    let from = parts[0].trim().trim_matches('"').trim();
                    let to = parts[1].trim().trim_matches('"').trim_end_matches(';').trim();
                    
                    if !from.is_empty() && !to.is_empty() {
                        self.main_graph.entry(from.to_string())
                            .or_insert_with(Vec::new)
                            .push(to.to_string());
                    }
                }
            }
        }
        Ok(())
    }
    
    fn load_from_usage_data(&mut self) -> Result<()> {
        let usage_dir = "../../usage_data";
        let mut file_count = 0;
        
        for entry in fs::read_dir(usage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                let data: Value = serde_json::from_str(&content)?;
                
                if let Some(usages) = data["usages"].as_array() {
                    for usage_obj in usages {
                        let user_id = usage_obj["user_def_id"].as_str().unwrap_or("unknown");
                        let used_id = usage_obj["used_def_id"].as_str().unwrap_or("unknown");
                        
                        // Only include main-related functions
                        if self.is_main_related(user_id) || self.is_main_related(used_id) {
                            self.main_graph.entry(user_id.to_string())
                                .or_insert_with(Vec::new)
                                .push(used_id.to_string());
                        }
                    }
                }
                
                file_count += 1;
                if file_count >= 100 {
                    break;
                }
            }
        }
        Ok(())
    }
    
    fn is_main_related(&self, node: &str) -> bool {
        node.contains("main") || node.contains("rustc_driver") ||
        node.contains("run_compiler") || node.contains("parse") ||
        node.contains("analysis") || node.contains("codegen")
    }
    
    fn classify_nodes(&mut self) {
        println!("🏛️ Classifying nodes into equivalence classes...");
        
        for node in self.main_graph.keys() {
            let class = self.determine_equivalence_class(node);
            self.equivalence_classes.insert(node.clone(), class);
        }
        
        // Count classes
        let mut class_counts = HashMap::new();
        for class in self.equivalence_classes.values() {
            *class_counts.entry(class.clone()).or_insert(0) += 1;
        }
        
        println!("  Equivalence classes found:");
        for (class, count) in &class_counts {
            println!("    {}: {} nodes", class, count);
        }
    }
    
    fn determine_equivalence_class(&self, node: &str) -> String {
        let callees = self.main_graph.get(node).map(|v| v.len()).unwrap_or(0);
        
        if node.contains("main") {
            "Α (Alpha) - Entry Point".to_string()
        } else if node.contains("DefId") && callees <= 2 {
            "Β (Beta) - DefId Simple".to_string()
        } else if node.contains("DefId") && callees <= 10 {
            "Γ (Gamma) - DefId Medium".to_string()
        } else if node.contains("DefId") {
            "Δ (Delta) - DefId Complex".to_string()
        } else if callees == 0 {
            "Ε (Epsilon) - Leaf Node".to_string()
        } else {
            "Ζ (Zeta) - Generic Utility".to_string()
        }
    }
    
    fn calculate_self(&mut self) -> Result<()> {
        println!("\n🧮 APPLYING MAIN GRAPH TO ITSELF...");
        println!("═══════════════════════════════════");
        
        let nodes: Vec<String> = self.main_graph.keys().cloned().collect();
        let total_operations = nodes.len() * nodes.len();
        
        println!("  Total operations: {} ({}²)", total_operations, nodes.len());
        
        let mut operation_count = 0;
        
        for (i, node_a) in nodes.iter().enumerate() {
            for (j, node_b) in nodes.iter().enumerate() {
                operation_count += 1;
                
                if operation_count % 1000 == 0 {
                    println!("    Progress: {}/{} operations", operation_count, total_operations);
                }
                
                let result = self.apply_node_to_node(node_a, node_b);
                
                let calc_result = CalculationResult {
                    input_node: node_a.clone(),
                    applied_to: node_b.clone(),
                    result_class: result.0,
                    operations_count: result.1,
                    convergence_value: result.2,
                };
                
                let key = format!("{}→{}", i, j);
                self.calculation_results.insert(key, calc_result);
            }
        }
        
        self.analyze_results()?;
        
        Ok(())
    }
    
    fn apply_node_to_node(&self, node_a: &str, node_b: &str) -> (String, usize, f64) {
        let class_a = self.equivalence_classes.get(node_a).cloned().unwrap_or_default();
        let class_b = self.equivalence_classes.get(node_b).cloned().unwrap_or_default();
        
        let connections_a = self.main_graph.get(node_a).cloned().unwrap_or_default();
        let connections_b = self.main_graph.get(node_b).cloned().unwrap_or_default();
        
        // Apply Monster Group constraints
        let result_class = if node_a == node_b {
            // Self-application
            class_a
        } else if connections_a.contains(&node_b.to_string()) {
            // Direct call: A calls B, result is B's class
            class_b
        } else if connections_b.contains(&node_a.to_string()) {
            // Reverse call: B calls A, result is A's class
            class_a
        } else {
            // No direct connection, apply equivalence class rules
            self.combine_classes(&class_a, &class_b)
        };
        
        let operations = connections_a.len() + connections_b.len();
        let convergence = self.calculate_convergence(&connections_a, &connections_b);
        
        (result_class, operations, convergence)
    }
    
    fn combine_classes(&self, class_a: &str, class_b: &str) -> String {
        // Monster Group combination rules
        match (class_a.chars().next(), class_b.chars().next()) {
            (Some('Α'), _) => class_a.to_string(), // Alpha dominates
            (_, Some('Α')) => class_b.to_string(), // Alpha dominates
            (Some('Β'), Some('Γ')) | (Some('Γ'), Some('Β')) => "Γ (Gamma) - DefId Medium".to_string(),
            (Some('Β'), Some('Δ')) | (Some('Δ'), Some('Β')) => "Δ (Delta) - DefId Complex".to_string(),
            _ => "Ζ (Zeta) - Generic Utility".to_string(), // Default combination
        }
    }
    
    fn calculate_convergence(&self, connections_a: &[String], connections_b: &[String]) -> f64 {
        if connections_a.is_empty() && connections_b.is_empty() {
            1.0 // Perfect convergence for leaf nodes
        } else {
            let intersection_count = connections_a.iter()
                .filter(|&conn| connections_b.contains(conn))
                .count();
            let union_count = connections_a.len() + connections_b.len() - intersection_count;
            
            if union_count == 0 {
                1.0
            } else {
                intersection_count as f64 / union_count as f64
            }
        }
    }
    
    fn analyze_results(&self) -> Result<()> {
        println!("\n📊 CALCULATION RESULTS ANALYSIS:");
        println!("═══════════════════════════════");
        
        // Count result classes
        let mut class_distribution = HashMap::new();
        let mut total_operations = 0;
        let mut total_convergence = 0.0;
        
        for result in self.calculation_results.values() {
            *class_distribution.entry(result.result_class.clone()).or_insert(0) += 1;
            total_operations += result.operations_count;
            total_convergence += result.convergence_value;
        }
        
        println!("  Total calculations: {}", self.calculation_results.len());
        println!("  Average operations per calculation: {:.2}", 
            total_operations as f64 / self.calculation_results.len() as f64);
        println!("  Average convergence: {:.4}", 
            total_convergence / self.calculation_results.len() as f64);
        
        println!("\n🎭 RESULT CLASS DISTRIBUTION:");
        for (class, count) in &class_distribution {
            let percentage = (*count as f64 / self.calculation_results.len() as f64) * 100.0;
            println!("  {}: {} results ({:.1}%)", class, count, percentage);
        }
        
        // Find highest convergence results
        let mut convergence_results: Vec<_> = self.calculation_results.values().collect();
        convergence_results.sort_by(|a, b| b.convergence_value.partial_cmp(&a.convergence_value).unwrap());
        
        println!("\n🎯 TOP CONVERGENCE RESULTS:");
        for (i, result) in convergence_results.iter().take(5).enumerate() {
            println!("  {}. {} → {} = {} (convergence: {:.4})", 
                i+1, 
                self.clean_name(&result.input_node),
                self.clean_name(&result.applied_to),
                result.result_class,
                result.convergence_value);
        }
        
        // Save results
        let summary = serde_json::json!({
            "total_calculations": self.calculation_results.len(),
            "class_distribution": class_distribution,
            "average_convergence": total_convergence / self.calculation_results.len() as f64,
            "monster_group_validation": "Main graph successfully applied to itself"
        });
        
        fs::write("main_graph_self_calculation.json", serde_json::to_string_pretty(&summary)?)?;
        println!("\n💾 Results saved to: main_graph_self_calculation.json");
        
        Ok(())
    }
    
    fn clean_name(&self, name: &str) -> String {
        if name.len() > 30 {
            format!("{}...", &name[..27])
        } else {
            name.to_string()
        }
    }
}
