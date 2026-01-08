#![feature(rustc_private)]

use std::env;
use std::process;

// Import rustc driver internals
use rustc_driver::Callbacks;
use rustc_interface::interface;

// LibP2P and async
use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, Swarm, Transport,
    futures::StreamExt,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize)]
struct CompilationData {
    peer_id: String,
    crate_name: String,
    ast_nodes: u32,
    compilation_time_ms: u64,
    tyctx_harmonics: Vec<f64>,
    topology: Vec<u32>,
}

#[derive(NetworkBehaviour)]
struct ZombieBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

struct ZombieCallbacks {
    data_sender: mpsc::UnboundedSender<CompilationData>,
    start_time: std::time::Instant,
}

impl ZombieCallbacks {
    fn new(sender: mpsc::UnboundedSender<CompilationData>) -> Self {
        Self {
            data_sender: sender,
            start_time: std::time::Instant::now(),
        }
    }

    fn serialize_tyctx<'tcx>(&self, tcx: rustc_middle::ty::TyCtxt<'tcx>) -> serde_json::Value {
        // Use serde to automatically serialize TyCtxt data
        serde_json::json!({
            "crate_name": tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string(),
            "crate_hash": format!("{:?}", tcx.crate_hash(rustc_span::def_id::LOCAL_CRATE))
        })
    }

    fn flatten_json(&self, json: &serde_json::Value) -> Vec<f64> {
        let mut flat = Vec::new();
        self.flatten_recursive(json, &mut flat);
        flat
    }

    fn flatten_recursive(&self, value: &serde_json::Value, flat: &mut Vec<f64>) {
        match value {
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    flat.push(f);
                }
            }
            serde_json::Value::String(s) => {
                // Convert string to numeric hash
                let hash = s.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
                flat.push(hash as f64);
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    self.flatten_recursive(item, flat);
                }
            }
            serde_json::Value::Object(obj) => {
                for (_, v) in obj {
                    self.flatten_recursive(v, flat);
                }
            }
            _ => {}
        }
    }

    fn compute_harmonics(&self, data: &[f64]) -> Vec<f64> {
        if data.is_empty() {
            return Vec::new();
        }
        
        let len = data.len();
        let mut harmonics = Vec::with_capacity(len.min(8));
        
        // Compute first 8 harmonics
        for h in 1..=8.min(len) {
            let mut sum = 0.0;
            for (i, &val) in data.iter().enumerate() {
                sum += val * (2.0 * std::f64::consts::PI * h as f64 * i as f64 / len as f64).sin();
            }
            harmonics.push(sum / len as f64);
        }
        
        harmonics
    }

    fn capture_topology(&self, data: &[f64]) -> Vec<u32> {
        if data.is_empty() {
            return Vec::new();
        }
        
        let mut topology = Vec::new();
        let chunk_size = (data.len() / 16).max(1);
        
        for chunk in data.chunks(chunk_size) {
            let sum: f64 = chunk.iter().sum();
            topology.push((sum.abs() as u32) % 1000);
        }
        
        topology
    }
}

impl Callbacks for ZombieCallbacks {
    fn config(&mut self, config: &mut interface::Config) {
        println!("🧟♂️ Zombie infected compiler config");
    }

    fn after_expansion<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        let crate_name = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
        let compilation_time = self.start_time.elapsed().as_millis() as u64;
        
        // Serialize TyCtxt to JSON
        let tyctx_json = self.serialize_tyctx(tcx);
        
        // Flatten JSON to array
        let flat_array = self.flatten_json(&tyctx_json);
        
        // Split array in harmonics
        let harmonics = self.compute_harmonics(&flat_array);
        
        // Capture topology
        let topology = self.capture_topology(&flat_array);
        
        let data = CompilationData {
            peer_id: "zombie_node".to_string(),
            crate_name,
            ast_nodes: flat_array.len() as u32,
            compilation_time_ms: compilation_time,
            tyctx_harmonics: harmonics,
            topology,
        };
        
        if let Err(e) = self.data_sender.send(data) {
            eprintln!("🧟 Failed to send TyCtxt data: {}", e);
        }
        
        rustc_driver::Compilation::Continue
    }
}

