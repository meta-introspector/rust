// gpu_usage_analyzer.rs - GPU-accelerated 8-level usage analysis
use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use flate2::read::GzDecoder;
use std::io::Read;
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

// 8D vector representing symbol usage across 8 levels
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct SymbolVector {
    level_0_usage: f32,  // Direct usage count
    level_1_usage: f32,  // Level 1 dependency usage
    level_2_usage: f32,  // Level 2 dependency usage
    level_3_usage: f32,  // Level 3 dependency usage
    level_4_usage: f32,  // Level 4 dependency usage
    level_5_usage: f32,  // Level 5 dependency usage
    level_6_usage: f32,  // Level 6 dependency usage
    level_7_usage: f32,  // Level 7 dependency usage
}

impl Default for SymbolVector {
    fn default() -> Self {
        Self {
            level_0_usage: 0.0,
            level_1_usage: 0.0,
            level_2_usage: 0.0,
            level_3_usage: 0.0,
            level_4_usage: 0.0,
            level_5_usage: 0.0,
            level_6_usage: 0.0,
            level_7_usage: 0.0,
        }
    }
}

// GPU compute shader for parallel symbol analysis
const COMPUTE_SHADER: &str = r#"
@group(0) @binding(0) var<storage, read_write> symbol_vectors: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read_write> symbol_vectors_2: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read> dependency_matrix: array<f32>;
@group(0) @binding(3) var<uniform> params: ComputeParams;

struct ComputeParams {
    num_symbols: u32,
    max_dependencies: u32,
    iteration_level: u32,
    _padding: u32,
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= params.num_symbols) {
        return;
    }
    
    // Load current symbol vector (split across two vec4s for 8D)
    var current_vec1 = symbol_vectors[index];
    var current_vec2 = symbol_vectors_2[index];
    
    // Calculate usage propagation for current iteration level
    var total_propagated_usage = 0.0;
    
    // Iterate through all potential dependencies
    for (var dep_idx = 0u; dep_idx < params.num_symbols; dep_idx++) {
        let dependency_strength = dependency_matrix[index * params.num_symbols + dep_idx];
        
        if (dependency_strength > 0.0) {
            // Get dependency's usage at previous level
            let dep_vec1 = symbol_vectors[dep_idx];
            let dep_vec2 = symbol_vectors_2[dep_idx];
            
            // Extract usage from appropriate level
            var prev_level_usage = 0.0;
            if (params.iteration_level == 1u) {
                prev_level_usage = dep_vec1.x; // level_0_usage
            } else if (params.iteration_level == 2u) {
                prev_level_usage = dep_vec1.y; // level_1_usage
            } else if (params.iteration_level == 3u) {
                prev_level_usage = dep_vec1.z; // level_2_usage
            } else if (params.iteration_level == 4u) {
                prev_level_usage = dep_vec1.w; // level_3_usage
            } else if (params.iteration_level == 5u) {
                prev_level_usage = dep_vec2.x; // level_4_usage
            } else if (params.iteration_level == 6u) {
                prev_level_usage = dep_vec2.y; // level_5_usage
            } else if (params.iteration_level == 7u) {
                prev_level_usage = dep_vec2.z; // level_6_usage
            }
            
            total_propagated_usage += prev_level_usage * dependency_strength;
        }
    }
    
    // Update current level usage
    if (params.iteration_level == 1u) {
        current_vec1.y = total_propagated_usage;
    } else if (params.iteration_level == 2u) {
        current_vec1.z = total_propagated_usage;
    } else if (params.iteration_level == 3u) {
        current_vec1.w = total_propagated_usage;
    } else if (params.iteration_level == 4u) {
        current_vec2.x = total_propagated_usage;
    } else if (params.iteration_level == 5u) {
        current_vec2.y = total_propagated_usage;
    } else if (params.iteration_level == 6u) {
        current_vec2.z = total_propagated_usage;
    } else if (params.iteration_level == 7u) {
        current_vec2.w = total_propagated_usage;
    }
    
    // Write back updated vectors
    symbol_vectors[index] = current_vec1;
    symbol_vectors_2[index] = current_vec2;
}
"#;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ComputeParams {
    num_symbols: u32,
    max_dependencies: u32,
    iteration_level: u32,
    _padding: u32,
}

