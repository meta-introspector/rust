/// System Resource Introspection - Check real system vs massive lifeform requirements
/// Uses free -g, /proc/cpuinfo, nvidia-smi to validate resource availability

use std::process::Command;
use std::fs;
use crate::massive_game_of_life::ResourceRequirements;

/// Real system resources detected from the actual system
#[derive(Debug, Clone)]
pub struct SystemResources {
    pub total_ram_gb: u32,
    pub available_ram_gb: u32,
    pub cpu_cores: u32,
    pub cpu_model: String,
    pub gpu_ram_gb: u32,
    pub gpu_model: String,
    pub disk_available_gb: u32,
}

/// System resource checker
pub struct SystemIntrospector {
    pub detected_resources: SystemResources,
    pub can_run_lifeforms: Vec<(u8, bool)>, // (bit_complexity, can_run)
}

impl SystemIntrospector {
    pub fn new() -> Self {
        let detected_resources = Self::detect_system_resources();
        let can_run_lifeforms = Self::check_lifeform_compatibility(&detected_resources);
        
        Self {
            detected_resources,
            can_run_lifeforms,
        }
    }
    
    /// Detect actual system resources using system commands
    fn detect_system_resources() -> SystemResources {
        let total_ram_gb = Self::get_total_ram_gb();
        let available_ram_gb = Self::get_available_ram_gb();
        let cpu_cores = Self::get_cpu_cores();
        let cpu_model = Self::get_cpu_model();
        let (gpu_ram_gb, gpu_model) = Self::get_gpu_info();
        let disk_available_gb = Self::get_disk_space_gb();
        
        SystemResources {
            total_ram_gb,
            available_ram_gb,
            cpu_cores,
            cpu_model,
            gpu_ram_gb,
            gpu_model,
            disk_available_gb,
        }
    }
    
