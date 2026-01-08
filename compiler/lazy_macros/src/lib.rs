use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, visit::Visit, File};
use std::collections::HashMap;

#[proc_macro]
pub fn lazyverb(_input: TokenStream) -> TokenStream {
    quote! {
        println!("🧟‍♂️💤 LAZYVERB: Too lazy to implement this verb properly!");
    }.into()
}

#[proc_macro]
pub fn lazyverb(input: TokenStream) -> TokenStream {
    let verb_name = input.to_string().trim_matches('"').to_string();
    
    quote! {
        {
            let verb_data = serde_json::json!({
                "verb": #verb_name,
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "peer_id": local_peer_id.to_string(),
                "action": "lazy_verb_execution"
            });
            
            if let Ok(verb_json) = serde_json::to_string(&verb_data) {
                if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), verb_json.as_bytes()) {
                    log_message(&format!("🧟‍♂️💤 LAZYVERB {}: Failed to publish - {}", #verb_name, e));
                } else {
                    log_message(&format!("🧟‍♂️📡 LAZYVERB {}: Published to network", #verb_name));
                }
            }
        }
    }.into()
}

#[proc_macro]
pub fn lazyapply(input: TokenStream) -> TokenStream {
    let operation = input.to_string().trim_matches('"').to_string();
    
    quote! {
        {
            let apply_data = serde_json::json!({
                "operation": #operation,
                "status": "lazy_applied",
                "result": "placeholder_success",
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            
            if let Ok(apply_json) = serde_json::to_string(&apply_data) {
                if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), apply_json.as_bytes()) {
                    log_message(&format!("🧟‍♂️💤 LAZYAPPLY {}: Failed to publish - {}", #operation, e));
                } else {
                    log_message(&format!("🧟‍♂️⚡ LAZYAPPLY {}: Operation applied and published", #operation));
                }
            }
        }
    }.into()
}

#[proc_macro]
pub fn lazydata(input: TokenStream) -> TokenStream {
    let file = parse_macro_input!(input as File);
    
    let mut visitor = SpanDataVisitor::new();
    visitor.visit_file(&file);
    
    let node_counts = visitor.node_counts;
    let total_nodes = visitor.total_nodes;
    let max_depth = visitor.max_depth;
    
    quote! {
        {
            let mut data = std::collections::HashMap::new();
            #(
                data.insert(#node_counts.0.to_string(), #node_counts.1);
            )*
            
            println!("🧟‍♂️📊 LAZYDATA: {} total nodes, max depth {}", #total_nodes, #max_depth);
            println!("🧠 Node types: {:?}", data);
            
            SpanFeatures {
                total_spans: #total_nodes,
                max_nesting_depth: #max_depth,
                node_type_counts: data,
                ..SpanFeatures::new()
            }
        }
    }.into()
}

struct SpanDataVisitor {
    node_counts: Vec<(String, u32)>,
    total_nodes: u32,
    max_depth: u32,
    current_depth: u32,
}

impl SpanDataVisitor {
    fn new() -> Self {
        Self {
            node_counts: Vec::new(),
            total_nodes: 0,
            max_depth: 0,
            current_depth: 0,
        }
    }
    
    fn count_node(&mut self, node_type: &str) {
        self.total_nodes += 1;
        self.max_depth = self.max_depth.max(self.current_depth);
        
        if let Some(pos) = self.node_counts.iter().position(|(name, _)| name == node_type) {
            self.node_counts[pos].1 += 1;
        } else {
            self.node_counts.push((node_type.to_string(), 1));
        }
    }
}

impl<'ast> Visit<'ast> for SpanDataVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.count_node("FnDef");
        self.current_depth += 1;
        syn::visit::visit_item_fn(self, node);
        self.current_depth -= 1;
    }
    
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.count_node("Struct");
        self.current_depth += 1;
        syn::visit::visit_item_struct(self, node);
        self.current_depth -= 1;
    }
    
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.count_node("Enum");
        self.current_depth += 1;
        syn::visit::visit_item_enum(self, node);
        self.current_depth -= 1;
    }
    
    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        self.count_node("IfExpr");
        self.current_depth += 1;
        syn::visit::visit_expr_if(self, node);
        self.current_depth -= 1;
    }
    
    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        self.count_node("MatchExpr");
        self.current_depth += 1;
        syn::visit::visit_expr_match(self, node);
        self.current_depth -= 1;
    }
    
    fn visit_local(&mut self, node: &'ast syn::Local) {
        self.count_node("LocalBinding");
        syn::visit::visit_local(self, node);
    }
    
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        self.count_node("CallExpr");
        syn::visit::visit_expr_call(self, node);
    }
}
