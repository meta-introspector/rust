use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct DepTriple {
    pub from: String,
    pub relation: String, 
    pub to: String,
}

pub struct TopologicalTracer {
    pub triples: Vec<DepTriple>,
    pub nodes: HashSet<String>,
    pub edges: HashMap<String, Vec<String>>,
}

impl TopologicalTracer {
    pub fn new() -> Self {
        Self {
            triples: Vec::new(),
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }
    
    pub fn add_triple(&mut self, from: &str, relation: &str, to: &str) {
        let triple = DepTriple {
            from: from.to_string(),
            relation: relation.to_string(),
            to: to.to_string(),
        };
        
        self.triples.push(triple);
        self.nodes.insert(from.to_string());
        self.nodes.insert(to.to_string());
        self.edges.entry(from.to_string()).or_default().push(to.to_string());
    }
    
    pub fn topological_sort(&self) -> Vec<String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        // Initialize in-degrees
        for node in &self.nodes {
            in_degree.insert(node.clone(), 0);
        }
        
        for edges in self.edges.values() {
            for to in edges {
                *in_degree.get_mut(to).unwrap() += 1;
            }
        }
        
        // Find nodes with no incoming edges
        for (node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node.clone());
            }
        }
        
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            
            if let Some(neighbors) = self.edges.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        result
    }
}
