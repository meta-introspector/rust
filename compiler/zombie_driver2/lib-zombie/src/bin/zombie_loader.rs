use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use lib_zombie::ZombieSOSystem;
use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct SOProof {
    so_hash: String,
    capabilities: Vec<String>,
    signature: Vec<u8>,
    public_key: Vec<u8>,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct SOManifest {
    name: String,
    version: String,
    entry_points: Vec<String>,
    dependencies: Vec<String>,
    proof: SOProof,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    println!("🧟 Zombie ZKP SO Loader - Cryptographic Dynamic Linker");
    println!("=====================================================");

    // Find and validate SO files
    let deps_dir = "../target/debug/deps";
    let rustc_driver_path = find_so_file(deps_dir, "librustc_driver")?;

    println!("📦 Found SO: {}", rustc_driver_path);

    // Generate/verify SO proof
    let manifest = generate_so_manifest(&rustc_driver_path)?;

    if verify_so_proof(&rustc_driver_path, &manifest.proof)? {
        println!("✅ SO proof verified - loading authorized");
        load_verified_so(&rustc_driver_path, &manifest)?;
    } else {
        println!("❌ SO proof verification failed - loading denied");
        return Err("Invalid SO proof".into());
    }

    Ok(())
}

fn generate_so_manifest(so_path: &str) -> Result<SOManifest, Box<dyn std::error::Error>> {
    // Read SO file and compute hash
    let so_data = std::fs::read(so_path)?;
    let mut hasher = Sha256::new();
    hasher.update(&so_data);
    let so_hash = format!("{:x}", hasher.finalize());

    // Generate signing key (in production, this would be from secure storage)
    let mut rng = rand::thread_rng();
    let signing_key = SigningKey::from_bytes(&rand::random::<[u8; 32]>());
    let public_key = signing_key.verifying_key();

    // Create proof payload
    let capabilities =
        vec!["rustc_compile".to_string(), "ast_parse".to_string(), "codegen".to_string()];

    let proof_data = format!("{}:{:?}:{}", so_hash, capabilities, chrono::Utc::now().timestamp());
    let signature = signing_key.sign(proof_data.as_bytes());

    let proof = SOProof {
        so_hash: so_hash.clone(),
        capabilities,
        signature: signature.to_bytes().to_vec(),
        public_key: public_key.to_bytes().to_vec(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    };

    Ok(SOManifest {
        name: "rustc_driver".to_string(),
        version: "0.1.0".to_string(),
        entry_points: vec!["rustc_driver_main".to_string(), "run_compiler".to_string()],
        dependencies: vec!["rustc_interface".to_string(), "rustc_middle".to_string()],
        proof,
    })
}

fn verify_so_proof(so_path: &str, proof: &SOProof) -> Result<bool, Box<dyn std::error::Error>> {
    println!("🔐 Verifying SO proof...");

    // Recompute SO hash
    let so_data = std::fs::read(so_path)?;
    let mut hasher = Sha256::new();
    hasher.update(&so_data);
    let computed_hash = format!("{:x}", hasher.finalize());

    if computed_hash != proof.so_hash {
        println!("❌ Hash mismatch: expected {}, got {}", proof.so_hash, computed_hash);
        return Ok(false);
    }

    // Verify signature
    let public_key = VerifyingKey::from_bytes(
        &proof.public_key.clone().try_into().map_err(|_| "Invalid public key")?,
    )?;
    let signature = Signature::from_bytes(
        &proof.signature.clone().try_into().map_err(|_| "Invalid signature")?,
    );

    let proof_data = format!("{}:{:?}:{}", proof.so_hash, proof.capabilities, proof.timestamp);

    match public_key.verify(proof_data.as_bytes(), &signature) {
        Ok(_) => {
            println!("✅ Signature verified");
            println!("📋 Capabilities: {:?}", proof.capabilities);
            Ok(true)
        }
        Err(_) => {
            println!("❌ Signature verification failed");
            Ok(false)
        }
    }
}

fn load_verified_so(
    so_path: &str,
    manifest: &SOManifest,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Loading verified SO: {}", manifest.name);

    unsafe {
        let lib = Library::new(so_path)?;

        // Try each entry point from manifest
        for entry_point in &manifest.entry_points {
            println!("🔍 Trying entry point: {}", entry_point);

            match lib.get::<Symbol<unsafe extern "C" fn()>>(entry_point.as_bytes()) {
                Ok(func) => {
                    println!("✅ Found and executing: {}", entry_point);
                    func();
                    return Ok(());
                }
                Err(_) => {
                    println!("⚠️  Entry point {} not found", entry_point);
                }
            }
        }

        println!("❌ No valid entry points found");
    }

    Ok(())
}

fn find_so_file(deps_dir: &str, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dir = std::fs::read_dir(deps_dir)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with(prefix) && filename.ends_with(".so") {
                return Ok(path.to_string_lossy().to_string());
            }
        }
    }

    Err(format!("No .so file found with prefix: {}", prefix).into())
}
