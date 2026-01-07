use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--network-mode" {
        println!("🧟 Starting zombie libp2p network...");
        println!("🧠 Zombie brain listening on port 4001");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            println!("🧟‍♂️ Zombie rustc network heartbeat...");
        }
    }
    
    // Normal rustc compilation with zombie infection
    println!("🧟‍♂️ Zombie rustc compiling: {:?}", &args[1..]);
    
    let mut cmd = Command::new("rustc");
    for arg in args.iter().skip(1) {
        cmd.arg(arg);
    }
    
    match cmd.status() {
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("🧟 Zombie rustc failed: {}", e);
            std::process::exit(1);
        }
    }
}
