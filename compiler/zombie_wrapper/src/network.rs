use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, Swarm, Transport,
    futures::StreamExt,
};
use tokio::sync::mpsc;
use crate::types::*;

#[derive(NetworkBehaviour)]
pub struct ZombieBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub async fn start_libp2p_network(mut data_receiver: mpsc::UnboundedReceiver<CompilationData>) {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let transport = tcp::tokio::Transport::default()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&local_key).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(std::time::Duration::from_secs(10))
        .validation_mode(gossipsub::ValidationMode::Strict)
        .build()
        .expect("Valid config");

    let gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    ).expect("Correct configuration");

    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)
        .expect("can create mdns");

    let behaviour = ZombieBehaviour { gossipsub, mdns };
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id, libp2p::swarm::Config::with_tokio_executor());

    swarm.listen_on("/ip4/0.0.0.0/tcp/4001".parse().unwrap()).unwrap();

    let topic = gossipsub::IdentTopic::new("zombie-compilation");
    swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();

    loop {
        tokio::select! {
            Some(data) = data_receiver.recv() => {
                let message = serde_json::to_string(&data).unwrap();
                if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.as_bytes()) {
                    eprintln!("Failed to publish: {}", e);
                }
            }
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Listening on {}", address);
                    }
                    _ => {}
                }
            }
        }
    }
}
