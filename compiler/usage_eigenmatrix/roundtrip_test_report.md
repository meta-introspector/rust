# Roundtrip Test Report: Source ↔ Syn ↔ HIR ↔ Numerical

## Summary
- Total tests: 3
- Successful roundtrips: 0
- Success rate: 0.0%

## Test Results
| Test | Syn Sig | HIR Sig | Numerical Sig | Roundtrip |
|------|---------|---------|---------------|----------|
| 1 | `0x00000013` | `0x00000272` | `0x69EB9C76` | ❌ |
| 2 | `0x002DE8E3` | `0x058F338E` | `0x07782FFA` | ❌ |
| 3 | `0x00000017` | `0x000002EE` | `0xB19C253B` | ❌ |

## Detailed Results

### Test 1
**Original**: `fn main() { let x = 1 + 2; }`
**Reconstructed**: `fn main() {{ let x = 51; }}`
**Signatures**: Syn=0x0000000000000013, HIR=0x0000000000000272, Num=0x0000000169EB9C76

### Test 2
**Original**: `fn add(a: i32, b: i32) -> i32 { a + b } fn main() { add(1, 2`
**Reconstructed**: `fn main() {{ let x = 88; }}`
**Signatures**: Syn=0x00000000002DE8E3, HIR=0x00000000058F338E, Num=0x000336D107782FFA

### Test 3
**Original**: `enum Color { Red, Green, Blue } fn main() { let c = Color::R`
**Reconstructed**: `fn main() {{ let x = 4; }}`
**Signatures**: Syn=0x0000000000000017, HIR=0x00000000000002EE, Num=0x00000001B19C253B
