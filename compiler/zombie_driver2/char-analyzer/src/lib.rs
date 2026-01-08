use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CString, CStr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharAnalysis {
    pub file_path: String,
    pub char_count: u64,
    pub unique_chars: usize,
    pub char_frequencies: HashMap<char, u64>,
    pub essential_arrows: Vec<EssentialArrow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EssentialArrow {
    pub pair: (char, char),
    pub strength: f64,
    pub must_preserve: bool,
}

#[no_mangle]
pub extern "C" fn analyze_char_transitions(file_path: *const u8, file_path_len: usize, content: *const u8, content_len: usize) -> *mut u8 {
    let file_path = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(file_path, file_path_len)) };
    let content = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(content, content_len)) };
    
    match analyze_file(file_path, content) {
        Ok(analysis) => {
            let json = serde_json::to_string(&analysis).unwrap_or_default();
            let bytes = json.into_bytes();
            let ptr = bytes.as_ptr() as *mut u8;
            std::mem::forget(bytes);
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

pub fn analyze_file(file_path: &str, content: &str) -> Result<CharAnalysis, Box<dyn std::error::Error>> {
    let mut char_frequencies = HashMap::new();
    let mut transitions = HashMap::new();
    
    let chars: Vec<char> = content.chars().collect();
    
    // Count character frequencies
    for &ch in &chars {
        *char_frequencies.entry(ch).or_insert(0) += 1;
    }
    
    // Count character transitions
    for window in chars.windows(2) {
        let pair = (window[0], window[1]);
        *transitions.entry(pair).or_insert(0) += 1;
    }
    
    // Calculate essential arrows (transitions with strength > 1.0)
    let mut essential_arrows = Vec::new();
    for ((from, to), count) in transitions {
        let from_freq = char_frequencies.get(&from).unwrap_or(&1);
        let strength = count as f64 / *from_freq as f64;
        
        if strength > 1.0 {
            essential_arrows.push(EssentialArrow {
                pair: (from, to),
                strength,
                must_preserve: strength > 2.0,
            });
        }
    }
    
    essential_arrows.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());
    
    Ok(CharAnalysis {
        file_path: file_path.to_string(),
        char_count: chars.len() as u64,
        unique_chars: char_frequencies.len(),
        char_frequencies,
        essential_arrows,
    })
}
