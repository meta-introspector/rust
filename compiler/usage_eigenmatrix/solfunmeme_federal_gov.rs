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
            GovernmentRole::Citizen // Will be updated in assign_roles
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
        // Sort holders by balance (excluding president)
        let mut sorted_holders: Vec<_> = self.all_holders.iter()
            .filter(|(addr, _)| *addr != &self.president)
            .collect();
        sorted_holders.sort_by(|a, b| b.1.token_balance.cmp(&a.1.token_balance));
        
        // Clear existing roles
        self.senators.clear();
        self.representatives.clear();
        
        // Assign Senate (top 100)
        for (i, (address, _)) in sorted_holders.iter().enumerate() {
            if i < 100 {
                self.senators.push(address.to_string());
                if let Some(holder) = self.all_holders.get_mut(*address) {
                    holder.government_role = GovernmentRole::Senator;
                }
            } else if i < 500 {
                self.representatives.push(address.to_string());
                if let Some(holder) = self.all_holders.get_mut(*address) {
                    holder.government_role = GovernmentRole::Representative;
                }
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
    
    /// Senate vote on bill
    fn senate_vote(&mut self, bill_id: u32, senator_address: &str, vote_for: bool) -> Result<(), String> {
        if !self.senators.contains(&senator_address.to_string()) {
            return Err("Not a senator".to_string());
        }
        
        if let Some(bill) = self.pending_bills.iter_mut().find(|b| b.bill_id == bill_id) {
            if let Some(holder) = self.all_holders.get(senator_address) {
                let voting_power = holder.voting_power as u32;
                if vote_for {
                    bill.senate_votes_for += voting_power;
                } else {
                    bill.senate_votes_against += voting_power;
                }
            }
            Ok(())
        } else {
            Err("Bill not found".to_string())
        }
    }
    
    /// House vote on bill
    fn house_vote(&mut self, bill_id: u32, rep_address: &str, vote_for: bool) -> Result<(), String> {
        if !self.representatives.contains(&rep_address.to_string()) {
            return Err("Not a representative".to_string());
        }
        
        if let Some(bill) = self.pending_bills.iter_mut().find(|b| b.bill_id == bill_id) {
            if let Some(holder) = self.all_holders.get(rep_address) {
                let voting_power = holder.voting_power as u32;
                if vote_for {
                    bill.house_votes_for += voting_power;
                } else {
                    bill.house_votes_against += voting_power;
                }
            }
            Ok(())
        } else {
            Err("Bill not found".to_string())
        }
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
    
    /// Process bills through congressional procedure
    fn process_bills(&mut self) {
        for bill in &mut self.pending_bills {
            // Check if bill passes Senate (simple majority)
            let senate_total = bill.senate_votes_for + bill.senate_votes_against;
            let senate_passed = senate_total > 0 && bill.senate_votes_for > bill.senate_votes_against;
            
            // Check if bill passes House (simple majority)
            let house_total = bill.house_votes_for + bill.house_votes_against;
            let house_passed = house_total > 0 && bill.house_votes_for > bill.house_votes_against;
            
            // Update bill status
            if senate_passed && house_passed {
                bill.status = BillStatus::PassedBoth;
            } else if senate_passed {
                bill.status = BillStatus::PassedSenate;
            } else if house_passed {
                bill.status = BillStatus::PassedHouse;
            }
        }
        
        // Move signed bills to laws
        let mut signed_indices = Vec::new();
        for (i, bill) in self.pending_bills.iter().enumerate() {
            if bill.status == BillStatus::SignedIntoLaw {
                signed_indices.push(i);
            }
        }
        
        for &i in signed_indices.iter().rev() {
            let law = self.pending_bills.remove(i);
            self.laws.push(law);
        }
    }
    
    fn get_government_stats(&self) -> GovernmentStats {
        GovernmentStats {
            total_holders: self.all_holders.len(),
            senators: self.senators.len(),
            representatives: self.representatives.len(),
            pending_bills: self.pending_bills.len(),
            laws_enacted: self.laws.len(),
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

/// Demonstrate SOLFUNMEME Federal Government
fn demonstrate_federal_government() {
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
    
    let stats = gov.get_government_stats();
    println!("\n🏛️  GOVERNMENT COMPOSITION:");
    println!("  President: dev.sol (500M tokens)");
    println!("  Senators: {} (top 100 holders)", stats.senators);
    println!("  Representatives: {} (top 500 holders)", stats.representatives);
    println!("  Total holders: {}", stats.total_holders);
    
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
    
    // Congressional voting
    println!("\n🗳️  CONGRESSIONAL VOTING:");
    
    // Senate votes on Bill 1 (Lambda Rotation)
    gov.senate_vote(bill1, "whale1.sol", true).unwrap();
    gov.senate_vote(bill1, "whale2.sol", true).unwrap();
    gov.senate_vote(bill1, "whale3.sol", true).unwrap();
    
    // House votes on Bill 1
    gov.house_vote(bill1, "holder4.sol", true).unwrap();
    gov.house_vote(bill1, "holder5.sol", true).unwrap();
    
    // Senate votes on Bill 2 (Emoji Scaling) - rejected
    gov.senate_vote(bill2, "whale1.sol", false).unwrap();
    gov.senate_vote(bill2, "whale2.sol", false).unwrap();
    
    println!("  Congressional votes cast on both bills");
    
    // Process bills through Congress
    gov.process_bills();
    
    // Presidential action
    println!("\n🖋️  PRESIDENTIAL ACTION:");
    gov.presidential_action(bill1, PresidentialAction::Signed).unwrap();
    println!("  Bill 1: SIGNED by President (dev.sol)");
    
    gov.presidential_action(bill2, PresidentialAction::Vetoed).unwrap();
    println!("  Bill 2: VETOED by President (dev.sol)");
    
    // Final processing
    gov.process_bills();
    
    let final_stats = gov.get_government_stats();
    println!("\n📊 FINAL RESULTS:");
    println!("  Pending bills: {}", final_stats.pending_bills);
    println!("  Laws enacted: {}", final_stats.laws_enacted);
    
    println!("\n⚖️  ENACTED LAWS:");
    for law in &gov.laws {
        println!("  Law {}: {} - {} → {}", 
                 law.bill_id, law.title, law.source_meme, law.target_meme);
    }
}

fn main() {
    demonstrate_federal_government();
    
    println!("\n{}", "=".repeat(60));
    println!("🏛️  SOLFUNMEME FEDERAL GOVERNMENT COMPLETE:");
    println!("🎩 President (Dev): Veto power over all bills");
    println!("🏛️  Senate: Top 100 token holders");
    println!("🏢 House: Top 500 token holders");
    println!("⚖️  Democratic governance of automorphic properties!");
    println!("{}", "=".repeat(60));
}
