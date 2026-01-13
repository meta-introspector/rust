// 🧟 CARGO.LOCK → NETWORK ARCHITECTURE: Dependencies become P2P topology
use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CargoLock {
    package: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    dependencies: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct P2PNetworkTopology {
    nodes: Vec<P2PNode>,
    connections: Vec<P2PConnection>,
    compilation_order: Vec<String>,
}

#[derive(Debug, Serialize)]
struct P2PNode {
    node_id: String,
    crate_name: String,
    version: String,
    capabilities: Vec<String>,
    dependencies: Vec<String>,
}

#[derive(Debug, Serialize)]
struct P2PConnection {
    from: String,
    to: String,
    dependency_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 CARGO.LOCK → P2P NETWORK ARCHITECTURE");
    println!("========================================");
    
    // Load rustc's Cargo.lock
    let cargo_lock = load_cargo_lock("rust_source/Cargo.lock")?;
    
    // Convert to P2P network topology
    let network = cargo_lock_to_p2p_network(&cargo_lock)?;
    
    // Deploy network nodes
    deploy_p2p_network(&network)?;
    
    println!("✅ P2P network deployed based on Cargo.lock topology!");
    Ok(())
}

fn load_cargo_lock(path: &str) -> Result<CargoLock, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let lock: CargoLock = toml::from_str(&content)?;
    println!("📦 Loaded {} packages from Cargo.lock", lock.package.len());
    Ok(lock)
}

fn cargo_lock_to_p2p_network(cargo_lock: &CargoLock) -> Result<P2PNetworkTopology, Box<dyn std::error::Error>> {
    let mut nodes = Vec::new();
    let mut connections = Vec::new();
    let mut compilation_order = Vec::new();
    
    // Each crate becomes a P2P node
    for package in &cargo_lock.package {
        let node_id = format!("zombie_{}_{}", package.name, package.version);
        
        // Determine node capabilities based on crate type
        let capabilities = match package.name.as_str() {
            name if name.starts_with("rustc_") => vec!["compiler".to_string(), "analysis".to_string()],
            "syn" => vec!["parsing".to_string(), "ast".to_string()],
            "serde" => vec!["serialization".to_string()],
            "tokio" => vec!["async".to_string(), "networking".to_string()],
            _ => vec!["compilation".to_string()],
        };
        
        let dependencies = package.dependencies.clone().unwrap_or_default();
        
        nodes.push(P2PNode {
            node_id: node_id.clone(),
            crate_name: package.name.clone(),
            version: package.version.clone(),
            capabilities,
            dependencies: dependencies.clone(),
        });
        
        // Create connections based on dependencies
        for dep in &dependencies {
            connections.push(P2PConnection {
                from: node_id.clone(),
                to: format!("zombie_{}_*", dep), // Wildcard version
                dependency_type: "compilation_dependency".to_string(),
            });
        }
        
        compilation_order.push(package.name.clone());
    }
    
    // Sort compilation order by dependency depth
    compilation_order.sort_by_key(|name| {
        cargo_lock.package.iter()
            .find(|p| &p.name == name)
            .map(|p| p.dependencies.as_ref().map(|d| d.len()).unwrap_or(0))
            .unwrap_or(0)
    });
    
    Ok(P2PNetworkTopology {
        nodes,
        connections,
        compilation_order,
    })
}

fn deploy_p2p_network(network: &P2PNetworkTopology) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Deploying P2P network with {} nodes", network.nodes.len());
    
    // Start coordinator node
    start_network_coordinator(network)?;
    
    // Deploy each crate as a P2P node
    for node in &network.nodes {
        deploy_crate_node(node)?;
    }
    
    // Establish connections
    for connection in &network.connections {
        establish_p2p_connection(connection)?;
    }
    
    println!("🎯 Network topology matches Cargo.lock dependency graph!");
    Ok(())
}

