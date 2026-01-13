# 🦀 Rust Mathematical Compiler Types Dataset

## Dataset Description

This dataset contains the **first complete mathematical classification of an entire programming language compiler**, featuring 5,724 unique Rust types extracted from the rustc_driver.so binary and analyzed using algebraic geometry, complexity theory, and visual emoji representations.

## Dataset Summary

- **Total Types**: 5,724 unique Rust compiler types
- **Mathematical Framework**: Algebraic geometry integration via LMFDB
- **Visual System**: Type-aware emoji representations
- **Analysis Method**: ELF binary symbol extraction and mathematical classification

## Features

| Feature | Type | Description |
|---------|------|-------------|
| `type_name` | string | Rust type name extracted from rustc_driver.so |
| `emoji` | string | Visual emoji representation based on mathematical properties |
| `category` | string | Type category classification |
| `frequency` | int32 | Usage frequency in rustc compiler binary |
| `complexity_score` | float64 | Mathematical complexity score (linear to elliptic progression) |
| `curve_class` | string | Algebraic curve classification from LMFDB integration |
| `genus` | int32 | Algebraic geometry genus from LMFDB mapping |
| `rank` | int32 | Elliptic curve rank from LMFDB classification |
| `torsion` | string | Torsion group structure from algebraic geometry |
| `mathematical_beauty` | float64 | Computed beauty score combining complexity, genus, and rank |

## Mathematical Foundation

### Algebraic Geometry Integration
Each type is mapped to mathematical objects from the **L-functions and Modular Forms Database (LMFDB)**:
- **Genus**: Topological invariant of algebraic curves
- **Rank**: Mordell-Weil rank of elliptic curves  
- **Torsion**: Finite subgroup structure

### Complexity Classification
Types are scored on mathematical complexity:
- **Linear**: Simple, direct types
- **Quadratic**: Moderate complexity with dependencies
- **Cubic**: Complex types with multiple relationships
- **Quartic**: High complexity with intricate structure
- **Elliptic**: Maximum complexity with deep mathematical properties

### Visual Emoji System
Type-aware emoji assignments based on mathematical properties:
- 🦀 Rust compiler core
- 🌳 AST structures  
- 🔢 Type system
- 🎯 Pattern matching
- 📦 Items/modules
- 💎 Literals
- ⚡ Functions

## Usage Examples

### Loading the Dataset

```python
from datasets import load_dataset

dataset = load_dataset("json", data_files="rust_types_huggingface_dataset.json")
```

### Analyzing Mathematical Beauty

```python
# Find most mathematically beautiful types
beautiful_types = dataset['train'].sort('mathematical_beauty', reverse=True)
print(beautiful_types.select(range(10))['type_name', 'emoji', 'mathematical_beauty'])
```

### Exploring Complexity Distribution

```python
import matplotlib.pyplot as plt

complexity_scores = dataset['train']['complexity_score']
plt.hist(complexity_scores, bins=50)
plt.xlabel('Mathematical Complexity Score')
plt.ylabel('Number of Types')
plt.title('Distribution of Rust Type Complexity')
```

### Analyzing Algebraic Properties

```python
# Group by curve class
curve_classes = dataset['train'].to_pandas().groupby('curve_class').size()
print("Types by algebraic curve class:")
print(curve_classes.sort_values(ascending=False))
```

## Research Applications

- **Programming Language Theory**: Mathematical foundations of type systems
- **Compiler Optimization**: Algebraic properties for optimization strategies
- **Visual Code Analysis**: Emoji-based code representation and pattern recognition
- **Cross-Language Analysis**: Mathematical comparison of programming languages
- **AI Training**: Mathematical code representations for machine learning

## Citation

```bibtex
@misc{rust_mathematical_compiler_2026,
  title={Mathematical Programming Language Analysis: Complete Classification of Rust Compiler Types},
  author={James Mike Dupont},
  email={h4@solfunmeme.com},
  year={2026},
  note={First complete mathematical classification of a programming language compiler using algebraic geometry}
}
```

## License

This dataset is released under MIT/Apache-2.0 license, consistent with the Rust programming language.

## Technical Details

### Extraction Method
Types were extracted from rustc_driver.so using:
1. **ELF Binary Analysis**: Symbol table and .rodata section parsing
2. **Demangling**: Rust symbol name demangling for clean type names
3. **Frequency Analysis**: Usage counting within the compiler binary
4. **Mathematical Classification**: LMFDB integration for algebraic properties

### Data Quality
- **Completeness**: All 5,724 unique types from rustc_driver.so
- **Accuracy**: Direct extraction from official Rust compiler binary
- **Consistency**: Mathematical properties verified against LMFDB
- **Reproducibility**: Complete toolchain available for regeneration

## Beautiful First Stab Philosophy

This dataset represents a "beautiful first stab" at mathematical programming language analysis - incomplete but mathematically consistent and visually elegant, establishing a new research field combining programming languages with pure mathematics.

## Contact

For questions about this dataset or the mathematical analysis methodology:
- **Author**: James Mike Dupont
- **Email**: h4@solfunmeme.com
- **Research Focus**: Mathematical programming language analysis and algebraic geometry applications
