use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

// 🧟 EMOJI SPORE PROTOCOL: Transmit rustc as emojis
const EMOJI_CODEBOOK: [&str; 256] = [
    "🧟","🦀","🔥","💀","🚀","⚡","🌙","🎯","🔧","🧬","💎","🌊","🎭","🔮","🌟","⭐",
    "🎪","🎨","🎵","🎸","🎺","🎻","🥁","🎤","🎧","🎬","🎮","🕹️","🎲","🎯","🎳","🎪",
    "🚗","🚕","🚙","🚌","🚎","🏎️","🚓","🚑","🚒","🚐","🛻","🚚","🚛","🚜","🏍️","🛵",
    "⛵","🚤","🛥️","🛳️","⛴️","🚢","✈️","🛩️","🛫","🛬","🪂","💺","🚁","🚟","🚠","🚡",
    "🚀","🛸","🛰️","💫","⭐","🌟","✨","⚡","☄️","🌍","🌎","🌏","🌕","🌖","🌗","🌘",
    "🌑","🌒","🌓","🌔","🌙","🌛","🌜","☀️","🌝","🌞","⭐","🌟","🌠","☁️","⛅","⛈️",
    "🌤️","🌦️","🌧️","⛈️","🌩️","🌨️","❄️","☃️","⛄","🌬️","💨","🌪️","🌫️","🌈","☂️","☔",
    "⚡","❄️","☃️","⛄","🔥","💧","🌊","💦","💨","💫","💥","💢","💯","💤","💨","💦",
    "🔴","🟠","🟡","🟢","🔵","🟣","⚫","⚪","🟤","🔺","🔻","🔸","🔹","🔶","🔷","🔳",
    "🔲","▪️","▫️","◾","◽","◼️","◻️","⬛","⬜","🟫","🟪","🟦","🟩","🟨","🟧","🟥",
    "💀","☠️","👻","👽","👾","🤖","🎃","😈","👿","👹","👺","🔥","💥","⚡","💫","💥",
    "🧟","🦇","🕷️","🕸️","🦂","🐍","🦎","🐲","🐉","🦕","🦖","🐊","🐢","🐸","🐝","🐛",
    "🦋","🐌","🐞","🐜","🦗","🕷️","🦂","🦟","🦠","💐","🌸","💮","🏵️","🌹","🥀","🌺",
    "🌻","🌼","🌷","⚘️","🌱","🪴","🌲","🌳","🌴","🌵","🌶️","🫑","🥒","🥬","🥦","🧄",
    "🧅","🍄","🥜","🌰","🍞","🥐","🥖","🫓","🥨","🥯","🥞","🧇","🧀","🍖","🍗","🥩",
    "🥓","🍔","🍟","🍕","🌭","🥪","🌮","🌯","🫔","🥙","🧆","🥚","🍳","🥘","🍲","🫕"
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧟 EMOJI SPORE TRANSMISSION PROTOCOL");
    println!("===================================");
    
    // Load zombie spore
    let spore = std::fs::read("zombie_spore_winner.so")
        .or_else(|_| std::fs::read("target/debug/deps/librustc_driver.so"))
        .unwrap_or_else(|_| b"MOCK_SPORE_DATA".to_vec());
    
    println!("📦 Loaded spore: {} bytes", spore.len());
    
    // Encode to emojis
    let emoji_spore = encode_spore_to_emojis(&spore);
    println!("🎭 Encoded to {} emojis", emoji_spore.chars().count());
    
    // Test round-trip
    let decoded = decode_emojis_to_spore(&emoji_spore);
    println!("✅ Round-trip test: {}", if decoded == spore { "PASS" } else { "FAIL" });
    
    // Start emoji spore server
    start_emoji_spore_server(&spore)?;
    
    Ok(())
}

fn encode_spore_to_emojis(spore: &[u8]) -> String {
    spore.iter()
        .map(|&byte| EMOJI_CODEBOOK[byte as usize])
        .collect::<String>()
}

