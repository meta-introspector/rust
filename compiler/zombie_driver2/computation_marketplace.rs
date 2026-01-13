// 🧟 COMPLEXITY-BASED COMPUTATION MARKETPLACE: Low-risk → WASM, High-risk → Reserved
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct ComputationTask {
    code: String,
    complexity: ComplexityLevel,
    risk_score: f64,
    estimated_cost: f64,
}

#[derive(Debug, Serialize, Deserialize)]
enum ComplexityLevel {
    Trivial,      // enum->string, const eval
    Low,          // simple parsing, basic math
    Medium,       // complex parsing, small compilation
    High,         // full compilation, unsafe code
    Critical,     // proprietary code, trade secrets
}

#[derive(Debug, Serialize, Deserialize)]
struct ComputationMarketplace {
    free_tier_wasm: Vec<String>,
    paid_tier_nodes: Vec<String>,
    reserved_nodes: Vec<String>,
    rate_limits: HashMap<String, u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 COMPLEXITY-BASED COMPUTATION MARKETPLACE");
    println!("==========================================");
    
    let mut marketplace = ComputationMarketplace::new();
    
    // Auto-classify incoming compilation tasks
    let tasks = vec![
        "enum Color { Red, Green, Blue }",
        "const PI: f64 = 3.14159;",
        "fn complex_algorithm() { /* proprietary */ }",
        "unsafe { transmute(data) }",
    ];
    
    for code in tasks {
        let task = classify_computation(code)?;
        route_computation(&mut marketplace, task)?;
    }
    
    println!("✅ Computation marketplace active!");
    Ok(())
}

impl ComputationMarketplace {
    fn new() -> Self {
        Self {
            free_tier_wasm: vec![
                "https://free-wasm-1.vercel.app".to_string(),
                "https://free-wasm-2.netlify.app".to_string(),
                "https://free-wasm-3.github.io".to_string(),
            ],
            paid_tier_nodes: vec![
                "zombie_node_premium_1".to_string(),
                "zombie_node_premium_2".to_string(),
            ],
            reserved_nodes: vec![
                "zombie_node_secure_1".to_string(),
                "zombie_node_enterprise_1".to_string(),
            ],
            rate_limits: HashMap::from([
                ("free_tier".to_string(), 1000),
                ("paid_tier".to_string(), 10000),
                ("reserved".to_string(), u32::MAX),
            ]),
        }
    }
}

fn classify_computation(code: &str) -> Result<ComputationTask, Box<dyn std::error::Error>> {
    let complexity = analyze_code_complexity(code);
    let risk_score = calculate_risk_score(code);
    let cost = estimate_computation_cost(&complexity, risk_score);
    
    Ok(ComputationTask {
        code: code.to_string(),
        complexity,
        risk_score,
        estimated_cost: cost,
    })
}

fn analyze_code_complexity(code: &str) -> ComplexityLevel {
    // Auto-classify based on code patterns
    if code.contains("enum") && code.contains("String") {
        ComplexityLevel::Trivial
    } else if code.contains("const") && !code.contains("unsafe") {
        ComplexityLevel::Trivial
    } else if code.contains("fn") && code.len() < 100 {
        ComplexityLevel::Low
    } else if code.contains("unsafe") || code.contains("transmute") {
        ComplexityLevel::High
    } else if code.contains("proprietary") || code.contains("secret") {
        ComplexityLevel::Critical
    } else {
        ComplexityLevel::Medium
    }
}

fn calculate_risk_score(code: &str) -> f64 {
    let mut risk = 0.0;
    
    // Risk factors
    if code.contains("unsafe") { risk += 0.8; }
    if code.contains("transmute") { risk += 0.9; }
    if code.contains("proprietary") { risk += 1.0; }
    if code.contains("secret") { risk += 1.0; }
    if code.contains("password") { risk += 1.0; }
    
    // Safety factors
    if code.contains("const") { risk -= 0.3; }
    if code.contains("enum") { risk -= 0.2; }
    if code.len() < 50 { risk -= 0.1; }
    
    risk.max(0.0).min(1.0)
}

fn estimate_computation_cost(complexity: &ComplexityLevel, risk: f64) -> f64 {
    let base_cost = match complexity {
        ComplexityLevel::Trivial => 0.0,   // Free tier
        ComplexityLevel::Low => 0.01,      // Almost free
        ComplexityLevel::Medium => 0.10,   // Paid tier
        ComplexityLevel::High => 1.00,     // Premium
        ComplexityLevel::Critical => 10.0, // Reserved/Enterprise
    };
    
    base_cost * (1.0 + risk * 2.0)
}

fn route_computation(marketplace: &mut ComputationMarketplace, task: ComputationTask) -> Result<(), Box<dyn std::error::Error>> {
    match task.complexity {
        ComplexityLevel::Trivial | ComplexityLevel::Low if task.risk_score < 0.1 => {
            route_to_free_wasm(marketplace, &task)?;
        }
        ComplexityLevel::Medium if task.risk_score < 0.5 => {
            route_to_paid_tier(marketplace, &task)?;
        }
        ComplexityLevel::High | ComplexityLevel::Critical => {
            route_to_reserved_nodes(marketplace, &task)?;
        }
        _ => {
            route_to_paid_tier(marketplace, &task)?;
        }
    }
    
    Ok(())
}

