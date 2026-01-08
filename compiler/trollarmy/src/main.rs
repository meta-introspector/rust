use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Fixme {
    id: String,
    content: String,
    source: String,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Ticket {
    id: String,
    fixmes: Vec<String>,
    cluster_id: Option<String>,
    priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cluster {
    id: String,
    tickets: Vec<String>,
    ontology_tag: String,
    similarity_score: f64,
}

struct TrollArmy {
    fixmes: HashMap<String, Fixme>,
    tickets: HashMap<String, Ticket>,
    clusters: HashMap<String, Cluster>,
}

impl TrollArmy {
    fn new() -> Self {
        Self {
            fixmes: HashMap::new(),
            tickets: HashMap::new(),
            clusters: HashMap::new(),
        }
    }

    // Stage 1: Consume everything as FIXMEs
    fn consume_as_fixme(&mut self, content: &str, source: &str) {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let id = format!("{:x}", hasher.finalize())[..8].to_string();
        
        let fixme = Fixme {
            id: id.clone(),
            content: content.to_string(),
            source: source.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        self.fixmes.insert(id, fixme);
        println!("🧌 CONSUMED: {} chars from {}", content.len(), source);
    }

    // Stage 2: De-dup and create tickets
    fn create_tickets(&mut self) {
        let mut content_groups: HashMap<String, Vec<String>> = HashMap::new();
        
        for (id, fixme) in &self.fixmes {
            let normalized = fixme.content.to_lowercase().trim().to_string();
            content_groups.entry(normalized).or_default().push(id.clone());
        }

        for (content, fixme_ids) in content_groups {
            if fixme_ids.len() > 1 {
                println!("🎫 DEDUP: {} duplicates found", fixme_ids.len());
            }
            
            let ticket_id = format!("T{:04}", self.tickets.len() + 1);
            let ticket = Ticket {
                id: ticket_id.clone(),
                fixmes: fixme_ids,
                cluster_id: None,
                priority: if content.contains("error") { 3 } else { 1 },
            };
            
            self.tickets.insert(ticket_id, ticket);
        }
        
        println!("🎫 TICKETS: {} created from {} fixmes", self.tickets.len(), self.fixmes.len());
    }

    // Stage 3: Create ontology with clustering
    fn create_ontology(&mut self) {
        let mut keyword_clusters: HashMap<String, Vec<String>> = HashMap::new();
        
        for (ticket_id, ticket) in &self.tickets {
            let combined_content: String = ticket.fixmes.iter()
                .filter_map(|fid| self.fixmes.get(fid))
                .map(|f| f.content.clone())
                .collect::<Vec<_>>()
                .join(" ");
            
            let ontology_tag = if combined_content.contains("compile") {
                "compilation"
            } else if combined_content.contains("network") || combined_content.contains("p2p") {
                "networking"
            } else if combined_content.contains("zombie") || combined_content.contains("spawn") {
                "distributed"
            } else if combined_content.contains("error") || combined_content.contains("fail") {
                "errors"
            } else {
                "general"
            };
            
            keyword_clusters.entry(ontology_tag.to_string())
                .or_default()
                .push(ticket_id.clone());
        }

        for (tag, ticket_ids) in keyword_clusters {
            let cluster_id = format!("C{}", tag.to_uppercase());
            let cluster = Cluster {
                id: cluster_id.clone(),
                tickets: ticket_ids.clone(),
                ontology_tag: tag,
                similarity_score: ticket_ids.len() as f64 / self.tickets.len() as f64,
            };
            
            // Update tickets with cluster assignment
            for ticket_id in &ticket_ids {
                if let Some(ticket) = self.tickets.get_mut(ticket_id) {
                    ticket.cluster_id = Some(cluster_id.clone());
                }
            }
            
            self.clusters.insert(cluster_id.clone(), cluster.clone());
            println!("🗂️  CLUSTER {}: {} tickets ({})", cluster_id, ticket_ids.len(), cluster.ontology_tag);
        }
    }

    fn report(&self) {
        println!("\n🧌 TROLLARMY REPORT:");
        println!("📝 FIXMEs: {}", self.fixmes.len());
        println!("🎫 Tickets: {}", self.tickets.len());
        println!("🗂️  Clusters: {}", self.clusters.len());
        
        for cluster in self.clusters.values() {
            println!("  {} ({}): {} tickets", cluster.id, cluster.ontology_tag, cluster.tickets.len());
        }
    }
}

fn main() {
    let mut army = TrollArmy::new();
    
    // Stage 1: Consume everything as FIXMEs
    army.consume_as_fixme("compilation failed with error", "zombie_client");
    army.consume_as_fixme("network connection timeout", "p2p_layer");
    army.consume_as_fixme("compilation failed with error", "zombie_wrapper");
    army.consume_as_fixme("spawn server on port 4001", "server_spawn");
    army.consume_as_fixme("zombie process memory leak", "distributed_system");
    army.consume_as_fixme("network connection timeout", "libp2p");
    
    // Stage 2: De-dup and create tickets
    army.create_tickets();
    
    // Stage 3: Create ontology with clustering
    army.create_ontology();
    
    // Report results
    army.report();
}