async fn start_libp2p_network(mut data_receiver: mpsc::UnboundedReceiver<CompilationData>) {
    println!("🧟 Starting zombie libp2p network...");
    
    // Create libp2p transport
    let transport = tcp::tokio::Transport::default()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&libp2p::identity::Keypair::generate_ed25519()).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    // Create network behaviour
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("🧠 Zombie peer ID: {}", local_peer_id);

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(std::time::Duration::from_secs(10))
        .validation_mode(gossipsub::ValidationMode::Strict)
        .build()
        .expect("Valid config");
    
    let gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    ).expect("Correct configuration");

    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap();
    
    let behaviour = ZombieBehaviour { gossipsub, mdns };
    let mut swarm = Swarm::new(
        transport, 
        behaviour, 
        local_peer_id,
        libp2p::swarm::Config::with_tokio_executor()
    );

    // Listen on all interfaces
    swarm.listen_on("/ip4/0.0.0.0/tcp/4001".parse().unwrap()).unwrap();
    
    // Subscribe to compilation data topic
    let topic = gossipsub::IdentTopic::new("zombie-compilation-data");
    swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();

    println!("🌐 Zombie network listening on port 4001");

    loop {
        tokio::select! {
            // Handle compilation data from rustc callbacks
            Some(data) = data_receiver.recv() => {
                println!("🧟♂️ Broadcasting compilation data: {} nodes", data.ast_nodes);
                let message = serde_json::to_string(&data).unwrap();
                if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.as_bytes()) {
                    eprintln!("🧟 Failed to publish: {}", e);
                }
            }
            
            // Handle libp2p events
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("🧠 Zombie listening on {}", address);
                    }
                    SwarmEvent::Behaviour(event) => {
                        match event {
                            ZombieBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                                propagation_source: _,
                                message_id: _,
                                message,
                            }) => {
                                if let Ok(data) = serde_json::from_slice::<CompilationData>(&message.data) {
                                    println!("🌐 Received zombie data from {}: {} AST nodes", 
                                             data.peer_id, data.ast_nodes);
                                }
                            }
                            ZombieBehaviourEvent::Mdns(mdns::Event::Discovered(list)) => {
                                for (peer_id, _) in list {
                                    println!("🧟♂️ Discovered zombie peer: {}", peer_id);
                                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    // Set required rustc environment variable FIRST
    std::env::set_var("CFG_COMPILER_HOST_TRIPLE", "x86_64-unknown-linux-gnu");
    
    // Debug: verify it's set
    println!("🔍 CFG_COMPILER_HOST_TRIPLE = {:?}", std::env::var("CFG_COMPILER_HOST_TRIPLE"));
    
    // Run async main
    tokio::runtime::Runtime::new().unwrap().block_on(async_main());
}

async fn async_main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "--network-mode" {
        // Pure network mode - just run the libp2p server
        let (_sender, receiver) = mpsc::unbounded_channel();
        start_libp2p_network(receiver).await;
        return;
    }
    
    // Check for network disable flags
    let disable_network = args.iter().any(|arg| arg == "--no-net" || arg == "--diagnose");
    
    // Filter out zombie-specific flags before passing to rustc
    let rustc_args: Vec<String> = args[1..].iter()
        .filter(|arg| !matches!(arg.as_str(), "--no-net" | "--diagnose"))
        .cloned()
        .collect();
    
    // Create channel for compilation data
    let (sender, receiver) = mpsc::unbounded_channel();
    
    // Start libp2p network in background only if not disabled
    if !disable_network {
        tokio::spawn(start_libp2p_network(receiver));
    }
    
    // Use rustc driver internals for compilation
    println!("🧟♂️ Zombie rustc driver starting with args: {:?}", &rustc_args);
    
    let mut callbacks = ZombieCallbacks::new(sender);
    
    // Run the compiler with zombie callbacks
    let exit_code = rustc_driver::catch_with_exit_code(|| {
        rustc_driver::run_compiler(&rustc_args, &mut callbacks);
    });
    
    println!("🧠 Zombie compilation complete with exit code: {}", exit_code);
    std::process::exit(exit_code);
}
