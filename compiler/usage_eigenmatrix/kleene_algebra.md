# Kleene Algebra Operations

## Lattice Operations

- **Join (∨)**: L₁ ∨ L₂ = L₁ ∪ L₂ (union of languages)
- **Meet (∧)**: L₁ ∧ L₂ = L₁ ∩ L₂ (intersection of languages)
- **Complement**: ¬L = Σ* \ L (complement language)
- **Kleene Star**: L* = ε ∪ L ∪ L² ∪ L³ ∪ ...

## Lattice Properties

1. **Partial Order**: L₁ ⊆ L₂ iff L₁ ∨ L₂ = L₂
2. **Bottom Element**: ∅ (empty language)
3. **Top Element**: Σ* (all strings)
4. **Distributivity**: L₁ ∧ (L₂ ∨ L₃) = (L₁ ∧ L₂) ∨ (L₁ ∧ L₃)

## Feature Composition Rules

**Level 1**: L, .
**Level 2**: [L], \d, \w
**Level 3**: L?, L*, L+, [L]*, \w+
**Level 4**: ^L, L$, ^L$
**Level 5**: (L), L|L, (L|L)
**Level 6**: (L|L)*, L\1, L(?=L)
