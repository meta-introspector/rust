use libp2p::{
    gossipsub, mdns, swarm::SwarmEvent, Swarm, PeerId,
};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};
use crate::rustc_matrix::{RustcMatrix, RustcMessage};

mod rustc_matrix;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Initialize the RustcMatrix
    let rustc_matrix = RustcMatrix::new();
    
    // Load our symbol map data
    println!("🔄 Loading rustc symbols as message handlers...");
    rustc_matrix.load_from_symbol_map("symbol_map_original.json.gz").await?;

    // Create libp2p swarm
    let mut swarm = create_swarm(&rustc_matrix).await?;
    
    // Subscribe to rustc topic
    swarm.behaviour_mut().gossipsub.subscribe(rustc_matrix.get_topic())?;
    
    // Listen on all interfaces
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("🚀 Rustc Matrix Server started!");
    println!("📡 All rustc symbols are now libp2p message types");
    println!("Commands (sent as libp2p messages):");
    println!("  compile rustc_driver::main");
    println!("  execute rustc_driver::main");
    println!("  resolve rustc_middle::ty::Ty");
    println!("  start_rustc");

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                if let Ok(Some(line)) = line {
                    handle_command(&mut swarm, &rustc_matrix, &line).await?;
                }
            }
            event = swarm.select_next_some() => {
                handle_swarm_event(&rustc_matrix, event).await;
            }
        }
    }
}

async fn create_swarm(matrix: &RustcMatrix) -> Result<Swarm<rustc_matrix::RustcBehaviour>, Box<dyn Error>> {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("🔑 Local peer id: {local_peer_id}");

    let transport = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true))
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(libp2p::noise::Config::new(&local_key)?)
        .multiplex(libp2p::yamux::Config::default())
        .boxed();

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(std::time::Duration::from_secs(10))
        .validation_mode(gossipsub::ValidationMode::Strict)
        .build()
        .expect("Valid config");

    let gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    )?;

    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;

    let behaviour = rustc_matrix::RustcBehaviour { gossipsub, mdns };
    let swarm = Swarm::with_tokio_executor(transport, behaviour, local_peer_id);

    Ok(swarm)
}

async fn handle_command(
    swarm: &mut Swarm<rustc_matrix::RustcBehaviour>, 
    matrix: &RustcMatrix, 
    command: &str
) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    
    let message = match parts.get(0) {
        Some(&"compile") => {
            if let Some(symbol) = parts.get(1) {
                RustcMessage::CompileRequest {
                    symbol: symbol.to_string(),
                    dependencies: vec![],
                }
            } else {
                println!("Usage: compile <symbol>");
                return Ok(());
            }
        }
        Some(&"execute") => {
            if let Some(symbol) = parts.get(1) {
                RustcMessage::ExecuteSymbol {
                    symbol: symbol.to_string(),
                    args: parts[2..].iter().map(|s| s.to_string()).collect(),
                }
            } else {
                println!("Usage: execute <symbol> [args...]");
                return Ok(());
            }
        }
        Some(&"resolve") => {
            if let Some(symbol) = parts.get(1) {
                RustcMessage::ResolveDependencies {
                    symbol: symbol.to_string(),
                }
            } else {
                println!("Usage: resolve <symbol>");
                return Ok(());
            }
        }
        Some(&"start_rustc") => {
            RustcMessage::StartRustc
        }
        _ => {
            println!("Unknown command: {}", command);
            return Ok(());
        }
    };

    // Handle message locally and broadcast response
    if let Some(response) = matrix.handle_message(message.clone()).await {
        let response_json = serde_json::to_string(&response)?;
        swarm.behaviour_mut().gossipsub.publish(matrix.get_topic(), response_json.as_bytes())?;
        println!("📡 Broadcasted response: {:?}", response);
    }

    // Also broadcast the original message
    let message_json = serde_json::to_string(&message)?;
    swarm.behaviour_mut().gossipsub.publish(matrix.get_topic(), message_json.as_bytes())?;
    println!("📡 Broadcasted message: {:?}", message);
    
    Ok(())
}

async fn handle_swarm_event(
    matrix: &RustcMatrix,
    event: SwarmEvent<rustc_matrix::RustcBehaviourEvent>
) {
    match event {
        SwarmEvent::NewListenAddr { address, .. } => {
            println!("🌐 Listening on {address}");
        }
        SwarmEvent::Behaviour(rustc_matrix::RustcBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, multiaddr) in list {
                println!("🔍 Discovered rustc peer: {peer_id} at {multiaddr}");
            }
        }
        SwarmEvent::Behaviour(rustc_matrix::RustcBehaviourEvent::Gossipsub(gossipsub::Event::Message {
            propagation_source: peer_id,
            message_id: _id,
            message,
        })) => {
            if let Ok(message_str) = String::from_utf8(message.data.clone()) {
                if let Ok(rustc_message) = serde_json::from_str::<RustcMessage>(&message_str) {
                    println!("📨 Received rustc message from {peer_id}: {:?}", rustc_message);
                    
                    // Handle the message and potentially respond
                    if let Some(response) = matrix.handle_message(rustc_message).await {
                        println!("🔄 Generated response: {:?}", response);
                    }
                }
            }
        }
        _ => {}
    }
}