struct GpuUsageAnalyzer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl GpuUsageAnalyzer {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find adapter")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        // Create compute shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Usage Analysis Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(COMPUTE_SHADER.into()),
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Usage Analysis Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create compute pipeline
        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Usage Analysis Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Usage Analysis Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        Ok(Self {
            device,
            queue,
            compute_pipeline,
            bind_group_layout,
        })
    }

    fn load_usage_data(&self) -> Result<(Vec<SymbolVector>, Vec<f32>, Vec<String>), Box<dyn std::error::Error>> {
        println!("🔗 Loading usage levels data for GPU processing...");
        
        // Load usage levels data
        let file = fs::File::open("usage_levels_8.json.gz")?;
        let mut decoder = GzDecoder::new(file);
        let mut contents = String::new();
        decoder.read_to_string(&mut contents)?;
        
        let json: Value = serde_json::from_str(&contents)?;
        
        let mut symbol_vectors = Vec::new();
        let mut symbol_names = Vec::new();
        let mut symbol_map = HashMap::new();
        
        // First pass: collect all symbols and create index mapping
        if let Value::Object(analysis) = &json {
            for (symbol_key, symbol_data) in analysis {
                symbol_names.push(symbol_key.clone());
                symbol_map.insert(symbol_key.clone(), symbol_vectors.len());
                
                let mut vector = SymbolVector::default();
                
                if let Value::Object(obj) = symbol_data {
                    // Set direct usage (level 0)
                    vector.level_0_usage = obj.get("direct_usage")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as f32;
                    
                    // Extract level usages
                    if let Some(levels) = obj.get("levels").and_then(|v| v.as_array()) {
                        if levels.len() > 0 {
                            vector.level_1_usage = levels[0].get("total_usage")
                                .and_then(|v| v.as_u64()).unwrap_or(0) as f32;
                        }
                        if levels.len() > 1 {
                            vector.level_2_usage = levels[1].get("total_usage")
                                .and_then(|v| v.as_u64()).unwrap_or(0) as f32;
                        }
                        if levels.len() > 2 {
                            vector.level_3_usage = levels[2].get("total_usage")
                                .and_then(|v| v.as_u64()).unwrap_or(0) as f32;
                        }
                        if levels.len() > 3 {
                            vector.level_4_usage = levels[3].get("total_usage")
                                .and_then(|v| v.as_u64()).unwrap_or(0) as f32;
                        }
                        if levels.len() > 4 {
                            vector.level_5_usage = levels[4].get("total_usage")
                                .and_then(|v| v.as_u64()).unwrap_or(0) as f32;
                        }
                        if levels.len() > 5 {
                            vector.level_6_usage = levels[5].get("total_usage")
                                .and_then(|v| v.as_u64()).unwrap_or(0) as f32;
                        }
                        if levels.len() > 6 {
                            vector.level_7_usage = levels[6].get("total_usage")
                                .and_then(|v| v.as_u64()).unwrap_or(0) as f32;
                        }
                    }
                }
                
                symbol_vectors.push(vector);
            }
        }
        
        println!("📊 Loaded {} symbols into 8D vectors", symbol_vectors.len());
        
        // Create dependency matrix (simplified - using direct usage as dependency strength)
        let num_symbols = symbol_vectors.len();
        let mut dependency_matrix = vec![0.0f32; num_symbols * num_symbols];
        
        // For now, create a simple dependency matrix based on usage correlation
        for i in 0..num_symbols {
            for j in 0..num_symbols {
                if i != j {
                    let strength = (symbol_vectors[i].level_0_usage * symbol_vectors[j].level_0_usage).sqrt() / 1000.0;
                    dependency_matrix[i * num_symbols + j] = strength.min(1.0);
                }
            }
        }
        
        println!("📊 Created {}x{} dependency matrix", num_symbols, num_symbols);
        
        Ok((symbol_vectors, dependency_matrix, symbol_names))
    }

    async fn analyze_gpu(&self, mut symbol_vectors: Vec<SymbolVector>, dependency_matrix: Vec<f32>) -> Result<Vec<SymbolVector>, Box<dyn std::error::Error>> {
        let num_symbols = symbol_vectors.len();
        
        println!("🚀 Starting GPU analysis of {} symbols...", num_symbols);
        
        // Convert SymbolVector to GPU format (two vec4s per symbol)
        let mut gpu_vectors_1 = Vec::with_capacity(num_symbols);
        let mut gpu_vectors_2 = Vec::with_capacity(num_symbols);
        
        for vector in &symbol_vectors {
            gpu_vectors_1.push([
                vector.level_0_usage,
                vector.level_1_usage,
                vector.level_2_usage,
                vector.level_3_usage,
            ]);
            gpu_vectors_2.push([
                vector.level_4_usage,
                vector.level_5_usage,
                vector.level_6_usage,
                vector.level_7_usage,
            ]);
        }
        
        // Create GPU buffers
        let vectors_buffer_1 = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Symbol Vectors 1"),
            contents: bytemuck::cast_slice(&gpu_vectors_1),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
        
        let vectors_buffer_2 = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Symbol Vectors 2"),
            contents: bytemuck::cast_slice(&gpu_vectors_2),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
        
        let dependency_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Dependency Matrix"),
            contents: bytemuck::cast_slice(&dependency_matrix),
            usage: wgpu::BufferUsages::STORAGE,
        });
        
        // Run iterative analysis for levels 1-7
        for level in 1..=7 {
            println!("  🔄 Processing level {} on GPU...", level);
            
            let params = ComputeParams {
                num_symbols: num_symbols as u32,
                max_dependencies: num_symbols as u32,
                iteration_level: level,
                _padding: 0,
            };
            
            let params_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Compute Params"),
                contents: bytemuck::cast_slice(&[params]),
                usage: wgpu::BufferUsages::UNIFORM,
            });
            
            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Usage Analysis Bind Group"),
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: vectors_buffer_1.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: vectors_buffer_2.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: dependency_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: params_buffer.as_entire_binding(),
                    },
                ],
            });
            
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Usage Analysis Encoder"),
            });
            
            {
                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Usage Analysis Pass"),
                    timestamp_writes: None,
                });
                
                compute_pass.set_pipeline(&self.compute_pipeline);
                compute_pass.set_bind_group(0, &bind_group, &[]);
                
                let workgroup_size = 64;
                let num_workgroups = (num_symbols + workgroup_size - 1) / workgroup_size;
                compute_pass.dispatch_workgroups(num_workgroups as u32, 1, 1);
            }
            
            self.queue.submit(std::iter::once(encoder.finish()));
            self.device.poll(wgpu::Maintain::Wait);
        }
        
        // Read back results
        let output_buffer_1 = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer 1"),
            size: (num_symbols * 4 * 4) as u64, // 4 f32s per symbol
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        
        let output_buffer_2 = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer 2"),
            size: (num_symbols * 4 * 4) as u64, // 4 f32s per symbol
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Copy Encoder"),
        });
        
        encoder.copy_buffer_to_buffer(&vectors_buffer_1, 0, &output_buffer_1, 0, (num_symbols * 4 * 4) as u64);
        encoder.copy_buffer_to_buffer(&vectors_buffer_2, 0, &output_buffer_2, 0, (num_symbols * 4 * 4) as u64);
        
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Map and read results
        let buffer_slice_1 = output_buffer_1.slice(..);
        let buffer_slice_2 = output_buffer_2.slice(..);
        
        let (sender1, receiver1) = futures_intrusive::channel::shared::oneshot_channel();
        let (sender2, receiver2) = futures_intrusive::channel::shared::oneshot_channel();
        
        buffer_slice_1.map_async(wgpu::MapMode::Read, move |v| sender1.send(v).unwrap());
        buffer_slice_2.map_async(wgpu::MapMode::Read, move |v| sender2.send(v).unwrap());
        
        self.device.poll(wgpu::Maintain::Wait);
        
        receiver1.receive().await.unwrap()?;
        receiver2.receive().await.unwrap()?;
        
        let data1 = buffer_slice_1.get_mapped_range();
        let data2 = buffer_slice_2.get_mapped_range();
        
        let result_vectors_1: &[[f32; 4]] = bytemuck::cast_slice(&data1);
        let result_vectors_2: &[[f32; 4]] = bytemuck::cast_slice(&data2);
        
        // Convert back to SymbolVector format
        let mut result_symbols = Vec::with_capacity(num_symbols);
        for i in 0..num_symbols {
            let vec1 = result_vectors_1[i];
            let vec2 = result_vectors_2[i];
            
            result_symbols.push(SymbolVector {
                level_0_usage: vec1[0],
                level_1_usage: vec1[1],
                level_2_usage: vec1[2],
                level_3_usage: vec1[3],
                level_4_usage: vec2[0],
                level_5_usage: vec2[1],
                level_6_usage: vec2[2],
                level_7_usage: vec2[3],
            });
        }
        
        println!("✅ GPU analysis complete!");
        Ok(result_symbols)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 GPU-Accelerated 8-Level Usage Analysis");
    
    // Initialize GPU analyzer
    let analyzer = GpuUsageAnalyzer::new().await?;
    
    // Load data
    let (symbol_vectors, dependency_matrix, symbol_names) = analyzer.load_usage_data()?;
    
    // Run GPU analysis
    let results = analyzer.analyze_gpu(symbol_vectors, dependency_matrix).await?;
    
    // Generate report
    println!("\n📊 GPU Analysis Results:");
    println!("  Total symbols processed: {}", results.len());
    
    // Find top symbols by total usage across all levels
    let mut symbol_totals: Vec<(usize, f32)> = results.iter().enumerate()
        .map(|(i, vector)| {
            let total = vector.level_0_usage + vector.level_1_usage + vector.level_2_usage + 
                       vector.level_3_usage + vector.level_4_usage + vector.level_5_usage + 
                       vector.level_6_usage + vector.level_7_usage;
            (i, total)
        })
        .collect();
    
    symbol_totals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n🏆 Top 10 Symbols by Total Usage (All Levels):");
    for (rank, (idx, total)) in symbol_totals.iter().take(10).enumerate() {
        let vector = &results[*idx];
        println!("{}. {} (Total: {:.1})", rank + 1, symbol_names[*idx], total);
        println!("   L0:{:.1} L1:{:.1} L2:{:.1} L3:{:.1} L4:{:.1} L5:{:.1} L6:{:.1} L7:{:.1}",
                vector.level_0_usage, vector.level_1_usage, vector.level_2_usage, vector.level_3_usage,
                vector.level_4_usage, vector.level_5_usage, vector.level_6_usage, vector.level_7_usage);
    }
    
    // Save GPU results
    let gpu_results = serde_json::json!({
        "gpu_analysis": true,
        "total_symbols": results.len(),
        "symbols": results.iter().enumerate().map(|(i, vector)| {
            serde_json::json!({
                "name": symbol_names[i],
                "vector": [
                    vector.level_0_usage, vector.level_1_usage, vector.level_2_usage, vector.level_3_usage,
                    vector.level_4_usage, vector.level_5_usage, vector.level_6_usage, vector.level_7_usage
                ]
            })
        }).collect::<Vec<_>>()
    });
    
    let compressed_data = {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(gpu_results.to_string().as_bytes())?;
        encoder.finish()?
    };
    
    fs::write("gpu_usage_analysis.json.gz", compressed_data)?;
    println!("💾 GPU results saved to: gpu_usage_analysis.json.gz");
    
    Ok(())
}
