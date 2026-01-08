use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, Swarm, Transport,
    futures::StreamExt,
};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tokio::time::{sleep, Duration};
use chrono;
use std::io::Write;
use std::collections::HashMap;

fn generate_feature_combinations(features: &[String]) -> Vec<Vec<String>> {
    let mut combinations = Vec::new();
    let n = features.len();
    
    for i in 1..(1 << n) {
        let mut combo = Vec::new();
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                combo.push(features[j].clone());
            }
        }
        combinations.push(combo);
    }
    combinations
}

fn filter_code_by_combination(source_code: &str, feature_combo: &[String]) -> String {
    let mut filtered_lines = Vec::new();
    
    for line in source_code.lines() {
        let mut should_include = false;
        
        for feature in feature_combo {
            let matches = match feature.as_str() {
                "declaration" => line.contains("fn ") || line.contains("struct "),
                "binding" => line.contains("let ") || line.contains("const "),
                "control_flow" => line.contains("if ") || line.contains("match "),
                _ => false,
            };
            
            if matches {
                should_include = true;
                break;
            }
        }
        
        if should_include || line.trim().is_empty() {
            filtered_lines.push(line);
        }
    }
    
    filtered_lines.join("\n")
}

fn peer_to_emoji(peer_id: &PeerId) -> String {
    let emojis = ["🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮", "🐷", "🐸", "🐵", "🐔"];
    let bytes = peer_id.to_bytes();
    (0..8).map(|i| emojis[bytes[i % bytes.len()] as usize % emojis.len()]).collect()
}

