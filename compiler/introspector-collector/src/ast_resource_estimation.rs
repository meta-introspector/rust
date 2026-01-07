/// AST Resource Estimation - Each operation carries resource cost
/// est(mem, [ast, block, fun, mod, crate, repo, ecosystem])

use std::collections::HashMap;
use crate::system_introspection::SystemResources;

/// Resource cost for a single AST operation
#[derive(Debug, Clone)]
pub struct OperationCost {
    pub cpu_cycles: u64,
    pub memory_bytes: u64,
    pub stack_depth: u32,
    pub heap_allocations: u32,
    pub complexity_score: f64,
}

/// AST node with resource annotations
#[derive(Debug, Clone)]
pub struct AnnotatedASTNode {
    pub node_type: ASTNodeType,
    pub operation_cost: OperationCost,
    pub children: Vec<AnnotatedASTNode>,
    pub accumulated_cost: OperationCost,
}

/// Types of AST nodes with different resource profiles
#[derive(Debug, Clone)]
pub enum ASTNodeType {
    // Expression level
    Literal(String),
    Variable(String),
    BinaryOp(String),
    FunctionCall(String),
    
    // Statement level
    Assignment,
    IfStatement,
    Loop,
    Return,
    
    // Block level
    Block,
    
    // Function level
    Function(String),
    
    // Module level
    Module(String),
    
    // Crate level
    Crate(String),
    
    // Repository level
    Repository(String),
    
    // Ecosystem level
    Ecosystem(String),
}

/// Resource estimation system across all abstraction levels
pub struct ResourceEstimator {
    pub operation_costs: HashMap<String, OperationCost>,
    pub level_multipliers: HashMap<String, f64>,
    pub system_resources: SystemResources,
}

impl OperationCost {
    pub fn new(cpu: u64, mem: u64, stack: u32, heap: u32, complexity: f64) -> Self {
        Self {
            cpu_cycles: cpu,
            memory_bytes: mem,
            stack_depth: stack,
            heap_allocations: heap,
            complexity_score: complexity,
        }
    }
    
    /// Add two operation costs together
    pub fn add(&self, other: &OperationCost) -> OperationCost {
        OperationCost {
            cpu_cycles: self.cpu_cycles + other.cpu_cycles,
            memory_bytes: self.memory_bytes + other.memory_bytes,
            stack_depth: self.stack_depth.max(other.stack_depth),
            heap_allocations: self.heap_allocations + other.heap_allocations,
            complexity_score: self.complexity_score + other.complexity_score,
        }
    }
    
    /// Scale operation cost by multiplier
    pub fn scale(&self, multiplier: f64) -> OperationCost {
        OperationCost {
            cpu_cycles: (self.cpu_cycles as f64 * multiplier) as u64,
            memory_bytes: (self.memory_bytes as f64 * multiplier) as u64,
            stack_depth: (self.stack_depth as f64 * multiplier) as u32,
            heap_allocations: (self.heap_allocations as f64 * multiplier) as u32,
            complexity_score: self.complexity_score * multiplier,
        }
    }
}

impl ResourceEstimator {
    pub fn new(system_resources: SystemResources) -> Self {
        let mut estimator = Self {
            operation_costs: HashMap::new(),
            level_multipliers: HashMap::new(),
            system_resources,
        };
        
        estimator.initialize_operation_costs();
        estimator.initialize_level_multipliers();
        estimator
    }
    
