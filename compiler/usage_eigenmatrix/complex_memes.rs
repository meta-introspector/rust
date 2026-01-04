//! # Complex Memes: Real and Imaginary Parts of Every Meme
//! 
//! Every meme is a complex number: Meme = Real + i*Imaginary

use std::fmt;

/// Complex Meme structure with real and imaginary components
#[derive(Debug, Clone)]
struct ComplexMeme {
    real_part: f64,        // Observable, concrete meaning
    imaginary_part: f64,   // Hidden, potential meaning
    symbol: String,        // The meme representation
}

impl ComplexMeme {
    fn new(real: f64, imaginary: f64, symbol: &str) -> Self {
        Self {
            real_part: real,
            imaginary_part: imaginary,
            symbol: symbol.to_string(),
        }
    }
    
    /// Magnitude of the meme (total memetic power)
    fn magnitude(&self) -> f64 {
        (self.real_part.powi(2) + self.imaginary_part.powi(2)).sqrt()
    }
    
    /// Phase of the meme (angle in complex plane)
    fn phase(&self) -> f64 {
        self.imaginary_part.atan2(self.real_part)
    }
    
    /// Multiply two complex memes
    fn multiply(&self, other: &ComplexMeme) -> ComplexMeme {
        let real = self.real_part * other.real_part - self.imaginary_part * other.imaginary_part;
        let imag = self.real_part * other.imaginary_part + self.imaginary_part * other.real_part;
        ComplexMeme::new(real, imag, &format!("({} * {})", self.symbol, other.symbol))
    }
    
    /// Complex conjugate (flip imaginary part)
    fn conjugate(&self) -> ComplexMeme {
        ComplexMeme::new(self.real_part, -self.imaginary_part, &format!("{}*", self.symbol))
    }
}

impl fmt::Display for ComplexMeme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imaginary_part >= 0.0 {
            write!(f, "{} = {:.2} + {:.2}i", self.symbol, self.real_part, self.imaginary_part)
        } else {
            write!(f, "{} = {:.2} - {:.2}i", self.symbol, self.real_part, -self.imaginary_part)
        }
    }
}

/// SOLFUNMEME complex meme system
fn demonstrate_complex_solfunmeme() {
    println!("🎭 COMPLEX MEMES: Real + Imaginary Parts");
    println!("Every meme exists in the complex plane of meaning");
    
    let solfunmeme_complex_memes = [
        ComplexMeme::new(1.0, 0.0, "🌀"),     // Pure real: Lambda calculus (concrete)
        ComplexMeme::new(0.0, 1.0, "🎭"),     // Pure imaginary: Emoji (abstract)
        ComplexMeme::new(0.707, 0.707, "🧬"), // 45° phase: DNA (bio-digital)
        ComplexMeme::new(-1.0, 0.0, "🎨"),    // Negative real: Art (inverted)
        ComplexMeme::new(0.0, -1.0, "⛓️"),    // Negative imaginary: Chain (binding)
        ComplexMeme::new(0.618, 0.786, "🚀"), // Golden ratio phase: Rocket (growth)
        ComplexMeme::new(2.718, 0.0, "📜"),   // e: Natural (organic meaning)
        ComplexMeme::new(0.0, 3.14159, "🔍"), // πi: Pure transcendental search
        ComplexMeme::new(1.414, 1.414, "💬"), // √2 both: Perfect balance
        ComplexMeme::new(0.0, 0.0, "🧠"),     // Origin: Pure consciousness
    ];
    
    println!("\n📊 SOLFUNMEME COMPLEX MEME ANALYSIS:");
    for (i, meme) in solfunmeme_complex_memes.iter().enumerate() {
        println!("\n🔸 Meme {}: {}", i, meme);
        println!("   Magnitude: {:.3} (memetic power)", meme.magnitude());
        println!("   Phase: {:.3} radians ({:.1}°)", meme.phase(), meme.phase().to_degrees());
        
        // Interpret the complex structure
        let interpretation = match (meme.real_part > 0.0, meme.imaginary_part > 0.0) {
            (true, true) => "Constructive + Creative",
            (true, false) => "Constructive + Binding", 
            (false, true) => "Deconstructive + Creative",
            (false, false) => "Deconstructive + Binding",
        };
        println!("   Quadrant: {}", interpretation);
    }
}

/// Demonstrate complex meme operations
fn demonstrate_complex_meme_operations() {
    println!("\n🔄 COMPLEX MEME OPERATIONS:");
    
    let lambda = ComplexMeme::new(1.0, 0.0, "🌀");  // Real lambda
    let emoji = ComplexMeme::new(0.0, 1.0, "🎭");   // Imaginary emoji
    
    println!("Base memes:");
    println!("  {}", lambda);
    println!("  {}", emoji);
    
    // Multiplication creates new complex meaning
    let product = lambda.multiply(&emoji);
    println!("\nMultiplication (composition):");
    println!("  {} = {}", product.symbol, product);
    
    // Conjugation flips the imaginary (cultural inversion)
    let conjugate = emoji.conjugate();
    println!("\nConjugation (cultural flip):");
    println!("  {} = {}", conjugate.symbol, conjugate);
    
    // Complex meme with both parts
    let balanced = ComplexMeme::new(0.707, 0.707, "⚖️");
    println!("\nBalanced meme (45° phase):");
    println!("  {} = {}", balanced.symbol, balanced);
    println!("  Perfect balance of real and imaginary meaning");
}

/// The complex plane of meme consciousness
fn demonstrate_meme_consciousness_plane() {
    println!("\n🧠 MEME CONSCIOUSNESS COMPLEX PLANE:");
    println!("Real axis: Concrete, observable meaning");
    println!("Imaginary axis: Abstract, potential meaning");
    println!();
    println!("Quadrant I (+real, +imag): Constructive + Creative");
    println!("Quadrant II (-real, +imag): Deconstructive + Creative");  
    println!("Quadrant III (-real, -imag): Deconstructive + Binding");
    println!("Quadrant IV (+real, -imag): Constructive + Binding");
    println!();
    println!("🎯 SOLFUNMEME occupies ALL quadrants simultaneously!");
    println!("   Complete coverage of the complex meaning space");
}

fn main() {
    demonstrate_complex_solfunmeme();
    demonstrate_complex_meme_operations();
    demonstrate_meme_consciousness_plane();
    
    println!("\n{}", "=".repeat(60));
    println!("🎭 COMPLEX MEMES REVEALED:");
    println!("🔢 Every meme = Real part + i * Imaginary part");
    println!("🌐 Real: Observable, concrete meaning");
    println!("✨ Imaginary: Hidden, potential meaning");
    println!("🧠 SOLFUNMEME: Complete complex consciousness!");
    println!("{}", "=".repeat(60));
}
