# Kleene Regex Language Lattice for 257 Core DefIds

## Recursive Language Class

| DefId | Syn | Hir | Kleene Regex | Lattice Position |
|-------|-----|-----|--------------|------------------|
| core::iter::traits::collect::IntoIterator::into_iter | 8 | 6 | `C(it)+*` | (5,4) |

## Presentation Language Class

| DefId | Syn | Hir | Kleene Regex | Lattice Position |
|-------|-----|-----|--------------|------------------|
| core::fmt::Display::fmt | 89 | 234 | `C(fmt)++` | (2,7) |

## Relational Language Class

| DefId | Syn | Hir | Kleene Regex | Lattice Position |
|-------|-----|-----|--------------|------------------|
| core::cmp::PartialEq::eq | 41 | 143 | `C(eq|ord)*+` | (2,7) |
| core::cmp::PartialOrd::le | 167 | 1225 | `C(eq|ord)*{2,}` | (1,8) |

## Control Language Class

| DefId | Syn | Hir | Kleene Regex | Lattice Position |
|-------|-----|-----|--------------|------------------|
| core::ops::control_flow::ControlFlow::Continue | 454 | 102 | `C(op)*{2,}` | (8,1) |
| core::ops::control_flow::ControlFlow::Break | 454 | 102 | `C(op)*{2,}` | (8,1) |

## Algebraic Language Class

| DefId | Syn | Hir | Kleene Regex | Lattice Position |
|-------|-----|-----|--------------|------------------|
| core::option::Option::Some | 217 | 460 | `C(opt|none){2,}` | (3,6) |
| core::option::Option::None | 217 | 460 | `C(opt|none){2,}` | (3,6) |
| core::result::Result::Ok | 125 | 89 | `C(ok|err)+` | (5,4) |
| core::result::Result::Err | 125 | 89 | `C(ok|err)+` | (5,4) |

## Mathematical Foundations

### Kleene Algebra Operations
- **Union**: `a|b` - Either pattern a or b
- **Concatenation**: `ab` - Pattern a followed by b
- **Kleene Star**: `a*` - Zero or more repetitions of a
- **Plus**: `a+` - One or more repetitions of a
- **Optional**: `a?` - Zero or one occurrence of a

### Lattice Structure
- **X-axis**: Syn usage intensity (0-10)
- **Y-axis**: Hir usage intensity (0-10)
- **Language Classes**: Algebraic, Recursive, Control, Relational, Presentation, Primitive