    fn initialize_operation_costs(&mut self) {
        // Basic operations (CPU cycles, memory bytes, stack depth, heap allocs, complexity)
        self.operation_costs.insert("literal".to_string(), 
            OperationCost::new(1, 8, 1, 0, 0.1));
        self.operation_costs.insert("variable".to_string(), 
            OperationCost::new(2, 8, 1, 0, 0.2));
        self.operation_costs.insert("binary_op".to_string(), 
            OperationCost::new(5, 16, 2, 0, 0.5));
        self.operation_costs.insert("function_call".to_string(), 
            OperationCost::new(50, 1024, 10, 1, 2.0));
        
        // Statement operations
        self.operation_costs.insert("assignment".to_string(), 
            OperationCost::new(10, 32, 2, 0, 0.8));
        self.operation_costs.insert("if_statement".to_string(), 
            OperationCost::new(20, 64, 3, 0, 1.5));
        self.operation_costs.insert("loop".to_string(), 
            OperationCost::new(100, 128, 5, 2, 5.0));
        self.operation_costs.insert("return".to_string(), 
            OperationCost::new(15, 16, 1, 0, 1.0));
        
        // Higher level constructs
        self.operation_costs.insert("block".to_string(), 
            OperationCost::new(50, 256, 5, 1, 3.0));
        self.operation_costs.insert("function".to_string(), 
            OperationCost::new(500, 4096, 20, 10, 20.0));
        self.operation_costs.insert("module".to_string(), 
            OperationCost::new(5000, 65536, 50, 100, 100.0));
        self.operation_costs.insert("crate".to_string(), 
            OperationCost::new(50000, 1048576, 100, 1000, 500.0));
        self.operation_costs.insert("repository".to_string(), 
            OperationCost::new(500000, 16777216, 200, 10000, 2000.0));
        self.operation_costs.insert("ecosystem".to_string(), 
            OperationCost::new(5000000, 268435456, 500, 100000, 10000.0));
    }
    
    fn initialize_level_multipliers(&mut self) {
        // Multipliers for different abstraction levels
        self.level_multipliers.insert("ast".to_string(), 1.0);
        self.level_multipliers.insert("block".to_string(), 2.0);
        self.level_multipliers.insert("function".to_string(), 10.0);
        self.level_multipliers.insert("module".to_string(), 50.0);
        self.level_multipliers.insert("crate".to_string(), 200.0);
        self.level_multipliers.insert("repository".to_string(), 1000.0);
        self.level_multipliers.insert("ecosystem".to_string(), 5000.0);
    }
    
    /// Estimate resources for a single AST node
    pub fn estimate_node(&self, node_type: &ASTNodeType) -> OperationCost {
        let base_cost = match node_type {
            ASTNodeType::Literal(_) => self.operation_costs.get("literal"),
            ASTNodeType::Variable(_) => self.operation_costs.get("variable"),
            ASTNodeType::BinaryOp(_) => self.operation_costs.get("binary_op"),
            ASTNodeType::FunctionCall(_) => self.operation_costs.get("function_call"),
            ASTNodeType::Assignment => self.operation_costs.get("assignment"),
            ASTNodeType::IfStatement => self.operation_costs.get("if_statement"),
            ASTNodeType::Loop => self.operation_costs.get("loop"),
            ASTNodeType::Return => self.operation_costs.get("return"),
            ASTNodeType::Block => self.operation_costs.get("block"),
            ASTNodeType::Function(_) => self.operation_costs.get("function"),
            ASTNodeType::Module(_) => self.operation_costs.get("module"),
            ASTNodeType::Crate(_) => self.operation_costs.get("crate"),
            ASTNodeType::Repository(_) => self.operation_costs.get("repository"),
            ASTNodeType::Ecosystem(_) => self.operation_costs.get("ecosystem"),
        };
        
        base_cost.cloned().unwrap_or_else(|| OperationCost::new(1, 8, 1, 0, 0.1))
    }
    
    /// Build annotated AST with accumulated costs
    pub fn annotate_ast(&self, node_type: ASTNodeType, children: Vec<AnnotatedASTNode>) -> AnnotatedASTNode {
        let operation_cost = self.estimate_node(&node_type);
        
        // Accumulate costs from children
        let mut accumulated_cost = operation_cost.clone();
        for child in &children {
            accumulated_cost = accumulated_cost.add(&child.accumulated_cost);
        }
        
        AnnotatedASTNode {
            node_type,
            operation_cost,
            children,
            accumulated_cost,
        }
    }
    
