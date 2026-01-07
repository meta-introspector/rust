use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ComputeResource {
    pub name: String,
    pub resource_type: ResourceType,
    pub cores: u32,
    pub memory_gb: u32,
    pub available: bool,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    CPU,
    GPU { model: String, vram_gb: u32 },
    LLM { model: String, context_size: u32 },
}

#[derive(Debug, Clone)]
pub struct RustBootstrapProfile {
    pub compilation_steps: Vec<CompilationStep>,
    pub perf_data: Vec<PerfRecord>,
    pub self_profile_data: Vec<SelfProfileRecord>,
    pub critical_path: Vec<usize>, // indices into compilation_steps
}

#[derive(Debug, Clone)]
pub struct CompilationStep {
    pub step_id: usize,
    pub phase: String, // "parse", "expand", "resolve", "typecheck", "codegen"
    pub duration_ms: u64,
    pub memory_peak_mb: u64,
    pub cpu_usage_percent: f64,
}

#[derive(Debug, Clone)]
pub struct PerfRecord {
    pub timestamp: u64,
    pub event_type: String,
    pub cpu_cycles: u64,
    pub instructions: u64,
    pub cache_misses: u64,
}

#[derive(Debug, Clone)]
pub struct SelfProfileRecord {
    pub query_name: String,
    pub duration_ns: u64,
    pub invocation_count: u32,
    pub self_time_ns: u64,
}

pub struct MaximalValueExtractor {
    pub available_resources: Vec<ComputeResource>,
    pub bootstrap_profile: RustBootstrapProfile,
}

impl MaximalValueExtractor {
    pub fn catalog_resources() -> Vec<ComputeResource> {
        let mut resources = Vec::new();
        
        // Catalog CPUs
        if let Ok(output) = Command::new("nproc").output() {
            if let Ok(cores_str) = String::from_utf8(output.stdout) {
                if let Ok(cores) = cores_str.trim().parse::<u32>() {
                    resources.push(ComputeResource {
                        name: "Local CPU".to_string(),
                        resource_type: ResourceType::CPU,
                        cores,
                        memory_gb: 32, // Default estimate
                        available: true,
                    });
                }
            }
        }
        
        // Catalog GPUs
        if let Ok(output) = Command::new("nvidia-smi").arg("--list-gpus").output() {
            if output.status.success() {
                let gpu_list = String::from_utf8_lossy(&output.stdout);
                for (i, line) in gpu_list.lines().enumerate() {
                    if line.contains("GPU") {
                        resources.push(ComputeResource {
                            name: format!("GPU {}", i),
                            resource_type: ResourceType::GPU {
                                model: "NVIDIA".to_string(),
                                vram_gb: 24, // Default estimate
                            },
                            cores: 0,
                            memory_gb: 0,
                            available: true,
                        });
                    }
                }
            }
        }
        
        // Catalog free LLMs
        resources.extend(vec![
            ComputeResource {
                name: "Ollama Local".to_string(),
                resource_type: ResourceType::LLM {
                    model: "llama3.2".to_string(),
                    context_size: 128000,
                },
                cores: 0,
                memory_gb: 8,
                available: true,
            },
            ComputeResource {
                name: "Gemini Flash".to_string(),
                resource_type: ResourceType::LLM {
                    model: "gemini-1.5-flash".to_string(),
                    context_size: 1000000,
                },
                cores: 0,
                memory_gb: 0,
                available: true,
            },
        ]);
        
        resources
    }
    
    pub fn profile_rust_bootstrap() -> RustBootstrapProfile {
        // This would run: rustc --self-profile=profile.json src/main.rs
        // And: perf record -g rustc src/main.rs
        
        RustBootstrapProfile {
            compilation_steps: vec![
                CompilationStep {
                    step_id: 0,
                    phase: "parse".to_string(),
                    duration_ms: 150,
                    memory_peak_mb: 256,
                    cpu_usage_percent: 85.0,
                },
                CompilationStep {
                    step_id: 1,
                    phase: "expand".to_string(),
                    duration_ms: 300,
                    memory_peak_mb: 512,
                    cpu_usage_percent: 90.0,
                },
                CompilationStep {
                    step_id: 2,
                    phase: "resolve".to_string(),
                    duration_ms: 200,
                    memory_peak_mb: 384,
                    cpu_usage_percent: 75.0,
                },
            ],
            perf_data: vec![
                PerfRecord {
                    timestamp: 1000,
                    event_type: "cpu-cycles".to_string(),
                    cpu_cycles: 1000000,
                    instructions: 800000,
                    cache_misses: 5000,
                },
            ],
            self_profile_data: vec![
                SelfProfileRecord {
                    query_name: "type_check_item".to_string(),
                    duration_ns: 50000000,
                    invocation_count: 1000,
                    self_time_ns: 45000000,
                },
            ],
            critical_path: vec![0, 1, 2], // All steps are critical
        }
    }
    
    pub fn extract_maximal_value(&self) -> f64 {
        let total_compute = self.available_resources.iter()
            .map(|r| match &r.resource_type {
                ResourceType::CPU => r.cores as f64 * 1.0,
                ResourceType::GPU { vram_gb, .. } => *vram_gb as f64 * 10.0,
                ResourceType::LLM { context_size, .. } => *context_size as f64 / 1000.0,
            })
            .sum::<f64>();
            
        let critical_path_cost: u64 = self.bootstrap_profile.critical_path.iter()
            .map(|&i| self.bootstrap_profile.compilation_steps[i].duration_ms)
            .sum();
            
        total_compute / (critical_path_cost as f64 / 1000.0)
    }
}