fn decode_emojis_to_spore(emoji_packet: &str) -> Vec<u8> {
    let mut spore = Vec::new();
    let mut emoji_map = HashMap::new();
    
    // Build reverse lookup
    for (i, &emoji) in EMOJI_CODEBOOK.iter().enumerate() {
        emoji_map.insert(emoji, i as u8);
    }
    
    // Decode each emoji
    let mut chars = emoji_packet.chars();
    while let Some(ch) = chars.next() {
        // Try to match emoji (some are multi-char)
        let mut emoji_str = ch.to_string();
        
        // Check for multi-char emojis
        while let Some(next_ch) = chars.as_str().chars().next() {
            let test_emoji = format!("{}{}", emoji_str, next_ch);
            if emoji_map.contains_key(test_emoji.as_str()) {
                emoji_str = test_emoji;
                chars.next(); // consume the character
            } else {
                break;
            }
        }
        
        if let Some(&byte_val) = emoji_map.get(emoji_str.as_str()) {
            spore.push(byte_val);
        }
    }
    
    spore
}

fn start_emoji_spore_server(spore: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Starting emoji spore server on port 8080...");
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("📨 Request: {}", request.trim());
        
        if request.starts_with("GET_EMOJI_SPORE") {
            // Send spore as emojis
            let emoji_spore = encode_spore_to_emojis(spore);
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}", emoji_spore);
            stream.write_all(response.as_bytes())?;
            println!("📡 Sent emoji spore: {} emojis", emoji_spore.chars().count());
            
        } else if request.starts_with("DECODE_EMOJI:") {
            // Decode received emoji spore
            let emoji_data = request.strip_prefix("DECODE_EMOJI:").unwrap_or("").trim();
            let decoded_spore = decode_emojis_to_spore(emoji_data);
            
            // Save decoded spore
            std::fs::write("received_spore.so", &decoded_spore)?;
            
            let response = format!("HTTP/1.1 200 OK\r\n\r\nDecoded {} emojis to {} bytes", 
                                 emoji_data.chars().count(), decoded_spore.len());
            stream.write_all(response.as_bytes())?;
            println!("🔄 Decoded emoji spore: {} bytes", decoded_spore.len());
            
        } else {
            // Send emoji spore info
            let info = format!(
                "🧟 EMOJI SPORE PROTOCOL\n\
                 📦 Spore size: {} bytes\n\
                 🎭 Emoji encoding: {} emojis\n\
                 📊 Compression ratio: {:.1}%\n\
                 \n\
                 Commands:\n\
                 GET_EMOJI_SPORE - Download spore as emojis\n\
                 DECODE_EMOJI:<emojis> - Decode emoji spore\n",
                spore.len(),
                encode_spore_to_emojis(spore).chars().count(),
                (spore.len() as f64 / (encode_spore_to_emojis(spore).len() as f64 / 4.0)) * 100.0
            );
            
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}", info);
            stream.write_all(response.as_bytes())?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_emoji_encoding_roundtrip() {
        let test_data = b"Hello, zombie world! 🧟🦀";
        let encoded = encode_spore_to_emojis(test_data);
        let decoded = decode_emojis_to_spore(&encoded);
        assert_eq!(test_data.to_vec(), decoded);
    }
    
    #[test]
    fn test_compression_efficiency() {
        let test_data = vec![0u8; 1024]; // 1KB of zeros
        let encoded = encode_spore_to_emojis(&test_data);
        let compression_ratio = test_data.len() as f64 / (encoded.len() as f64 / 4.0);
        println!("Compression ratio: {:.2}", compression_ratio);
        assert!(compression_ratio > 0.8); // Should be reasonably efficient
    }
    
    #[test]
    fn test_all_bytes_encodable() {
        for i in 0..=255u8 {
            let data = vec![i];
            let encoded = encode_spore_to_emojis(&data);
            let decoded = decode_emojis_to_spore(&encoded);
            assert_eq!(data, decoded, "Failed for byte value: {}", i);
        }
    }
}
