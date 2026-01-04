// # mklattice! - Generate lattice structures with join/meet operations

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mklattice() {
        mklattice!(TestLattice, Bottom, Middle, Top);
        
        let bottom = TestLattice::Bottom;
        let middle = TestLattice::Middle;
        let top = TestLattice::Top;
        
        // Test join (supremum)
        assert_eq!(bottom.join(&middle), TestLattice::Middle);
        assert_eq!(middle.join(&top), TestLattice::Top);
        assert_eq!(bottom.join(&top), TestLattice::Top);
        
        // Test meet (infimum)
        assert_eq!(middle.meet(&top), TestLattice::Middle);
        assert_eq!(bottom.meet(&middle), TestLattice::Bottom);
        assert_eq!(bottom.meet(&top), TestLattice::Bottom);
    }
}
