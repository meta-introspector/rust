# Regex Language Embedding Proof

## Theorem: Regexes are Embedded Languages in Rust

**Proof by Construction:**

1. **Full rustc** has N features for complete language
2. **rustcregex** has subset of features for regex constructs only
3. **Embedding**: rustcregex ⊆ rustc
4. **Completeness**: rustcregex can compile any regex pattern

### Regex Feature Set

| Feature | Pattern | Functions | Type |
|---------|---------|-----------|------|
| CharClass | `\w` | 3 | character |
| DigitClass | `\d` | 3 | digit |
| ZeroOrMore | `*` | 3 | unknown |
| OneOrMore | `+` | 3 | unknown |
| Optional | `?` | 3 | unknown |
| StartAnchor | `^` | 3 | unknown |
| EndAnchor | `$` | 3 | unknown |
| Group | `()` | 4 | unknown |
| Alternation | `|` | 3 | unknown |

### Grammar Reduction

**Full Rust Grammar**: ~200 productions
**Regex Grammar**: 8 productions

1. Regex ::= Alternation
2. Alternation ::= Sequence ('|' Sequence)*
3. Sequence ::= Atom Quantifier*
4. Atom ::= Char | CharClass | Group | Anchor
5. Quantifier ::= '*' | '+' | '?' | '{n,m}'
6. CharClass ::= '\w' | '\d' | '\s' | '[...]'
7. Group ::= '(' Regex ')'
8. Anchor ::= '^' | '$'

### Automorphism Preservation

Regex automorphisms preserved in reduction:

- a* ≡ (a)*
- a+ ≡ aa*
- a? ≡ (a|ε)
- (a|b) ≡ (b|a)
- a** ≡ a*
- \w ≡ [a-zA-Z0-9_]

### Applications

- **Domain-specific compilers**: Extract language subsets
- **Embedded DSLs**: Regex, SQL, HTML within Rust
- **Language composition**: Combine multiple embedded languages
- **Compiler optimization**: Reduce to minimal feature set
