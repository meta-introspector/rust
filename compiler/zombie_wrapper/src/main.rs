use libzombie::zombie_init;
use tokio::sync::mpsc;

#[cfg(feature = "rustc_private")]
use rustc_driver;

mod types;
mod troll_army;
mod network;

use types::*;
use troll_army::TrollArmy;
use network::start_libp2p_network;

#[tokio::main]
async fn main() {
    zombie_init();
    let mut troll_army = TrollArmy::new();
    
    let args: Vec<String> = std::env::args().collect();
    
    let (sender, receiver) = mpsc::unbounded_channel();
    tokio::spawn(start_libp2p_network(receiver));
    
    if args.len() > 1 {
        let start_time = std::time::Instant::now();
        
        #[cfg(feature = "rustc_private")]
        let exit_code = rustc_driver::catch_with_exit_code(|| {
            rustc_driver::RunCompiler::new(&args[1..], &mut troll_army).run()
        });
        
        #[cfg(not(feature = "rustc_private"))]
        let exit_code = {
            let mut cmd = std::process::Command::new("rustc");
            for arg in &args[1..] {
                cmd.arg(arg);
            }
            cmd.status().unwrap().code().unwrap_or(1)
        };
        
        let compilation_time = start_time.elapsed().as_millis() as u64;
        
        let data = CompilationData {
            peer_id: "zombie_node".to_string(),
            crate_name: args[1].clone(),
            ast_nodes: troll_army.span_fixmes.len() as u32,
            compilation_time_ms: compilation_time,
        };
        
        if let Err(e) = sender.send(data) {
            eprintln!("Failed to send compilation data: {}", e);
        }
        
        std::process::exit(exit_code);
    }
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
