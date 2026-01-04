include!("../../mklattice_macro.rs");

fn main() {
    println!("🔬 MKLATTICE! MACRO DEMONSTRATION");
    
    // Generate a simple boolean lattice
    mklattice!(BoolLattice, False, True);
    
    // Generate a type lattice for Rust
    mklattice!(TypeLattice, Unit, Bool, Int, String, Generic);
    
    // Generate a feature lattice
    mklattice!(FeatureLattice, Core, Std, Alloc, Collections, Full);
    
    println!("✅ Generated lattice structures with mklattice! macro");
    
    // Test the lattices
    let bool_false = BoolLattice::False;
    let bool_true = BoolLattice::True;
    
    println!("Boolean lattice join: {:?}", bool_false.join(&bool_true));
    println!("Boolean lattice meet: {:?}", bool_false.meet(&bool_true));
}
