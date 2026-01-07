use rustc_brainrot_driver::router::{BrainrotRouter, CompilationRequest};
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠🧟‍♂️ BRAINROT COMPILATION ROUTER STARTING");
    println!("==========================================");
    
    let mut router = BrainrotRouter::new();
    let mut health_check = interval(Duration::from_secs(30));
    let mut status_report = interval(Duration::from_secs(60));
    
    // Test compilation requests with different complexities
    let test_requests = vec![
        CompilationRequest {
            source_code: "fn main() {}".to_string(),
            complexity: 1,
            target: "simple".to_string(),
            requester_id: "test_1".to_string(),
        },
        CompilationRequest {
            source_code: "struct Complex { a: i32, b: String } fn process() -> Result<(), Error> { Ok(()) }".to_string(),
            complexity: 5,
            target: "medium".to_string(),
            requester_id: "test_2".to_string(),
        },
        CompilationRequest {
            source_code: "enum BigEnum { A, B, C, D, E } impl BigEnum { fn complex_method(&self) -> Vec<HashMap<String, Option<Result<i32, Error>>>> { vec![] } }".to_string(),
            complexity: 15,
            target: "complex".to_string(),
            requester_id: "test_3".to_string(),
        },
    ];
    
    // Route initial test compilations
    for request in test_requests {
        let complexity = request.analyze_complexity();
        println!("📝 Routing compilation: complexity {} -> {}", complexity, request.target);
        
        match router.route_compilation(complexity, &request.source_code).await {
            Ok(result) => println!("✅ {}", result),
            Err(e) => println!("❌ Routing failed: {}", e),
        }
        
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
    
    println!("\n🌐 Router entering eternal operation mode...");
    
    loop {
        tokio::select! {
            _ = health_check.tick() => {
                println!("🏥 Checking zombie health...");
                router.check_zombie_health().await;
            }
            
            _ = status_report.tick() => {
                let status = router.get_zombie_network_status().await;
                println!("📊 Network Status: {} active zombies", status.len());
                
                for (complexity, zombie) in status {
                    println!("  🧟‍♂️ Zombie {} (C{}): {:?} - {} brainrot memes", 
                        zombie.zombie_id, complexity, zombie.status, zombie.brainrot_count);
                }
                
                // Broadcast network heartbeat
                router.broadcast_to_all_zombies("ROUTER_HEARTBEAT").await;
            }
        }
    }
}