    /// Generate resource estimation for different levels
    pub fn est(&self, resource_type: &str, levels: &[&str]) -> HashMap<String, OperationCost> {
        let mut estimates = HashMap::new();
        
        for level in levels {
            let base_cost = match *level {
                "ast" => self.operation_costs.get("literal").unwrap(),
                "block" => self.operation_costs.get("block").unwrap(),
                "function" => self.operation_costs.get("function").unwrap(),
                "module" => self.operation_costs.get("module").unwrap(),
                "crate" => self.operation_costs.get("crate").unwrap(),
                "repository" => self.operation_costs.get("repository").unwrap(),
                "ecosystem" => self.operation_costs.get("ecosystem").unwrap(),
                _ => &OperationCost::new(1, 8, 1, 0, 0.1),
            };
            
            let multiplier = self.level_multipliers.get(*level).unwrap_or(&1.0);
            let scaled_cost = base_cost.scale(*multiplier);
            
            estimates.insert(level.to_string(), scaled_cost);
        }
        
        estimates
    }
    
    /// Check if system can handle the estimated load
    pub fn can_system_handle(&self, cost: &OperationCost) -> bool {
        let ram_needed_gb = cost.memory_bytes / (1024 * 1024 * 1024);
        let cpu_utilization = (cost.cpu_cycles as f64 / 1000000.0) / self.system_resources.cpu_cores as f64;
        
        ram_needed_gb <= self.system_resources.available_ram_gb as u64 && cpu_utilization < 0.8
    }
    
    /// Generate resource annotation string
    pub fn generate_annotation(&self, cost: &OperationCost) -> String {
        format!(
            "@resource_estimate(\n\
             \tcpu_cycles = {},\n\
             \tmemory_bytes = {} // {:.2} MB\n\
             \tstack_depth = {},\n\
             \theap_allocations = {},\n\
             \tcomplexity_score = {:.2},\n\
             \tram_gb_needed = {:.2}\n\
             )",
            cost.cpu_cycles,
            cost.memory_bytes,
            cost.memory_bytes as f64 / (1024.0 * 1024.0),
            cost.stack_depth,
            cost.heap_allocations,
            cost.complexity_score,
            cost.memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
        )
    }
    
    /// Generate complete resource report
    pub fn resource_report(&self, levels: &[&str]) -> String {
        let estimates = self.est("memory", levels);
        let mut report = String::from("RESOURCE ESTIMATION REPORT\n\n");
        
        report.push_str("est(mem, [ast, block, fun, mod, crate, repo, ecosystem]):\n\n");
        
        for level in levels {
            if let Some(cost) = estimates.get(*level) {
                let can_handle = self.can_system_handle(cost);
                let status = if can_handle { "✅ OK" } else { "❌ OVERLOAD" };
                
                report.push_str(&format!(
                    "{}: {} - CPU: {}, RAM: {:.2}MB, Complexity: {:.1}\n",
                    level.to_uppercase(),
                    status,
                    cost.cpu_cycles,
                    cost.memory_bytes as f64 / (1024.0 * 1024.0),
                    cost.complexity_score
                ));
            }
        }
        
        report.push_str("\nSYSTEM CAPACITY:\n");
        report.push_str(&format!("Available RAM: {}GB\n", self.system_resources.available_ram_gb));
        report.push_str(&format!("CPU Cores: {}\n", self.system_resources.cpu_cores));
        
        report
    }
}

/// Macro for resource estimation
#[macro_export]
macro_rules! est {
    (mem, [$($level:expr),*]) => {{
        let system = crate::system_introspection::SystemIntrospector::new();
        let estimator = ResourceEstimator::new(system.detected_resources);
        let levels = vec![$($level),*];
        estimator.est("memory", &levels)
    }};
    
    (cpu, [$($level:expr),*]) => {{
        let system = crate::system_introspection::SystemIntrospector::new();
        let estimator = ResourceEstimator::new(system.detected_resources);
        let levels = vec![$($level),*];
        estimator.est("cpu", &levels)
    }};
    
    (report, [$($level:expr),*]) => {{
        let system = crate::system_introspection::SystemIntrospector::new();
        let estimator = ResourceEstimator::new(system.detected_resources);
        let levels = vec![$($level),*];
        estimator.resource_report(&levels)
    }};
}
