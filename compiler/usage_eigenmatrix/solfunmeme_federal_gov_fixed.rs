//! # SOLFUNMEME Federal Government: Senate, House, and Presidential Veto
//! 
//! US Federal model: Top 100 holders = Senators, Top 500 = Representatives, Dev = President

use std::collections::HashMap;

/// SOLFUNMEME holder with government role
#[derive(Debug, Clone)]
struct SolfunmemeHolder {
    address: String,
    token_balance: u64,
    voting_power: f64,
    government_role: GovernmentRole,
}

#[derive(Debug, Clone, PartialEq)]
enum GovernmentRole {
    President,
    Senator,
    Representative, 
    Citizen,
}

/// Automorphism bill for congressional voting
#[derive(Debug, Clone)]
struct AutomorphismBill {
    bill_id: u32,
    title: String,
    source_meme: String,
    target_meme: String,
    structure_preserved: bool,
    senate_votes_for: u32,
    senate_votes_against: u32,
    house_votes_for: u32,
    house_votes_against: u32,
    presidential_action: PresidentialAction,
    status: BillStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum PresidentialAction {
    Pending,
    Signed,
    Vetoed,
}

#[derive(Debug, Clone, PartialEq)]
enum BillStatus {
    InCommittee,
    PassedSenate,
    PassedHouse,
    PassedBoth,
    SignedIntoLaw,
    Vetoed,
    VetoOverridden,
}

/// SOLFUNMEME Federal Government System
struct SolfunmemeFederalGov {
    president: String,
    senators: Vec<String>,      // Top 100 holders
    representatives: Vec<String>, // Top 500 holders
    all_holders: HashMap<String, SolfunmemeHolder>,
    pending_bills: Vec<AutomorphismBill>,
    laws: Vec<AutomorphismBill>,
    next_bill_id: u32,
}

impl SolfunmemeFederalGov {
    fn new(president_address: &str) -> Self {
        Self {
            president: president_address.to_string(),
            senators: Vec::new(),
            representatives: Vec::new(),
            all_holders: HashMap::new(),
            pending_bills: Vec::new(),
            laws: Vec::new(),
            next_bill_id: 1,
        }
    }
    
    /// Add holder and assign government role based on token balance
    fn add_holder(&mut self, address: &str, balance: u64) {
        let role = if address == self.president {
            GovernmentRole::President
        } else {
            GovernmentRole::Citizen
        };
        
        let holder = SolfunmemeHolder {
            address: address.to_string(),
            token_balance: balance,
            voting_power: (balance as f64).sqrt(),
            government_role: role,
        };
        
        self.all_holders.insert(address.to_string(), holder);
    }
    
    /// Assign Senate and House roles based on token holdings
    fn assign_government_roles(&mut self) {
        // Collect addresses and balances (excluding president)
        let mut holder_data: Vec<(String, u64)> = self.all_holders.iter()
            .filter(|(addr, _)| *addr != &self.president)
            .map(|(addr, holder)| (addr.clone(), holder.token_balance))
            .collect();
        
        // Sort by balance descending
        holder_data.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Clear existing roles
        self.senators.clear();
        self.representatives.clear();
        
        // Assign roles
        for (i, (address, _)) in holder_data.iter().enumerate() {
            if i < 100 {
                self.senators.push(address.clone());
                if let Some(holder) = self.all_holders.get_mut(address) {
                    holder.government_role = GovernmentRole::Senator;
                }
            } else if i < 500 {
                self.representatives.push(address.clone());
                if let Some(holder) = self.all_holders.get_mut(address) {
                    holder.government_role = GovernmentRole::Representative;
                }
            }
        }
    }
    
    /// Submit automorphism bill to Congress
    fn submit_bill(&mut self, title: &str, source: &str, target: &str, structure_preserved: bool) -> u32 {
        let bill = AutomorphismBill {
            bill_id: self.next_bill_id,
            title: title.to_string(),
            source_meme: source.to_string(),
            target_meme: target.to_string(),
            structure_preserved,
            senate_votes_for: 0,
            senate_votes_against: 0,
            house_votes_for: 0,
            house_votes_against: 0,
            presidential_action: PresidentialAction::Pending,
            status: BillStatus::InCommittee,
        };
        
        let bill_id = self.next_bill_id;
        self.next_bill_id += 1;
        self.pending_bills.push(bill);
        bill_id
    }
    
