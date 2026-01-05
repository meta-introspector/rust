use usage_eigenmatrix::{MorseHarmonic, harmonic_analyze};

// Apply Morse-harmonic analysis to core Rust types
#[derive(MorseHarmonic)]
struct MyOption<T> {
    value: Option<T>,
}

#[derive(MorseHarmonic)]
enum ControlFlow<B, C> {
    Continue(C),
    Break(B),
}

#[derive(MorseHarmonic)]
struct Iterator<T> {
    items: Vec<T>,
    position: usize,
}

fn main() {
    println!("🌊 Morse-Harmonic Macro Overlay Demo");
    println!("====================================");
    
    // Demonstrate automatic harmonic analysis
    println!("\n📊 Type Analysis:");
    println!("{}", MyOption::<i32> { value: None });
    println!("{}", ControlFlow::<i32, i32>::Continue(42));
    println!("{}", Iterator::<i32> { items: vec![], position: 0 });
    
    // Show harmonic resonance patterns
    println!("\n🎵 Harmonic Resonance:");
    println!("MyOption resonance: {}", MyOption::<()>::semantic_resonance());
    println!("ControlFlow resonance: {}", ControlFlow::<(), ()>::semantic_resonance());
    println!("Iterator resonance: {}", Iterator::<()>::semantic_resonance());
    
    // Demonstrate inline harmonic analysis
    println!("\n🔍 Inline Analysis:");
    let x = harmonic_analyze!(42 + 24);
    let y = harmonic_analyze!("hello world".to_string());
    
    println!("Result x: {}", x);
    println!("Result y: {}", y);
    
    // Show topological classification
    println!("\n🌐 Topological Classification:");
    println!("MyOption: {}", MyOption::<()>::topological_type());
    println!("ControlFlow: {}", ControlFlow::<(), ()>::topological_type());
    println!("Iterator: {}", Iterator::<()>::topological_type());
    
    // Show prime factorization of types
    println!("\n🔢 Prime Signatures:");
    println!("MyOption: {:?}", MyOption::<()>::prime_signature());
    println!("ControlFlow: {:?}", ControlFlow::<(), ()>::prime_signature());
    println!("Iterator: {:?}", Iterator::<()>::prime_signature());
    
    println!("\n🎉 Macro overlay successfully embedded Morse-harmonic analysis!");
}