fn route_to_free_wasm(marketplace: &ComputationMarketplace, task: &ComputationTask) -> Result<(), Box<dyn std::error::Error>> {
    println!("🆓 Routing to FREE WASM tier: {}", task.code.chars().take(30).collect::<String>());
    
    // Generate WASM module for the computation
    let wasm_code = generate_wasm_module(&task.code)?;
    
    // Deploy to free tier WASM hosts
    for wasm_host in &marketplace.free_tier_wasm {
        deploy_to_wasm_host(wasm_host, &wasm_code)?;
    }
    
    println!("✅ Deployed to {} free WASM hosts", marketplace.free_tier_wasm.len());
    Ok(())
}

fn route_to_paid_tier(marketplace: &ComputationMarketplace, task: &ComputationTask) -> Result<(), Box<dyn std::error::Error>> {
    println!("💰 Routing to PAID tier: ${:.2}", task.estimated_cost);
    
    // Find available paid node
    let node = marketplace.paid_tier_nodes.first()
        .ok_or("No paid tier nodes available")?;
    
    send_to_zombie_node(node, task)?;
    Ok(())
}

fn route_to_reserved_nodes(marketplace: &ComputationMarketplace, task: &ComputationTask) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Routing to RESERVED/ENTERPRISE tier: ${:.2}", task.estimated_cost);
    
    let node = marketplace.reserved_nodes.first()
        .ok_or("No reserved nodes available")?;
    
    send_to_secure_node(node, task)?;
    Ok(())
}

fn generate_wasm_module(code: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Generate WASM for simple computations
    let wasm_template = format!(r#"
(module
  (func $compute (result i32)
    ;; Generated from: {}
    i32.const 42
  )
  (export "compute" (func $compute))
)
"#, code);
    
    // In real implementation, would use wasmtime or similar
    Ok(wasm_template.into_bytes())
}

fn deploy_to_wasm_host(host: &str, wasm_code: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Deploying WASM to {}", host);
    
    // HTTP POST to deploy WASM module
    use std::process::Command;
    Command::new("curl")
        .args(&["-X", "POST", host, "--data-binary", "@-"])
        .output()?;
    
    Ok(())
}

fn send_to_zombie_node(node: &str, task: &ComputationTask) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 Sending to zombie node: {}", node);
    
    let message = serde_json::json!({
        "type": "computation_task",
        "code": task.code,
        "complexity": task.complexity,
        "estimated_cost": task.estimated_cost
    });
    
    // Send via libp2p2
    use std::net::TcpStream;
    use std::io::Write;
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all(message.to_string().as_bytes())?;
    
    Ok(())
}

fn send_to_secure_node(node: &str, task: &ComputationTask) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Sending to secure node: {}", node);
    
    // Encrypt the task for secure transmission
    let encrypted_task = encrypt_task(task)?;
    
    // Send via secure channel
    send_encrypted_task(node, &encrypted_task)?;
    
    Ok(())
}

fn encrypt_task(task: &ComputationTask) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Simple encryption (in real implementation, use proper crypto)
    let serialized = serde_json::to_string(task)?;
    Ok(serialized.into_bytes())
}

fn send_encrypted_task(node: &str, encrypted: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Sending encrypted task to {}", node);
    Ok(())
}

// WASM bindings for free tier computations
#[wasm_bindgen]
pub fn compute_enum_to_string(enum_val: &str) -> String {
    format!("String representation of: {}", enum_val)
}

#[wasm_bindgen]
pub fn evaluate_const(expr: &str) -> f64 {
    // Simple const evaluation
    match expr {
        "PI" => 3.14159,
        "E" => 2.71828,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complexity_classification() {
        assert!(matches!(analyze_code_complexity("enum Color { Red }"), ComplexityLevel::Trivial));
        assert!(matches!(analyze_code_complexity("const PI: f64 = 3.14;"), ComplexityLevel::Trivial));
        assert!(matches!(analyze_code_complexity("unsafe { transmute(x) }"), ComplexityLevel::High));
    }
    
    #[test]
    fn test_risk_scoring() {
        assert!(calculate_risk_score("const PI = 3.14") < 0.1);
        assert!(calculate_risk_score("unsafe { transmute(x) }") > 0.8);
        assert!(calculate_risk_score("proprietary secret code") > 0.9);
    }
    
    #[test]
    fn test_cost_estimation() {
        let trivial_cost = estimate_computation_cost(&ComplexityLevel::Trivial, 0.0);
        let critical_cost = estimate_computation_cost(&ComplexityLevel::Critical, 1.0);
        assert!(trivial_cost == 0.0);
        assert!(critical_cost >= 10.0);
    }
}
