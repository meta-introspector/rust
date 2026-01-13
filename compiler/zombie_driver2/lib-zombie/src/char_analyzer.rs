use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct CharTransition {
    from_char: char,
    to_char: char,
    probability: f64,
    frequency: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EssentialArrow {
    pub pair: (char, char),
    pub strength: f64,
    pub must_preserve: bool,
}

pub struct CharLevelAnalyzer {
    transition_matrix: HashMap<char, HashMap<char, u64>>,
    char_frequencies: HashMap<char, u64>,
    total_chars: u64,
    essential_arrows: Vec<EssentialArrow>,
}

impl CharLevelAnalyzer {
    pub fn new() -> Self {
        Self {
            transition_matrix: HashMap::new(),
            char_frequencies: HashMap::new(),
            total_chars: 0,
            essential_arrows: Vec::new(),
        }
    }

    pub fn analyze_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        println!("🔤 Analyzing character transitions in: {}", file_path);

        self.build_transition_matrix(&content);
        self.extract_essential_arrows();

        Ok(())
    }

    fn build_transition_matrix(&mut self, content: &str) {
        let chars: Vec<char> = content.chars().collect();
        self.total_chars = chars.len() as u64;

        // Count character frequencies
        for &ch in &chars {
            *self.char_frequencies.entry(ch).or_insert(0) += 1;
        }

        // Build transition matrix: char -> next_char -> count
        for window in chars.windows(2) {
            let from_char = window[0];
            let to_char = window[1];

            *self
                .transition_matrix
                .entry(from_char)
                .or_insert_with(HashMap::new)
                .entry(to_char)
                .or_insert(0) += 1;
        }

        println!(
            "📊 Built transition matrix: {} unique chars, {} transitions",
            self.char_frequencies.len(),
            self.transition_matrix.values().map(|m| m.len()).sum::<usize>()
        );
    }

    fn extract_essential_arrows(&mut self) {
        let mut all_transitions = Vec::new();

        // Convert transition matrix to probability transitions
        for (&from_char, transitions) in &self.transition_matrix {
            let from_total: u64 = transitions.values().sum();

            for (&to_char, &count) in transitions {
                let probability = count as f64 / from_total as f64;

                all_transitions.push(CharTransition {
                    from_char,
                    to_char,
                    probability,
                    frequency: count,
                });
            }
        }

        // Sort by frequency to find most common pairs
        all_transitions.sort_by(|a, b| b.frequency.cmp(&a.frequency));

        // Extract top frequent pairs as essential arrows
        let top_count = (all_transitions.len() / 10).max(20); // Top 10% or at least 20

        for transition in all_transitions.iter().take(top_count) {
            let strength = transition.probability * (transition.frequency as f64).log2();
            let must_preserve = transition.frequency > 10 && transition.probability > 0.1;

            self.essential_arrows.push(EssentialArrow {
                pair: (transition.from_char, transition.to_char),
                strength,
                must_preserve,
            });
        }

        println!(
            "🎯 Extracted {} essential arrows (must preserve: {})",
            self.essential_arrows.len(),
            self.essential_arrows.iter().filter(|a| a.must_preserve).count()
        );
    }

    pub fn predict_next_char(&self, current_char: char) -> Option<(char, f64)> {
        let transitions = self.transition_matrix.get(&current_char)?;
        let total: u64 = transitions.values().sum();

        // Find most likely next character
        let (&next_char, &count) = transitions.iter().max_by_key(|(_, &count)| count)?;

        let probability = count as f64 / total as f64;
        Some((next_char, probability))
    }

    pub fn get_essential_arrows(&self) -> &[EssentialArrow] {
        &self.essential_arrows
    }

    pub fn print_top_transitions(&self, limit: usize) {
        println!("\n🔝 Top {} character transitions:", limit);

        let mut sorted_arrows = self.essential_arrows.clone();
        sorted_arrows.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());

        for (i, arrow) in sorted_arrows.iter().take(limit).enumerate() {
            let preserve_mark = if arrow.must_preserve { "🔒" } else { "  " };
            println!(
                "{:2}. {} '{}' → '{}' (strength: {:.3})",
                i + 1,
                preserve_mark,
                Self::display_char(arrow.pair.0),
                Self::display_char(arrow.pair.1),
                arrow.strength
            );
        }
    }

    fn display_char(ch: char) -> String {
        match ch {
            ' ' => "SPC".to_string(),
            '\n' => "\\n".to_string(),
            '\t' => "\\t".to_string(),
            '\r' => "\\r".to_string(),
            c if c.is_control() => format!("\\x{:02x}", c as u8),
            c => c.to_string(),
        }
    }

    pub fn export_analysis(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct Analysis {
            char_frequencies: HashMap<char, u64>,
            essential_arrows: Vec<EssentialArrow>,
            total_chars: u64,
        }

        let analysis = Analysis {
            char_frequencies: self.char_frequencies.clone(),
            essential_arrows: self.essential_arrows.clone(),
            total_chars: self.total_chars,
        };

        let json = serde_json::to_string_pretty(&analysis)?;
        std::fs::write(path, json)?;
        println!("📁 Character analysis exported to {}", path);
        Ok(())
    }
}