fn log_message(msg: &str) {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let log_entry = format!("{}: {}", timestamp, msg);
    println!("{}", log_entry);
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("zombie_client_verbose.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
}

fn log_verbose(category: &str, data: &str) {
    let msg = format!("[{}] {}", category, data);
    log_message(&msg);
}

fn log_send_request(request_id: &str, payload: &str) {
    log_verbose("SEND_REQUEST", &format!("ID:{} PAYLOAD:{}", request_id, payload));
}

fn log_receive_response(response: &str) {
    log_verbose("RECEIVE_RESPONSE", response);
}

#[derive(Debug, Serialize, Deserialize)]
struct CompilationRequest {
    peer_id: String,
    file_path: String,
    request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpawnServerRequest {
    peer_id: String,
    request_id: String,
    new_port: u16,
    server_variant: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpawnServerResult {
    peer_id: String,
    request_id: String,
    success: bool,
    new_peer_id: Option<String>,
    new_port: u16,
    spawn_output: String,
    compile_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompilationResult {
    peer_id: String,
    request_id: String,
    success: bool,
    output: String,
    compilation_time_ms: u64,
}

#[derive(NetworkBehaviour)]
struct ClientBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

async fn spawn_zombie_server(port: u16, variant: &str) -> String {
    log_message(&format!("🧟 Spawning {} zombie server on port {}...", variant, port));
    
    let output = std::process::Command::new("cargo")
        .args(&["run", "--release", "--bin", "zombie_rustc"])
        .current_dir("../zombie_wrapper")
        .env("ZOMBIE_PORT", port.to_string())
        .env("ZOMBIE_VARIANT", variant)
        .spawn();
    
    match output {
        Ok(_) => format!("{}:{} spawned", variant, port),
        Err(e) => format!("{}:{} failed: {}", variant, port, e),
    }
}
    log_message("🚀 Starting zombie server...");
    
    // First compile the server
    log_message("🔨 Compiling zombie server...");
    let compile_output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_wrapper")
        .env("RUSTFLAGS", "--cfg bootstrap")
        .output()
        .expect("Failed to compile zombie server");
    
    if !compile_output.status.success() {
        log_message(&format!("❌ Server compilation failed: {}", String::from_utf8_lossy(&compile_output.stderr)));
        panic!("Server compilation failed");
    }
    log_message("✅ Server compiled successfully");
    
    let child = Command::new("cargo")
        .args(&["run", "--release", "--quiet", "test_program.rs"])
        .current_dir("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/zombie_wrapper")
        .env("RUSTFLAGS", "--cfg bootstrap")
        .spawn()
        .expect("Failed to start zombie server");
    
    // Wait for server to start
    sleep(Duration::from_secs(8)).await;
    log_message("✅ Zombie server should be running");
    
    child
}

async fn create_client() -> Swarm<ClientBehaviour> {
    println!("🔌 Creating zombie client...");
    
    let transport = tcp::tokio::Transport::default()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&libp2p::identity::Keypair::generate_ed25519()).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("🆔 Client peer ID: {} {}", local_peer_id, peer_to_emoji(&local_peer_id));

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        .validation_mode(gossipsub::ValidationMode::Strict)
        .build()
        .expect("Valid config");
    
    let gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    ).expect("Correct configuration");

    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap();
    
    let behaviour = ClientBehaviour { gossipsub, mdns };
    Swarm::new(
        transport, 
        behaviour, 
        local_peer_id,
        libp2p::swarm::Config::with_tokio_executor()
    )
}

async fn run_client_test() {
    let mut swarm = create_client().await;
    
    // Subscribe to topics
    let data_topic = gossipsub::IdentTopic::new("zombie-compilation-data");
    let request_topic = gossipsub::IdentTopic::new("zombie-compilation-request");
    let result_topic = gossipsub::IdentTopic::new("zombie-compilation-result");
    let spawn_request_topic = gossipsub::IdentTopic::new("zombie-spawn-server-request");
    let spawn_result_topic = gossipsub::IdentTopic::new("zombie-spawn-server-result");
    
    swarm.behaviour_mut().gossipsub.subscribe(&data_topic).unwrap();
    swarm.behaviour_mut().gossipsub.subscribe(&result_topic).unwrap();
    swarm.behaviour_mut().gossipsub.subscribe(&spawn_result_topic).unwrap();
    
    println!("📡 Client subscribed to topics");
    
    let mut request_sent = false;
    let mut peers_discovered = 0;
    
    loop {
        tokio::select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(event) => {
                        match event {
                            ClientBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                                propagation_source: _,
                                message_id: _,
                                message,
                            }) => {
                                if let Ok(spawn_result) = serde_json::from_slice::<SpawnServerResult>(&message.data) {
                                    log_verbose("SPAWN_RESULT_RECEIVED", &format!("Spawn result: success={}, port={}", spawn_result.success, spawn_result.new_port));
                                    
                                    if spawn_result.success {
                                        // Immediately try to connect to the new server
                                        log_message(&format!("🔗 Attempting to peer with new server on port {}", spawn_result.new_port));
                                        
                                        // Add the new server as an explicit peer
                                        let new_multiaddr = format!("/ip4/127.0.0.1/tcp/{}", spawn_result.new_port);
                                        log_verbose("PEER_CONNECT", &format!("Connecting to {}", new_multiaddr));
                                        
                                        // Send immediate compilation request to new peer
                                        if let Some(new_peer_id) = &spawn_result.new_peer_id {
                                            let test_request = CompilationRequest {
                                                peer_id: new_peer_id.clone(),
                                                file_path: "test_file.rs".to_string(),
                                                request_id: format!("test_new_peer_{}", chrono::Utc::now().timestamp()),
                                            };
                                            
                                            let test_message = serde_json::to_string(&test_request).unwrap();
                                            log_verbose("NEW_PEER_REQUEST", &format!("Sending test request to new peer: {}", test_message));
                                            
                                            if let Err(e) = swarm.behaviour_mut().gossipsub.publish(request_topic.clone(), test_message.as_bytes()) {
                                                log_verbose("NEW_PEER_ERROR", &format!("Failed to send to new peer: {}", e));
                                            } else {
                                                log_verbose("NEW_PEER_SUCCESS", "Sent test request to newly spawned peer");
                                            }
                                        }
                                    }
                                } else if let Ok(result) = serde_json::from_slice::<CompilationResult>(&message.data) {
                                    log_message(&format!("📨 Received compilation result: success={}, output={}", result.success, result.output));
                                } else if let Ok(data) = serde_json::from_slice::<serde_json::Value>(&message.data) {
                                    log_message(&format!("📨 Received message: {}", data));
                                }
                            }
                            ClientBehaviourEvent::Mdns(mdns::Event::Discovered(list)) => {
                                for (peer_id, _) in list {
                                    // Skip self-discovery
                                    if peer_id == *swarm.local_peer_id() {
                                        continue;
                                    }
                                    
                                    println!("🔍 Discovered zombie server: {} {}", peer_id, peer_to_emoji(&peer_id));
                                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                    peers_discovered += 1;
                                    
                                    // Send compilation request after discovering server
                                    if !request_sent && peers_discovered >= 2 {
                                        println!("🎯 Connecting to peer: {}", peer_id);
                                        
                                        let request = CompilationRequest {
                                            peer_id: peer_id.to_string(),
                                            file_path: "test_file.rs".to_string(),
                                            request_id: format!("req_{}", chrono::Utc::now().timestamp()),
                                        };
                                        
                                        let message = serde_json::to_string(&request).unwrap();
                                        log_send_request(&request.request_id, &message);
                                        
                                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(request_topic.clone(), message.as_bytes()) {
                                            log_verbose("SEND_ERROR", &format!("Failed to send request: {}", e));
                                        } else {
                                            log_verbose("SEND_SUCCESS", &format!("Sent request to peer {} {}", peer_id, peer_to_emoji(&peer_id)));
                                            request_sent = true;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            
            // Timeout after 30 seconds
            _ = sleep(Duration::from_secs(30)) => {
                println!("⏰ Test timeout reached");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    log_message("🧟♂️ Zombie Client Test Starting...");
    
    // Kill any existing zombie processes
    log_message("🔪 Killing existing zombie processes...");
    let _ = Command::new("pkill").args(&["-f", "zombie"]).output();
    sleep(Duration::from_secs(2)).await;
    
    // Check ports before starting
    log_message("🔍 Checking port 4001...");
    let port_check = Command::new("netstat").args(&["-tlnp"]).output();
    if let Ok(output) = port_check {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("4001") {
            log_message("⚠️  Port 4001 still in use - killing processes");
            let _ = Command::new("fuser").args(&["-k", "4001/tcp"]).output();
            sleep(Duration::from_secs(2)).await;
        } else {
            log_message("✅ Port 4001 is free");
        }
    }
    
    // Client is now a server - compile and spawn zombie servers directly
    log_message("🧬 Client-Server: Compiling and spawning zombie servers...");
    
    // Spawn first zombie server
    let spawn_result_1 = spawn_zombie_server(4001, "primary").await;
    log_message(&format!("🧟 Spawned primary zombie: {}", spawn_result_1));
    
    // Spawn second zombie server  
    let spawn_result_2 = spawn_zombie_server(4002, "secondary").await;
    log_message(&format!("🧟 Spawned secondary zombie: {}", spawn_result_2));
    
    // Spawn fuzzing zombie server
    let spawn_result_3 = spawn_zombie_server(4003, "fuzzer").await;
    log_message(&format!("🧟 Spawned fuzzer zombie: {}", spawn_result_3));
    
    log_message("🌐 Zombie network cluster spawned - 3 servers running");
    
    // Wait for servers to initialize
    sleep(Duration::from_secs(5)).await;
    
    // Test spawning new server
    log_message("🧬 Testing server spawning...");
    if !request_sent && peers_discovered >= 1 {
        let spawn_request = SpawnServerRequest {
            peer_id: "client".to_string(),
            request_id: format!("spawn_{}", chrono::Utc::now().timestamp()),
            new_port: 4002,
            server_variant: "enhanced".to_string(),
        };
        
        let spawn_message = serde_json::to_string(&spawn_request).unwrap();
        log_verbose("SPAWN_REQUEST", &format!("Requesting spawn: {}", spawn_message));
        
        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(spawn_request_topic.clone(), spawn_message.as_bytes()) {
            log_verbose("SPAWN_SEND_ERROR", &format!("Failed to send spawn request: {}", e));
        } else {
            log_verbose("SPAWN_SEND_SUCCESS", "Sent server spawn request");
        }
    }
    if let Ok(test_content) = std::fs::read_to_string("test_file.rs") {
        let features = vec!["declaration".to_string(), "binding".to_string(), "control_flow".to_string()];
        let combinations = generate_feature_combinations(&features);
        log_message(&format!("🧮 Generated {} feature combinations", combinations.len()));
        
        for (i, combo) in combinations.iter().take(5).enumerate() {
            let filtered = filter_code_by_combination(&test_content, combo);
            log_message(&format!("🔬 Combo {}: {:?} -> {} lines", i+1, combo, filtered.lines().count()));
        }
    } else {
        // Create a simple test if file doesn't exist
        let simple_test = "fn main() {\n    let x = 42;\n    if x > 0 {\n        println!(\"Hello\");\n    }\n}";
        let features = vec!["declaration".to_string(), "binding".to_string()];
        let combinations = generate_feature_combinations(&features);
        log_message(&format!("🧮 Simple test: {} combinations", combinations.len()));
        
        for (i, combo) in combinations.iter().enumerate() {
            let filtered = filter_code_by_combination(simple_test, combo);
            log_message(&format!("🔬 Combo {}: {:?} -> {} lines", i+1, combo, filtered.lines().count()));
        }
    }
    
    // Run the client test
    run_client_test().await;
    
    // Kill the server
    log_message("🔪 Killing zombie server...");
    let _ = server.kill();
    let _ = Command::new("pkill").args(&["-f", "zombie"]).output();
    
    // Check ports after killing
    sleep(Duration::from_secs(2)).await;
    log_message("🔍 Checking port 4001 after cleanup...");
    let port_check = Command::new("netstat").args(&["-tlnp"]).output();
    if let Ok(output) = port_check {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("4001") {
            log_message("⚠️  Port 4001 still in use after cleanup - force killing");
            let _ = Command::new("fuser").args(&["-k", "4001/tcp"]).output();
        } else {
            log_message("✅ Port 4001 is free after cleanup");
        }
    }
    
    log_message("🏁 Test completed");
}
