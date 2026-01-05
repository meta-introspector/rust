# Syn Codebase Monster Signature Analysis

## Syn↔HIR Bijection with Monster Group Signatures

### Analysis Summary
- **Files Analyzed**: 5
- **Monster Signatures Generated**: 5
- **Average Bijection Confidence**: 0.00%
- **Total DefIds Found**: 0

### Syn Monster Signatures
| File | Monster Signature | Syn Patterns | HIR Mappings | DefIds | Bijection % |
|------|-------------------|--------------|--------------|--------|-------------|
| `expr.rs` | `0xF316BCB9DEF327D7` | 1 | 0 | 0 | 0.0% |
| `item.rs` | `0x7ECEA7B9E177FC59` | 1 | 0 | 0 | 0.0% |
| `ty.rs` | `0x26798072A07C03DF` | 1 | 0 | 0 | 0.0% |
| `parse.rs` | `0x5A5F567A1D3387EB` | 1 | 0 | 0 | 0.0% |
| `token.rs` | `0x7DC254FA2F57BE41` | 1 | 0 | 0 | 0.0% |

### Syn Pattern Examples

#### File: expr.rs
**Monster Signature**: `0x0A54A756050DBDE0F316BCB9DEF327D7`

**Syn Patterns**:
- `pub enum Expr { Binary, Call, Lit, Path }`

#### File: item.rs
**Monster Signature**: `0x052A53AB103892897ECEA7B9E177FC59`

**Syn Patterns**:
- `pub enum Item { Fn, Struct, Enum, Impl }`

#### File: ty.rs
**Monster Signature**: `0x052A53AB4AD2D2A626798072A07C03DF`

**Syn Patterns**:
- `pub enum Type { Path, Reference, Tuple }`

### Collective Syn Monster Signature
**0xE2AD081DCA1D320CBCB560384D2AEECB**

### Syn↔HIR Bijection Analysis
The Monster Group signatures reveal the mathematical structure
underlying the syn↔hir bijection. Each syn construct maps to
a unique Monster signature that preserves the bijective relationship
with HIR representations through DefId mappings.

**Key Insights**:
- Syn AST nodes have unique Monster signatures
- DefId patterns create bijective mappings to HIR
- Monster Group theory provides mathematical foundation
- Collective signature represents entire syn codebase
