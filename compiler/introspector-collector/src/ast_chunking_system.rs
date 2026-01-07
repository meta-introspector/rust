/// AST Chunking and LLM Context Optimizer
/// Partition global ASTs → 4K chunks → Backpack optimization → Feed to Ollama/Gemini

use crate::llm_context_optimizer::{LLMContextOptimizer, ContextItem, ContextType};
use std::collections::HashMap;
use serde_json::Value;

/// AST chunk with metadata
#[derive(Debug, Clone)]
pub struct ASTChunk {
    pub id: String,
    pub content: String,
    pub token_count: u32,
    pub ast_type: ASTType,
    pub relevance_score: u32,
    pub file_path: String,
    pub line_range: (u32, u32),
}

#[derive(Debug, Clone)]
pub enum ASTType {
    Function,
    Struct,
    Enum,
    Impl,
    Module,
    Macro,
    Type,
    Trait,
}

/// LLM provider interface
#[derive(Debug, Clone)]
pub enum LLMProvider {
    Ollama { model: String, endpoint: String },
    Gemini { api_key: String, model: String },
}

/// AST chunking and LLM feeding system
pub struct ASTChunkingSystem {
    pub chunks: Vec<ASTChunk>,
    pub chunk_size: u32,
    pub optimizer: LLMContextOptimizer,
    pub llm_provider: LLMProvider,
}

impl ASTChunkingSystem {
    pub fn new(chunk_size: u32, provider: LLMProvider) -> Self {
        Self {
            chunks: vec![],
            chunk_size,
            optimizer: LLMContextOptimizer::new(chunk_size),
            llm_provider: provider,
        }
    }
    
    /// Partition global AST into 4K chunks
    pub fn partition_global_ast(&mut self, ast_content: &str, file_path: &str) -> Vec<ASTChunk> {
        let mut chunks = vec![];
        let lines: Vec<&str> = ast_content.lines().collect();
        
        let mut current_chunk = String::new();
        let mut current_tokens = 0;
        let mut start_line = 1;
        let mut chunk_id = 0;
        
        for (line_num, line) in lines.iter().enumerate() {
            let line_tokens = self.estimate_tokens(line);
            
            if current_tokens + line_tokens > self.chunk_size && !current_chunk.is_empty() {
                // Create chunk
                let chunk = ASTChunk {
                    id: format!("{}_{}", file_path, chunk_id),
                    content: current_chunk.clone(),
                    token_count: current_tokens,
                    ast_type: self.detect_ast_type(&current_chunk),
                    relevance_score: self.calculate_relevance(&current_chunk),
                    file_path: file_path.to_string(),
                    line_range: (start_line, line_num as u32),
                };
                
                chunks.push(chunk);
                
                // Reset for next chunk
                current_chunk.clear();
                current_tokens = 0;
                start_line = line_num as u32 + 1;
                chunk_id += 1;
            }
            
            current_chunk.push_str(line);
            current_chunk.push('\n');
            current_tokens += line_tokens;
        }
        
        // Add final chunk
        if !current_chunk.is_empty() {
            let chunk = ASTChunk {
                id: format!("{}_{}", file_path, chunk_id),
                content: current_chunk,
                token_count: current_tokens,
                ast_type: self.detect_ast_type(&current_chunk),
                relevance_score: self.calculate_relevance(&current_chunk),
                file_path: file_path.to_string(),
                line_range: (start_line, lines.len() as u32),
            };
            chunks.push(chunk);
        }
        
        self.chunks.extend(chunks.clone());
        chunks
    }
    
    /// Estimate token count (rough approximation)
    fn estimate_tokens(&self, text: &str) -> u32 {
        // Rough estimate: 1 token per 4 characters
        (text.len() as f32 / 4.0).ceil() as u32
    }
    
    /// Detect AST type from content
    fn detect_ast_type(&self, content: &str) -> ASTType {
        if content.contains("fn ") { ASTType::Function }
        else if content.contains("struct ") { ASTType::Struct }
        else if content.contains("enum ") { ASTType::Enum }
        else if content.contains("impl ") { ASTType::Impl }
        else if content.contains("mod ") { ASTType::Module }
        else if content.contains("macro_rules!") { ASTType::Macro }
        else if content.contains("trait ") { ASTType::Trait }
        else { ASTType::Type }
    }
    
    /// Calculate relevance score
    fn calculate_relevance(&self, content: &str) -> u32 {
        let mut score = 50; // Base score
        
        // Boost for important patterns
        if content.contains("DiracDelta") { score += 20; }
        if content.contains("Universal") { score += 15; }
        if content.contains("Self") { score += 10; }
        if content.contains("impl") { score += 10; }
        if content.contains("pub fn") { score += 8; }
        if content.contains("macro") { score += 12; }
        
        // Penalty for common/low-value content
        if content.contains("use std") { score -= 5; }
        if content.contains("println!") { score -= 3; }
        
        score.max(1)
    }
    