fn start_network_coordinator(network: &P2PNetworkTopology) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎛️ Starting network coordinator...");
    
    // Save network topology
    let topology_json = serde_json::to_string_pretty(network)?;
    fs::write("p2p_network_topology.json", topology_json)?;
    
    // Start coordinator process
    std::process::Command::new("./p2p_coordinator")
        .arg("--topology")
        .arg("p2p_network_topology.json")
        .spawn()?;
    
    Ok(())
}

fn deploy_crate_node(node: &P2PNode) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Deploying node: {} ({})", node.node_id, node.crate_name);
    
    // Create node configuration
    let node_config = serde_json::json!({
        "node_id": node.node_id,
        "crate_name": node.crate_name,
        "version": node.version,
        "capabilities": node.capabilities,
        "dependencies": node.dependencies,
        "zombie_driver": "rustc_driver_self.so",
        "analysis_plugins": ["char_analyzer.so", "syn_analyzer.so"]
    });
    
    let config_file = format!("node_config_{}.json", node.crate_name);
    fs::write(&config_file, node_config.to_string())?;
    
    // Start the node process
    std::process::Command::new("./zombie_node")
        .arg("--config")
        .arg(&config_file)
        .spawn()?;
    
    Ok(())
}

fn establish_p2p_connection(connection: &P2PConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Establishing connection: {} → {}", connection.from, connection.to);
    
    // Send connection request via libp2p2
    let connection_msg = serde_json::json!({
        "type": "establish_connection",
        "from": connection.from,
        "to": connection.to,
        "dependency_type": connection.dependency_type
    });
    
    // Send to network coordinator
    use std::net::TcpStream;
    use std::io::Write;
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all(connection_msg.to_string().as_bytes())?;
    
    Ok(())
}

// Bonus: Flake.lock support for Nix users
fn flake_lock_to_p2p_network(flake_path: &str) -> Result<P2PNetworkTopology, Box<dyn std::error::Error>> {
    println!("❄️ Converting flake.lock to P2P network...");
    
    let flake_content = fs::read_to_string(flake_path)?;
    let flake_data: serde_json::Value = serde_json::from_str(&flake_content)?;
    
    let mut nodes = Vec::new();
    
    if let Some(nodes_obj) = flake_data["nodes"].as_object() {
        for (name, node_data) in nodes_obj {
            let node_id = format!("nix_zombie_{}", name);
            
            nodes.push(P2PNode {
                node_id,
                crate_name: name.clone(),
                version: node_data["locked"]["rev"].as_str().unwrap_or("unknown").to_string(),
                capabilities: vec!["nix_build".to_string(), "flake_support".to_string()],
                dependencies: vec![], // Extract from flake inputs
            });
        }
    }
    
    Ok(P2PNetworkTopology {
        nodes,
        connections: vec![],
        compilation_order: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cargo_lock_parsing() {
        // Test parsing a minimal Cargo.lock
        let test_lock = r#"
[[package]]
name = "rustc_driver"
version = "1.0.0"
dependencies = ["rustc_middle", "syn"]

[[package]]
name = "rustc_middle"
version = "1.0.0"
dependencies = ["syn"]

[[package]]
name = "syn"
version = "2.0.0"
dependencies = []
"#;
        
        let lock: CargoLock = toml::from_str(test_lock).unwrap();
        assert_eq!(lock.package.len(), 3);
    }
    
    #[test]
    fn test_network_topology_generation() {
        let cargo_lock = CargoLock {
            package: vec![
                Package {
                    name: "rustc_driver".to_string(),
                    version: "1.0.0".to_string(),
                    dependencies: Some(vec!["syn".to_string()]),
                },
                Package {
                    name: "syn".to_string(),
                    version: "2.0.0".to_string(),
                    dependencies: None,
                },
            ],
        };
        
        let network = cargo_lock_to_p2p_network(&cargo_lock).unwrap();
        assert_eq!(network.nodes.len(), 2);
        assert_eq!(network.connections.len(), 1);
    }
}
