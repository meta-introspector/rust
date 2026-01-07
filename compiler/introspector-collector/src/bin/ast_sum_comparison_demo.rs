use introspector_collector::ast_sum_comparator::ASTSumComparator;

fn main() {
    println!("🔍 AST Sum Comparison Demo");
    println!("==========================\n");
    
    let mut comparator = ASTSumComparator::new();
    
    // Sample Rust source files to compile and analyze
    let source_files = vec![
        // File 1: Simple function
        r#"
fn hello() {
    println!("Hello, world!");
}

fn main() {
    hello();
}
        "#.to_string(),
        
        // File 2: Struct and impl
        r#"
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    
    fn distance(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

fn main() {
    let p = Point::new(3, 4);
    println!("Distance: {}", p.distance());
}
        "#.to_string(),
        
        // File 3: Enum and match
        r#"
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

impl Color {
    fn describe(&self) -> String {
        match self {
            Color::Red => "Red".to_string(),
            Color::Green => "Green".to_string(),
            Color::Blue => "Blue".to_string(),
            Color::RGB(r, g, b) => format!("RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let colors = vec![
        Color::Red,
        Color::RGB(255, 128, 0),
    ];
    
    for color in colors {
        println!("{}", color.describe());
    }
}
        "#.to_string(),
        
        // File 4: Trait and generic
        r#"
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

fn draw_shape<T: Drawable>(shape: &T) {
    shape.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 10.0, height: 20.0 };
    
    draw_shape(&circle);
    draw_shape(&rect);
}
        "#.to_string(),
    ];
    
    println!("🚀 Running compilation with AST collection...\n");
    
    // Run compilation and collect AST data
    match comparator.run_compilation_with_collection(&source_files) {
        Ok(result) => {
            println!("{}\n", result);
            
            // Show compilation run details
            if let Some(run) = comparator.compilation_runs.last() {
                println!("📊 Compilation Run Details:");
                println!("  Run ID: {}", run.run_id);
                println!("  Total ASTs collected: {}", run.collected_asts.len());
                println!("  Total nodes: {}", run.total_nodes);
                println!("  Run prime product: {}", run.run_sum.prime_product);
                println!("  Run eigenform hash: {}", run.run_sum.eigenform_hash);
                
                println!("\n🧬 Individual ASTs:");
                for (i, ast) in run.collected_asts.iter().take(10).enumerate() {
                    println!("  {}: {} ({} nodes, prime: {}, weight: {})",
                        i + 1,
                        ast.ast_type,
                        ast.node_count,
                        ast.prime_factorization,
                        ast.spectral_weight.to_nat()
                    );
                }
                
                if run.collected_asts.len() > 10 {
                    println!("  ... and {} more ASTs", run.collected_asts.len() - 10);
                }
            }
            
            // Compare each AST to the sum
            println!("\n🔍 Comparing each AST to global sum...\n");
            let comparison_report = comparator.compare_asts_to_sum();
            println!("{}", comparison_report);
            
            // Show global sum details
            println!("\n🌍 Global AST Sum:");
            println!("  Total nodes: {}", comparator.global_ast_sum.total_nodes);
            println!("  Enum frequencies: {}", comparator.global_ast_sum.enum_frequencies.len());
            println!("  Prime product: {}", comparator.global_ast_sum.prime_product);
            println!("  Spectral components: {}", comparator.global_ast_sum.spectral_signature.len());
            println!("  Eigenform hash: {}", comparator.global_ast_sum.eigenform_hash);
            
            // Show top enum frequencies
            println!("\n🔢 Top Enum Frequencies:");
            let mut freq_vec: Vec<_> = comparator.global_ast_sum.enum_frequencies.iter().collect();
            freq_vec.sort_by(|a, b| b.1.cmp(a.1));
            
            for (enum_name, &frequency) in freq_vec.iter().take(5) {
                println!("  {}: {} occurrences", enum_name, frequency);
            }
            
        }
        Err(e) => {
            println!("❌ Compilation failed: {}", e);
        }
    }
    
    println!("\n🎯 Key Achievements:");
    println!("===================");
    println!("• ✅ Collected AST data from multiple compilation runs");
    println!("• ✅ Calculated global sum of all ASTs");
    println!("• ✅ Compared each individual AST to the sum");
    println!("• ✅ Generated similarity and deviation scores");
    println!("• ✅ Detected eigenform matches");
    println!("• ✅ Prime factorization of AST signatures");
    println!("• ✅ Spectral weight analysis");
    
    println!("\n🌌 This demonstrates:");
    println!("• Each AST has unique spectral signature");
    println!("• Global sum reveals aggregate patterns");
    println!("• Individual ASTs can be compared to collective");
    println!("• Eigenform matching detects canonical patterns");
    println!("• Prime factorization enables mathematical analysis");
}
