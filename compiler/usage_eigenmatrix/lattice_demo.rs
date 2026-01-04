/// mklattice! - Generate complete lattice structures
macro_rules! mklattice {
    ($name:ident, $($element:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        enum $name {
            $($element),*
        }

        impl $name {
            fn order(&self) -> u8 {
                unsafe { *(self as *const Self as *const u8) }
            }
            
            fn join(&self, other: &Self) -> Self {
                if self.order() >= other.order() { self.clone() } else { other.clone() }
            }
            
            fn meet(&self, other: &Self) -> Self {
                if self.order() <= other.order() { self.clone() } else { other.clone() }
            }
        }
    };
}

fn main() {
    println!("🔬 MKLATTICE! MACRO DEMONSTRATION");
    
    // Generate lattices
    mklattice!(BoolLattice, False, True);
    mklattice!(TypeLattice, Unit, Bool, Int, String, Generic);
    mklattice!(FeatureLattice, Core, Std, Alloc, Collections, Full);
    
    println!("✅ Generated lattice structures with mklattice! macro");
    
    // Test boolean lattice
    let bool_false = BoolLattice::False;
    let bool_true = BoolLattice::True;
    
    println!("Boolean lattice operations:");
    println!("  False ∨ True = {:?}", bool_false.join(&bool_true));
    println!("  False ∧ True = {:?}", bool_false.meet(&bool_true));
    
    // Test type lattice
    let unit = TypeLattice::Unit;
    let generic = TypeLattice::Generic;
    
    println!("Type lattice operations:");
    println!("  Unit ∨ Generic = {:?}", unit.join(&generic));
    println!("  Unit ∧ Generic = {:?}", unit.meet(&generic));
    
    // Test feature lattice
    let core = FeatureLattice::Core;
    let full = FeatureLattice::Full;
    
    println!("Feature lattice operations:");
    println!("  Core ∨ Full = {:?}", core.join(&full));
    println!("  Core ∧ Full = {:?}", core.meet(&full));
}
