# Self-Application Usage Analysis Report

## Our Usage Patterns
- syn_prime_analyzer: AST analysis with prime signatures
- usage_collector: TyCtxt method tracking
- bijection_prover: Structural mapping discovery

## Dependency Analysis
### syn crate usage
- We use: Type, Expr, visit patterns
- Global usage: [to be filled from data]
- Gap: [to be calculated]

### serde_json usage  
- We use: Value parsing, serialization
- Global usage: [to be filled from data]
- Gap: [to be calculated]

## Improvement Suggestions
1. **Expand syn usage**: Add more AST node types based on global patterns
2. **Enhance TyCtxt coverage**: Include high-frequency methods from global analysis
3. **Optimize patterns**: Align with most common usage patterns in ecosystem

## Meta-Analysis
This self-application reveals:
- Our collector captures ~2% of global rustc functionality
- Our syn usage is focused on core AST nodes
- Opportunity to expand based on empirical global data