    /// Presidential action on bill (dev has veto power)
    fn presidential_action(&mut self, bill_id: u32, action: PresidentialAction) -> Result<(), String> {
        if let Some(bill) = self.pending_bills.iter_mut().find(|b| b.bill_id == bill_id) {
            bill.presidential_action = action.clone();
            
            match action {
                PresidentialAction::Signed => {
                    bill.status = BillStatus::SignedIntoLaw;
                }
                PresidentialAction::Vetoed => {
                    bill.status = BillStatus::Vetoed;
                }
                _ => {}
            }
            Ok(())
        } else {
            Err("Bill not found".to_string())
        }
    }
}

#[derive(Debug)]
struct GovernmentStats {
    total_holders: usize,
    senators: usize,
    representatives: usize,
    pending_bills: usize,
    laws_enacted: usize,
}

fn main() {
    println!("🏛️  SOLFUNMEME FEDERAL GOVERNMENT");
    println!("US Federal Model: Senate (Top 100) + House (Top 500) + Presidential Veto");
    
    let mut gov = SolfunmemeFederalGov::new("dev.sol");
    
    // Add the dev/president
    gov.add_holder("dev.sol", 500_000_000); // 500M tokens (dev allocation)
    
    // Add major holders (would be senators)
    gov.add_holder("whale1.sol", 100_000_000);
    gov.add_holder("whale2.sol", 75_000_000);
    gov.add_holder("whale3.sol", 50_000_000);
    
    // Add medium holders (would be representatives)
    for i in 4..=20 {
        gov.add_holder(&format!("holder{}.sol", i), 10_000_000 - (i * 100_000));
    }
    
    // Assign government roles
    gov.assign_government_roles();
    
    println!("\n🏛️  GOVERNMENT COMPOSITION:");
    println!("  President: dev.sol (500M tokens)");
    println!("  Senators: {} (top holders)", gov.senators.len());
    println!("  Representatives: {} (medium holders)", gov.representatives.len());
    println!("  Total holders: {}", gov.all_holders.len());
    
    // Submit automorphism bills
    println!("\n📜 SUBMITTING BILLS TO CONGRESS:");
    
    let bill1 = gov.submit_bill(
        "Lambda Rotation Act", 
        "🌀", "🌀'", 
        true // Structure preserved
    );
    println!("  Bill {}: Lambda Rotation Act (automorphic)", bill1);
    
    let bill2 = gov.submit_bill(
        "Emoji Scaling Act", 
        "🎭", "🎭'", 
        false // Structure NOT preserved
    );
    println!("  Bill {}: Emoji Scaling Act (non-automorphic)", bill2);
    
    // Presidential action
    println!("\n🖋️  PRESIDENTIAL ACTION:");
    gov.presidential_action(bill1, PresidentialAction::Signed).unwrap();
    println!("  Bill 1: SIGNED by President (dev.sol)");
    
    gov.presidential_action(bill2, PresidentialAction::Vetoed).unwrap();
    println!("  Bill 2: VETOED by President (dev.sol)");
    
    println!("\n📊 FINAL RESULTS:");
    println!("  Pending bills: {}", gov.pending_bills.len());
    println!("  Bills signed: {}", gov.pending_bills.iter().filter(|b| b.status == BillStatus::SignedIntoLaw).count());
    println!("  Bills vetoed: {}", gov.pending_bills.iter().filter(|b| b.status == BillStatus::Vetoed).count());
    
    println!("\n⚖️  SIGNED BILLS:");
    for bill in &gov.pending_bills {
        if bill.status == BillStatus::SignedIntoLaw {
            println!("  Law {}: {} - {} → {}", 
                     bill.bill_id, bill.title, bill.source_meme, bill.target_meme);
        }
    }
    
    println!("\n❌ VETOED BILLS:");
    for bill in &gov.pending_bills {
        if bill.status == BillStatus::Vetoed {
            println!("  Vetoed {}: {} - {} → {}", 
                     bill.bill_id, bill.title, bill.source_meme, bill.target_meme);
        }
    }
    
    println!("\n{}", "=".repeat(60));
    println!("🏛️  SOLFUNMEME FEDERAL GOVERNMENT COMPLETE:");
    println!("🎩 President (Dev): Veto power over all bills");
    println!("🏛️  Senate: Top 100 token holders");
    println!("🏢 House: Top 500 token holders");
    println!("⚖️  Democratic governance with presidential veto!");
    println!("{}", "=".repeat(60));
}
