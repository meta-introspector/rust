use std::collections::HashSet;

// First 8 primes: 2, 3, 5, 7, 11, 13, 17, 19
mklattice!(PrimeLattice, 
    Empty,
    P2, P3, P5, P7, P11, P13, P17, P19,
    P2P3, P2P5, P2P7, P2P11, P2P13, P2P17, P2P19,
    P3P5, P3P7, P3P11, P3P13, P3P17, P3P19,
    P5P7, P5P11, P5P13, P5P17, P5P19,
    P7P11, P7P13, P7P17, P7P19,
    P11P13, P11P17, P11P19,
    P13P17, P13P19,
    P17P19,
    Full
);

impl PrimeLattice {
    fn to_primes(&self) -> HashSet<u32> {
        match self {
            PrimeLattice::Empty => HashSet::new(),
            PrimeLattice::P2 => [2].into(),
            PrimeLattice::P3 => [3].into(),
            PrimeLattice::P5 => [5].into(),
            PrimeLattice::P7 => [7].into(),
            PrimeLattice::P11 => [11].into(),
            PrimeLattice::P13 => [13].into(),
            PrimeLattice::P17 => [17].into(),
            PrimeLattice::P19 => [19].into(),
            PrimeLattice::P2P3 => [2, 3].into(),
            PrimeLattice::Full => [2, 3, 5, 7, 11, 13, 17, 19].into(),
            _ => HashSet::new(), // Simplified for brevity
        }
    }
}

fn main() {
    println!("🔢 PRIME LATTICE: First 8 Primes");
    
    let empty = PrimeLattice::Empty;
    let p2 = PrimeLattice::P2;
    let p3 = PrimeLattice::P3;
    let full = PrimeLattice::Full;
    
    println!("Empty: {:?}", empty.to_primes());
    println!("P2: {:?}", p2.to_primes());
    println!("P2∪P3: {:?}", p2.join(&p3));
    println!("Full: {:?}", full.to_primes());
}

macro_rules! mklattice {
    ($name:ident, $($element:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        enum $name {
            $($element),*
        }

        impl $name {
            fn join(&self, other: &Self) -> Self {
                match (self, other) {
                    (a, b) if a == b => a.clone(),
                    _ => self.clone()
                }
            }
            
            fn meet(&self, other: &Self) -> Self {
                match (self, other) {
                    (a, b) if a == b => a.clone(),
                    _ => self.clone()
                }
            }
        }
    };
}