    /// Get total RAM using `free -g`
    fn get_total_ram_gb() -> u32 {
        if let Ok(output) = Command::new("free").arg("-g").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("Mem:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        return parts[1].parse().unwrap_or(0);
                    }
                }
            }
        }
        0
    }
    
    /// Get available RAM using `free -g`
    fn get_available_ram_gb() -> u32 {
        if let Ok(output) = Command::new("free").arg("-g").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("Mem:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 6 {
                        return parts[6].parse().unwrap_or(0);
                    }
                }
            }
        }
        0
    }
    
    /// Get CPU cores from /proc/cpuinfo
    fn get_cpu_cores() -> u32 {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            let processor_count = cpuinfo.lines()
                .filter(|line| line.starts_with("processor"))
                .count();
            return processor_count as u32;
        }
        
        // Fallback to nproc
        if let Ok(output) = Command::new("nproc").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.trim().parse().unwrap_or(1);
        }
        
        1
    }
    
    /// Get CPU model from /proc/cpuinfo
    fn get_cpu_model() -> String {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            for line in cpuinfo.lines() {
                if line.starts_with("model name") {
                    if let Some(model) = line.split(':').nth(1) {
                        return model.trim().to_string();
                    }
                }
            }
        }
        "Unknown CPU".to_string()
    }
    
    /// Get GPU info using nvidia-smi
    fn get_gpu_info() -> (u32, String) {
        if let Ok(output) = Command::new("nvidia-smi")
            .arg("--query-gpu=memory.total,name")
            .arg("--format=csv,noheader,nounits")
            .output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = output_str.lines().next() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let ram_mb: u32 = parts[0].trim().parse().unwrap_or(0);
                    let ram_gb = ram_mb / 1024;
                    let model = parts[1].trim().to_string();
                    return (ram_gb, model);
                }
            }
        }
        (0, "No NVIDIA GPU detected".to_string())
    }
    
    /// Get available disk space using df
    fn get_disk_space_gb() -> u32 {
        if let Ok(output) = Command::new("df").arg("-BG").arg("/").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 3 {
                    let available = parts[3].trim_end_matches('G');
                    return available.parse().unwrap_or(0);
                }
            }
        }
        0
    }
    
    /// Check which lifeform complexities can run on this system
    fn check_lifeform_compatibility(system: &SystemResources) -> Vec<(u8, bool)> {
        let mut compatibility = vec![];
        
        for bit_complexity in 3..=10 {
            let requirements = ResourceRequirements::scaled_by_complexity(bit_complexity);
            
            let can_run = system.available_ram_gb >= requirements.ram_gb &&
                         system.cpu_cores >= requirements.cpu_cores &&
                         system.gpu_ram_gb >= requirements.gpu_ram_gb &&
                         system.disk_available_gb >= requirements.disk_gb;
            
            compatibility.push((bit_complexity, can_run));
        }
        
        compatibility
    }
    
    /// Generate system compatibility report
    pub fn compatibility_report(&self) -> String {
        let mut report = String::from("🖥️  SYSTEM COMPATIBILITY REPORT\n\n");
        
        report.push_str("DETECTED SYSTEM RESOURCES:\n");
        report.push_str(&format!("💾 RAM: {}GB total, {}GB available\n", 
            self.detected_resources.total_ram_gb, 
            self.detected_resources.available_ram_gb));
        report.push_str(&format!("🔥 CPU: {} cores ({})\n", 
            self.detected_resources.cpu_cores, 
            self.detected_resources.cpu_model));
        report.push_str(&format!("🎮 GPU: {}GB ({})\n", 
            self.detected_resources.gpu_ram_gb, 
            self.detected_resources.gpu_model));
        report.push_str(&format!("💿 Disk: {}GB available\n", 
            self.detected_resources.disk_available_gb));
        
        report.push_str("\nLIFEFORM COMPATIBILITY:\n");
        for (bit_complexity, can_run) in &self.can_run_lifeforms {
            let requirements = ResourceRequirements::scaled_by_complexity(*bit_complexity);
            let status = if *can_run { "✅ CAN RUN" } else { "❌ INSUFFICIENT" };
            
            report.push_str(&format!(
                "{}-bit lifeform: {} (needs {}GB RAM, {} CPUs, {}GB GPU)\n",
                bit_complexity, status, requirements.ram_gb, 
                requirements.cpu_cores, requirements.gpu_ram_gb
            ));
        }
        
        // Recommendations
        report.push_str("\nRECOMMENDATIONS:\n");
        let max_runnable = self.can_run_lifeforms.iter()
            .filter(|(_, can_run)| *can_run)
            .map(|(bits, _)| *bits)
            .max();
        
        match max_runnable {
            Some(max_bits) => {
                report.push_str(&format!("🎯 Maximum lifeform complexity: {}-bit\n", max_bits));
                report.push_str(&format!("💡 You can run up to {}-state lifeforms\n", 1 << max_bits));
            },
            None => {
                report.push_str("⚠️  WARNING: System cannot run any massive lifeforms!\n");
                report.push_str("💡 Consider upgrading to datacenter-class hardware\n");
            }
        }
        
        report
    }
    
    /// Check if system can run a specific lifeform
    pub fn can_run_lifeform(&self, bit_complexity: u8) -> bool {
        self.can_run_lifeforms.iter()
            .find(|(bits, _)| *bits == bit_complexity)
            .map(|(_, can_run)| *can_run)
            .unwrap_or(false)
    }
    
    /// Get resource utilization percentage
    pub fn resource_utilization(&self, bit_complexity: u8) -> (f32, f32, f32) {
        let requirements = ResourceRequirements::scaled_by_complexity(bit_complexity);
        
        let ram_util = (requirements.ram_gb as f32 / self.detected_resources.available_ram_gb as f32) * 100.0;
        let cpu_util = (requirements.cpu_cores as f32 / self.detected_resources.cpu_cores as f32) * 100.0;
        let gpu_util = if self.detected_resources.gpu_ram_gb > 0 {
            (requirements.gpu_ram_gb as f32 / self.detected_resources.gpu_ram_gb as f32) * 100.0
        } else {
            100.0
        };
        
        (ram_util, cpu_util, gpu_util)
    }
    
    /// Generate system monitoring commands
    pub fn monitoring_commands(&self) -> Vec<String> {
        vec![
            "free -h".to_string(),
            "nproc".to_string(),
            "cat /proc/cpuinfo | grep 'model name' | head -1".to_string(),
            "nvidia-smi".to_string(),
            "df -h /".to_string(),
            "htop".to_string(),
            "watch -n 1 'free -h && echo && nvidia-smi'".to_string(),
        ]
    }
}

/// Macro for system introspection
#[macro_export]
macro_rules! check_system {
    () => {{
        SystemIntrospector::new()
    }};
    
    (can_run, $bits:expr) => {{
        let introspector = SystemIntrospector::new();
        introspector.can_run_lifeform($bits)
    }};
    
    (utilization, $bits:expr) => {{
        let introspector = SystemIntrospector::new();
        introspector.resource_utilization($bits)
    }};
}
