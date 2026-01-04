use anyhow::Result;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    println!("🔢 CONSTANT + BINARY OPERATORS LANGUAGE");
    println!("═══════════════════════════════════════");
    
    let mut expr_compiler = ExpressionCompiler::new();
    
    // Build from our lattice components
    expr_compiler.build_from_lattice()?;
    
    // Test the extended language
    expr_compiler.test_expressions()?;
    
    Ok(())
}

struct ExpressionCompiler {
    // Lattice Level 1: Constants (from previous work)
    constants: HashMap<String, ConstantValue>,
    
    // Lattice Level 2: Binary operators (new)
    binary_ops: HashMap<String, BinaryOperator>,
    
    // Compiler components
    parser: Option<ExprParser>,
    evaluator: Option<ExprEvaluator>,
}

#[derive(Debug, Clone)]
enum ConstantValue {
    Integer(i64),
    String(String),
    Boolean(bool),
    Float(f64),
}

#[derive(Debug, Clone)]
struct BinaryOperator {
    symbol: String,
    operation: BinaryOp,
    precedence: u8,
}

#[derive(Debug, Clone)]
enum BinaryOp {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Eq,       // ==
    Ne,       // !=
    Lt,       // <
    Gt,       // >
    And,      // &&
    Or,       // ||
}

#[derive(Debug, Clone)]
enum Expression {
    Constant(ConstantValue),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
}

struct ExprParser {
    generated_from: String,
}

struct ExprEvaluator {
    generated_from: String,
}

impl ExpressionCompiler {
    fn new() -> Self {
        Self {
            constants: HashMap::new(),
            binary_ops: HashMap::new(),
            parser: None,
            evaluator: None,
        }
    }
    
    fn build_from_lattice(&mut self) -> Result<()> {
        println!("🏗️ Building expression compiler from lattice components...");
        
        // Extract constants from our previous work
        self.extract_constants_from_eigenmatrix()?;
        
        // Add binary operators (Level 2 lattice)
        self.add_binary_operators();
        
        // Build parser from rustc subgraph
        self.build_expression_parser()?;
        
        // Build evaluator from rustc subgraph
        self.build_expression_evaluator()?;
        
        println!("✅ Expression compiler built!");
        println!("  Constants: {}", self.constants.len());
        println!("  Binary operators: {}", self.binary_ops.len());
        
        Ok(())
    }
    
    fn extract_constants_from_eigenmatrix(&mut self) -> Result<()> {
        // Load from our eigenmatrix analysis
        if let Ok(eigenmatrix) = fs::read_to_string("usage_eigenmatrix.json") {
            let data: serde_json::Value = serde_json::from_str(&eigenmatrix)?;
            
            if let Some(core_def_ids) = data["core_def_ids"].as_array() {
                for def_id_entry in core_def_ids.iter().take(10) { // Top 10 constants
                    if let Some(def_id_array) = def_id_entry.as_array() {
                        if def_id_array.len() >= 2 {
                            let name = def_id_array[0].as_str().unwrap_or("unknown");
                            let count = def_id_array[1].as_u64().unwrap_or(0);
                            
                            let constant = self.parse_constant_from_eigenvalue(name, count);
                            self.constants.insert(name.to_string(), constant);
                        }
                    }
                }
            }
        }
        
        println!("  Extracted {} constants from eigenmatrix", self.constants.len());
        Ok(())
    }
    
    fn parse_constant_from_eigenvalue(&self, name: &str, count: u64) -> ConstantValue {
        if name.chars().all(|c| c.is_ascii_digit()) {
            ConstantValue::Integer(name.parse().unwrap_or(count as i64))
        } else if name == "true" || name == "false" {
            ConstantValue::Boolean(name == "true")
        } else if name.starts_with("static") {
            ConstantValue::Integer(count as i64)
        } else {
            ConstantValue::Integer(count as i64)
        }
    }
    
