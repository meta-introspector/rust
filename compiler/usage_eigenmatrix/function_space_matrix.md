# Function Space Occupancy Matrix

## Space Signatures

| Program | Functions Visited | Signature |
|---------|-------------------|----------|
| const_int | 11 | codegen_int|lex_const|parse|parse_assignment|parse_const_decl|parse_identifier|parse_integer|parse_semicolon|symbol_table_lookup|tokenize|type_check_int |
| const_int_2 | 11 | codegen_int|lex_const|parse|parse_assignment|parse_const_decl|parse_identifier|parse_integer|parse_semicolon|symbol_table_lookup|tokenize|type_check_int |
| const_int_3 | 11 | codegen_int|lex_const|parse|parse_assignment|parse_const_decl|parse_identifier|parse_integer|parse_semicolon|symbol_table_lookup|tokenize|type_check_int |
| const_bool | 11 | codegen_bool|lex_const|parse|parse_assignment|parse_bool|parse_const_decl|parse_identifier|parse_semicolon|symbol_table_lookup|tokenize|type_check_bool |
| let_int | 11 | codegen_int|lex_let|parse|parse_assignment|parse_identifier|parse_integer|parse_let_decl|parse_semicolon|symbol_table_lookup|tokenize|type_check_int |
| empty_fn | 8 | lex_fn|parse|parse_block|parse_fn_decl|parse_identifier|parse_parens|symbol_table_lookup|tokenize |

## Equivalence Matrix (✅ = Same Space)

| const_int | const_int_2 | const_int_3 | const_bool | let_int | empty_fn |
|-------|-------|-------|-------|-------|-------|
| const_int | 🔵 | ✅ | ✅ | ❌ | ❌ | ❌ |
| const_int_2 | ✅ | 🔵 | ✅ | ❌ | ❌ | ❌ |
| const_int_3 | ✅ | ✅ | 🔵 | ❌ | ❌ | ❌ |
| const_bool | ❌ | ❌ | ❌ | 🔵 | ❌ | ❌ |
| let_int | ❌ | ❌ | ❌ | ❌ | 🔵 | ❌ |
| empty_fn | ❌ | ❌ | ❌ | ❌ | ❌ | 🔵 |

## Detailed Function Spaces

### const_int Functions
- codegen_int
- lex_const
- parse
- parse_assignment
- parse_const_decl
- parse_identifier
- parse_integer
- parse_semicolon
- symbol_table_lookup
- tokenize
- type_check_int

### const_int_2 Functions
- codegen_int
- lex_const
- parse
- parse_assignment
- parse_const_decl
- parse_identifier
- parse_integer
- parse_semicolon
- symbol_table_lookup
- tokenize
- type_check_int

### const_int_3 Functions
- codegen_int
- lex_const
- parse
- parse_assignment
- parse_const_decl
- parse_identifier
- parse_integer
- parse_semicolon
- symbol_table_lookup
- tokenize
- type_check_int

### const_bool Functions
- codegen_bool
- lex_const
- parse
- parse_assignment
- parse_bool
- parse_const_decl
- parse_identifier
- parse_semicolon
- symbol_table_lookup
- tokenize
- type_check_bool

### let_int Functions
- codegen_int
- lex_let
- parse
- parse_assignment
- parse_identifier
- parse_integer
- parse_let_decl
- parse_semicolon
- symbol_table_lookup
- tokenize
- type_check_int

### empty_fn Functions
- lex_fn
- parse
- parse_block
- parse_fn_decl
- parse_identifier
- parse_parens
- symbol_table_lookup
- tokenize