    /// Optimize chunks for LLM context using backpack algorithm
    pub fn optimize_context_chunks(&mut self, query: &str) -> Vec<ASTChunk> {
        // Clear previous optimization
        self.optimizer = LLMContextOptimizer::new(self.chunk_size);
        
        // Add chunks as context items
        for chunk in &self.chunks {
            let context_type = match chunk.ast_type {
                ASTType::Function => ContextType::Code,
                ASTType::Struct | ASTType::Enum => ContextType::Code,
                ASTType::Impl => ContextType::Code,
                ASTType::Module => ContextType::Documentation,
                ASTType::Macro => ContextType::Example,
                ASTType::Trait => ContextType::Code,
                ASTType::Type => ContextType::Documentation,
            };
            
            // Boost relevance based on query similarity
            let boosted_relevance = self.boost_relevance_for_query(&chunk.content, query, chunk.relevance_score);
            
            self.optimizer.add_item(
                chunk.content.clone(),
                chunk.token_count,
                boosted_relevance,
                context_type,
            );
        }
        
        // Get optimal selection
        let (selected_indices, _, _, _) = self.optimizer.optimize_with_mcts();
        
        // Return selected chunks
        selected_indices.into_iter()
            .filter_map(|i| self.chunks.get(i).cloned())
            .collect()
    }
    
    fn boost_relevance_for_query(&self, content: &str, query: &str, base_score: u32) -> u32 {
        let query_words: Vec<&str> = query.split_whitespace().collect();
        let mut boost = 0;
        
        for word in query_words {
            if content.to_lowercase().contains(&word.to_lowercase()) {
                boost += 10;
            }
        }
        
        base_score + boost
    }
    
    /// Feed optimized chunks to LLM
    pub async fn feed_to_llm(&self, chunks: &[ASTChunk], query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let context = self.build_context_prompt(chunks, query);
        
        match &self.llm_provider {
            LLMProvider::Ollama { model, endpoint } => {
                self.query_ollama(endpoint, model, &context).await
            },
            LLMProvider::Gemini { api_key, model } => {
                self.query_gemini(api_key, model, &context).await
            },
        }
    }
    
    fn build_context_prompt(&self, chunks: &[ASTChunk], query: &str) -> String {
        let mut prompt = String::from("CONTEXT: Rust AST chunks for analysis\n\n");
        
        for (i, chunk) in chunks.iter().enumerate() {
            prompt.push_str(&format!(
                "CHUNK {}: {} ({:?}, {} tokens)\n",
                i + 1, chunk.id, chunk.ast_type, chunk.token_count
            ));
            prompt.push_str(&format!("File: {} (lines {}-{})\n", 
                chunk.file_path, chunk.line_range.0, chunk.line_range.1));
            prompt.push_str("```rust\n");
            prompt.push_str(&chunk.content);
            prompt.push_str("```\n\n");
        }
        
        prompt.push_str(&format!("QUERY: {}\n\n", query));
        prompt.push_str("Please analyze the provided Rust AST chunks and answer the query.");
        
        prompt
    }
    
    async fn query_ollama(&self, endpoint: &str, model: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        });
        
        let response = client
            .post(&format!("{}/api/generate", endpoint))
            .json(&payload)
            .send()
            .await?;
        
        let result: Value = response.json().await?;
        Ok(result["response"].as_str().unwrap_or("No response").to_string())
    }
    
    async fn query_gemini(&self, api_key: &str, model: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }]
        });
        
        let response = client
            .post(&format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", model, api_key))
            .json(&payload)
            .send()
            .await?;
        
        let result: Value = response.json().await?;
        Ok(result["candidates"][0]["content"]["parts"][0]["text"]
            .as_str().unwrap_or("No response").to_string())
    }
    
    /// Complete AST analysis pipeline
    pub async fn analyze_ast_with_llm(&mut self, ast_content: &str, file_path: &str, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 1. Partition AST into chunks
        let chunks = self.partition_global_ast(ast_content, file_path);
        println!("📊 Partitioned into {} chunks", chunks.len());
        
        // 2. Optimize chunks for context
        let optimized_chunks = self.optimize_context_chunks(query);
        println!("🎒 Optimized to {} chunks for context", optimized_chunks.len());
        
        // 3. Feed to LLM
        let response = self.feed_to_llm(&optimized_chunks, query).await?;
        
        Ok(format!(
            "AST ANALYSIS RESULTS:\n\
             \n\
             Query: {}\n\
             File: {}\n\
             Total chunks: {}\n\
             Optimized chunks: {}\n\
             \n\
             LLM Response:\n\
             {}",
            query, file_path, chunks.len(), optimized_chunks.len(), response
        ))
    }
}

/// Macro for AST analysis
#[macro_export]
macro_rules! analyze_ast {
    ($ast_content:expr, $file_path:expr, $query:expr, $provider:expr) => {{
        let mut system = ASTChunkingSystem::new(4096, $provider);
        system.analyze_ast_with_llm($ast_content, $file_path, $query).await
    }};
}