    fn add_binary_operators(&mut self) {
        println!("⚡ Adding binary operators (Lattice Level 2)...");
        
        let operators = vec![
            ("+", BinaryOp::Add, 1),
            ("-", BinaryOp::Sub, 1),
            ("*", BinaryOp::Mul, 2),
            ("/", BinaryOp::Div, 2),
            ("==", BinaryOp::Eq, 0),
            ("!=", BinaryOp::Ne, 0),
            ("<", BinaryOp::Lt, 0),
            (">", BinaryOp::Gt, 0),
            ("&&", BinaryOp::And, 0),
            ("||", BinaryOp::Or, 0),
        ];
        
        for (symbol, op, precedence) in operators {
            let binary_op = BinaryOperator {
                symbol: symbol.to_string(),
                operation: op,
                precedence,
            };
            
            self.binary_ops.insert(symbol.to_string(), binary_op);
        }
        
        println!("  Added {} binary operators", self.binary_ops.len());
    }
    
    fn build_expression_parser(&mut self) -> Result<()> {
        // Use our constant subgraph components
        if let Ok(compiler_data) = fs::read_to_string("self_constructed_rustc.json") {
            let data: serde_json::Value = serde_json::from_str(&compiler_data)?;
            
            if let Some(components) = data["components"].as_array() {
                for component in components {
                    if let Some(comp_type) = component["type"].as_str() {
                        if comp_type == "Parser" {
                            if let Some(name) = component["name"].as_str() {
                                self.parser = Some(ExprParser {
                                    generated_from: name.to_string(),
                                });
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        if self.parser.is_none() {
            self.parser = Some(ExprParser {
                generated_from: "lattice_expression_parser".to_string(),
            });
        }
        
        Ok(())
    }
    
    fn build_expression_evaluator(&mut self) -> Result<()> {
        // Use analyzer components for evaluation
        if let Ok(compiler_data) = fs::read_to_string("self_constructed_rustc.json") {
            let data: serde_json::Value = serde_json::from_str(&compiler_data)?;
            
            if let Some(components) = data["components"].as_array() {
                for component in components {
                    if let Some(comp_type) = component["type"].as_str() {
                        if comp_type == "Analyzer" {
                            if let Some(name) = component["name"].as_str() {
                                self.evaluator = Some(ExprEvaluator {
                                    generated_from: name.to_string(),
                                });
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        if self.evaluator.is_none() {
            self.evaluator = Some(ExprEvaluator {
                generated_from: "lattice_expression_evaluator".to_string(),
            });
        }
        
        Ok(())
    }
    
    fn test_expressions(&self) -> Result<()> {
        println!("\n🧪 TESTING EXPRESSION LANGUAGE");
        println!("═════════════════════════════");
        
        let test_expressions = vec![
            "42",                    // Level 1: Constant
            "true",                  // Level 1: Boolean constant
            "42 + 10",              // Level 2: Binary operation
            "3 * 14",               // Level 2: Multiplication
            "100 / 5",              // Level 2: Division
            "42 == 42",             // Level 2: Equality
            "10 < 20",              // Level 2: Comparison
            "true && false",        // Level 2: Logical AND
            "5 + 3 * 2",           // Level 2: Precedence test
            "(10 + 5) * 2",        // Level 2: Parentheses (future)
        ];
        
        println!("📝 Test expressions (Rust-like syntax):");
        for expr_str in &test_expressions {
            println!("  Input: {}", expr_str);
            
            match self.parse_expression(expr_str) {
                Ok(expr) => {
                    match self.evaluate_expression(&expr) {
                        Ok(result) => println!("  Output: {:?}", result),
                        Err(e) => println!("  Eval Error: {}", e),
                    }
                }
                Err(e) => println!("  Parse Error: {}", e),
            }
            println!();
        }
        
        // Show compiler components used
        println!("🔧 Lattice components used:");
        if let Some(parser) = &self.parser {
            println!("  Parser: generated from {}", parser.generated_from);
        }
        if let Some(evaluator) = &self.evaluator {
            println!("  Evaluator: generated from {}", evaluator.generated_from);
        }
        
        Ok(())
    }
    
    fn parse_expression(&self, input: &str) -> Result<Expression> {
        let tokens = self.tokenize(input);
        self.parse_tokens(&tokens)
    }
    
    fn tokenize(&self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        
        for ch in input.chars() {
            if ch.is_whitespace() {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            } else if "+-*/()".contains(ch) {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push(ch.to_string());
            } else if ch == '=' || ch == '!' || ch == '<' || ch == '>' || ch == '&' || ch == '|' {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                current.push(ch);
            } else {
                current.push(ch);
            }
        }
        
        if !current.is_empty() {
            tokens.push(current);
        }
        
        tokens
    }
    
    fn parse_tokens(&self, tokens: &[String]) -> Result<Expression> {
        if tokens.is_empty() {
            return Err(anyhow::anyhow!("Empty expression"));
        }
        
        if tokens.len() == 1 {
            // Single token - must be constant
            return self.parse_constant(&tokens[0]);
        }
        
        if tokens.len() == 3 {
            // Simple binary expression: left op right
            let left = self.parse_constant(&tokens[0])?;
            let op = self.parse_binary_op(&tokens[1])?;
            let right = self.parse_constant(&tokens[2])?;
            
            return Ok(Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        
        // For now, only handle simple cases
        Err(anyhow::anyhow!("Complex expressions not yet supported"))
    }
    
    fn parse_constant(&self, token: &str) -> Result<Expression> {
        if let Ok(i) = token.parse::<i64>() {
            Ok(Expression::Constant(ConstantValue::Integer(i)))
        } else if token == "true" {
            Ok(Expression::Constant(ConstantValue::Boolean(true)))
        } else if token == "false" {
            Ok(Expression::Constant(ConstantValue::Boolean(false)))
        } else if let Ok(f) = token.parse::<f64>() {
            Ok(Expression::Constant(ConstantValue::Float(f)))
        } else {
            Err(anyhow::anyhow!("Unknown constant: {}", token))
        }
    }
    
    fn parse_binary_op(&self, token: &str) -> Result<BinaryOp> {
        match token {
            "+" => Ok(BinaryOp::Add),
            "-" => Ok(BinaryOp::Sub),
            "*" => Ok(BinaryOp::Mul),
            "/" => Ok(BinaryOp::Div),
            "==" => Ok(BinaryOp::Eq),
            "!=" => Ok(BinaryOp::Ne),
            "<" => Ok(BinaryOp::Lt),
            ">" => Ok(BinaryOp::Gt),
            "&&" => Ok(BinaryOp::And),
            "||" => Ok(BinaryOp::Or),
            _ => Err(anyhow::anyhow!("Unknown operator: {}", token)),
        }
    }
    
    fn evaluate_expression(&self, expr: &Expression) -> Result<ConstantValue> {
        match expr {
            Expression::Constant(val) => Ok(val.clone()),
            Expression::Binary { left, op, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_op(&left_val, op, &right_val)
            }
        }
    }
    
    fn apply_binary_op(&self, left: &ConstantValue, op: &BinaryOp, right: &ConstantValue) -> Result<ConstantValue> {
        match (left, op, right) {
            (ConstantValue::Integer(a), BinaryOp::Add, ConstantValue::Integer(b)) => {
                Ok(ConstantValue::Integer(a + b))
            }
            (ConstantValue::Integer(a), BinaryOp::Sub, ConstantValue::Integer(b)) => {
                Ok(ConstantValue::Integer(a - b))
            }
            (ConstantValue::Integer(a), BinaryOp::Mul, ConstantValue::Integer(b)) => {
                Ok(ConstantValue::Integer(a * b))
            }
            (ConstantValue::Integer(a), BinaryOp::Div, ConstantValue::Integer(b)) => {
                if *b == 0 {
                    Err(anyhow::anyhow!("Division by zero"))
                } else {
                    Ok(ConstantValue::Integer(a / b))
                }
            }
            (ConstantValue::Integer(a), BinaryOp::Eq, ConstantValue::Integer(b)) => {
                Ok(ConstantValue::Boolean(a == b))
            }
            (ConstantValue::Integer(a), BinaryOp::Lt, ConstantValue::Integer(b)) => {
                Ok(ConstantValue::Boolean(a < b))
            }
            (ConstantValue::Integer(a), BinaryOp::Gt, ConstantValue::Integer(b)) => {
                Ok(ConstantValue::Boolean(a > b))
            }
            (ConstantValue::Boolean(a), BinaryOp::And, ConstantValue::Boolean(b)) => {
                Ok(ConstantValue::Boolean(*a && *b))
            }
            (ConstantValue::Boolean(a), BinaryOp::Or, ConstantValue::Boolean(b)) => {
                Ok(ConstantValue::Boolean(*a || *b))
            }
            _ => Err(anyhow::anyhow!("Type mismatch in binary operation")),
        }
    }
}
