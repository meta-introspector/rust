// Test P2P verbs for .so management
use tokio;

mod plugin_driver;
mod p2p_server;

use p2p_server::{P2PPluginServer, P2PVerb};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Testing P2P Plugin Verbs");
    
    let mut server = P2PPluginServer::new();
    
    // Test verb sequence
    let verbs = vec![
        "LOAD_SO span_wrapper target/release/libspan_wrapper.so",
        "REGISTER span_wrapper 1",
        "ATTACH span_wrapper test_data",
        "RUN_FILES span_wrapper test.rs",
        "CAPTURE span_wrapper",
    ];
    
    for verb_msg in verbs {
        match server.process_message(verb_msg).await {
            Ok(result) => println!("✅ {}: {}", verb_msg, result),
            Err(e) => println!("❌ {}: {}", verb_msg, e),
        }
    }
    
    println!("🎉 P2P verb tests completed!");
    Ok(())
}
