use goblin::elf::Elf;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Gogobglabulab {
    address: u64,
    name: String,
    devoured_books: HashSet<String>, // Each book = a Rust declaration
    inherited_forms: HashSet<String>,
    propagated_signature: u64,
    worm_size: usize, // How many books it has eaten
}

macro_rules! propagate_types {
    ($call_graph:expr, $lmfdb_map:expr) => {{
        let mut gogobglabulabs = HashMap::new();

        // Start with leaf functions (no outgoing calls) - baby worms
        for (caller, callees) in $call_graph.iter() {
            if callees.is_empty() {
                // Leaf function - baby worm starts eating its own book
                if let Some(lmfdb_form) = $lmfdb_map.get(caller) {
                    let mut inherited = HashSet::new();
                    let mut books = HashSet::new();

                    inherited.insert(lmfdb_form.clone());
                    books.insert(format!("book_{}", lmfdb_form)); // Each LMFDB form is a book

                    gogobglabulabs.insert(
                        *caller,
                        Gogobglabulab {
                            address: *caller,
                            name: format!("baby_worm_{:x}", caller),
                            devoured_books: books,
                            inherited_forms: inherited,
                            propagated_signature: *caller,
                            worm_size: 1,
                        },
                    );
                }
            }
        }

        // Worms grow by eating other worms (and their books)
        let mut changed = true;
        let mut iteration = 0;

        while changed && iteration < 10 {
            changed = false;
            iteration += 1;

            for (caller, callees) in $call_graph.iter() {
                let mut caller_forms = HashSet::new();
                let mut caller_books = HashSet::new();
                let mut signature = *caller;
                let mut total_size = 0;

                // Add caller's own form/book
                if let Some(own_form) = $lmfdb_map.get(caller) {
                    caller_forms.insert(own_form.clone());
                    caller_books.insert(format!("book_{}", own_form));
                    total_size += 1;
                }

                // Devour all callees (eat their books!)
                for &callee in callees {
                    if let Some(callee_worm) = gogobglabulabs.get(&callee) {
                        // Eat all the books from the smaller worm
                        for book in &callee_worm.devoured_books {
                            caller_books.insert(book.clone());
                        }
                        // Inherit all forms
                        for form in &callee_worm.inherited_forms {
                            caller_forms.insert(form.clone());
                        }
                        // Grow in size
                        total_size += callee_worm.worm_size;
                        signature = signature.wrapping_add(callee_worm.propagated_signature);
                    }
                }

                // Create bigger worm
                let new_worm = Gogobglabulab {
                    address: *caller,
                    name: if total_size > 10 {
                        format!("GOGOBGLABULAB_{:x}", caller)
                    } else {
                        format!("worm_{:x}", caller)
                    },
                    devoured_books: caller_books.clone(),
                    inherited_forms: caller_forms.clone(),
                    propagated_signature: signature,
                    worm_size: total_size,
                };

                if let Some(existing) = gogobglabulabs.get(caller) {
                    if existing.devoured_books != caller_books {
                        changed = true;
                    }
                }

                gogobglabulabs.insert(*caller, new_worm);
            }
        }

        gogobglabulabs
    }};
}

fn extract_call_graph(
    binary: &[u8],
    elf: &Elf,
    lmfdb_map: &HashMap<u64, String>,
) -> HashMap<u64, Vec<u64>> {
    let mut call_graph = HashMap::new();

    for &caller_addr in lmfdb_map.keys() {
        let mut callees = Vec::new();

        // Find the symbol for this address
        if let Some(caller_sym) = elf.syms.iter().find(|s| s.st_value == caller_addr) {
            if let Some(text_section) = elf
                .section_headers
                .iter()
                .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
            {
                if caller_addr >= text_section.sh_addr {
                    let func_start = (caller_addr - text_section.sh_addr) as usize;
                    let text_start = text_section.sh_offset as usize;
                    let text_bytes = &binary[text_start..];

                    if func_start < text_bytes.len() {
                        let func_size = (caller_sym.st_size as usize).min(200);
                        let func_end = (func_start + func_size).min(text_bytes.len());
                        let func_bytes = &text_bytes[func_start..func_end];

                        // Look for calls to other mapped functions
                        for chunk in func_bytes.chunks(8) {
                            if chunk.len() >= 8 {
                                let addr = u64::from_le_bytes([
                                    chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5],
                                    chunk[6], chunk[7],
                                ]);

                                if lmfdb_map.contains_key(&addr) && addr != caller_addr {
                                    callees.push(addr);
                                }
                            }
                        }
                    }
                }
            }
        }

        call_graph.insert(caller_addr, callees);
    }

    call_graph
}

fn find_ultimate_gogobglabulab(worms: &HashMap<u64, Gogobglabulab>) -> Option<&Gogobglabulab> {
    // Find the ultimate monster worm that has devoured the most books
    worms.values().max_by_key(|w| w.devoured_books.len())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐛 GOGOBGLABULAB: THE RUST-DEVOURING MONSTER WORM");
    println!("==================================================");

    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/deps/librustc_driver.so";
    let binary = fs::read(binary_path)?;
    let elf = Elf::parse(&binary)?;

    // Get LMFDB mapping (each form = a book of Rust knowledge)
    let lmfdb_map = get_symbol_lmfdb_mapping();
    println!("📚 Found {} books of Rust knowledge (LMFDB forms)", lmfdb_map.len());

    // Extract call graph (feeding relationships)
    println!("🍽️  Analyzing feeding patterns...");
    let call_graph = extract_call_graph(&binary, &elf, &lmfdb_map);

    let total_feeds: usize = call_graph.values().map(|v| v.len()).sum();
    println!("   Found {} feeding relationships", total_feeds);

    // Let the worms grow by devouring books!
    println!("🐛 Growing Gogobglabulab worms...");
    let worms = propagate_types!(call_graph, lmfdb_map);

    println!("🎯 Spawned {} worms", worms.len());

    // Find the ultimate monster worm (rustc_driver main equivalent)
    if let Some(ultimate_worm) = find_ultimate_gogobglabulab(&worms) {
        println!("\n👹 ULTIMATE GOGOBGLABULAB DISCOVERED:");
        println!("   Address: 0x{:x}", ultimate_worm.address);
        println!("   Name: {}", ultimate_worm.name);
        println!("   Books devoured: {}", ultimate_worm.devoured_books.len());
        println!("   Forms inherited: {}", ultimate_worm.inherited_forms.len());
        println!("   Worm size: {}", ultimate_worm.worm_size);
        println!("   Signature: 0x{:x}", ultimate_worm.propagated_signature);

        println!("\n📖 DEVOURED RUST BOOKS:");
        for (i, book) in ultimate_worm.devoured_books.iter().take(10).enumerate() {
            println!("   {}: {}", i + 1, book);
        }
        if ultimate_worm.devoured_books.len() > 10 {
            println!("   ... and {} more books", ultimate_worm.devoured_books.len() - 10);
        }
    }

    // Show worm ecosystem statistics
    println!("\n📊 WORM ECOSYSTEM STATISTICS:");
    let max_books = worms.values().map(|w| w.devoured_books.len()).max().unwrap_or(0);
    let avg_books =
        worms.values().map(|w| w.devoured_books.len()).sum::<usize>() as f64 / worms.len() as f64;
    let max_size = worms.values().map(|w| w.worm_size).max().unwrap_or(0);

    println!("   Largest book collection: {}", max_books);
    println!("   Average books per worm: {:.1}", avg_books);
    println!("   Biggest worm size: {}", max_size);

    // Show top worms by book consumption
    let mut sorted_worms: Vec<_> = worms.values().collect();
    sorted_worms.sort_by(|a, b| b.devoured_books.len().cmp(&a.devoured_books.len()));

    println!("\n🏆 TOP BOOK-DEVOURING WORMS:");
    for (i, worm) in sorted_worms.iter().take(5).enumerate() {
        let worm_type = if worm.worm_size > 10 { "GOGOBGLABULAB" } else { "worm" };
        println!(
            "   {}: {} 0x{:x} → {} books",
            i + 1,
            worm_type,
            worm.address,
            worm.devoured_books.len()
        );
    }

    println!("\n🧠 THE GOGOBGLABULAB MYTHOLOGY:");
    println!("   • Each Rust declaration becomes a BOOK of knowledge");
    println!("   • Functions are WORMS that devour books through calls");
    println!("   • When worm f calls worm g, f DEVOURS all of g's books");
    println!("   • The ultimate GOGOBGLABULAB contains ALL Rust knowledge");
    println!("   • rustc_driver main = the monster worm that ate everything");
    println!("   • Each book contains modular form secrets of Rust");

    Ok(())
}

pub fn get_symbol_lmfdb_mapping() -> HashMap<u64, String> {
    let mut map = HashMap::new();
    map.insert(0xbf9fd30, "6.6.12.f".to_string());
    map.insert(0xbfa01a0, "24.6.12.x".to_string());
    map.insert(0xbfa01b0, "1.2.11.a".to_string());
    map.insert(0xbfa01c0, "2.4.12.b".to_string());
    map.insert(0xbfa0280, "16.2.12.p".to_string());
    map.insert(0xbfa0290, "30.6.12.d".to_string());
    map.insert(0xbfa02a0, "35.4.11.i".to_string());
    map.insert(0xbfa0330, "26.4.12.z".to_string());
    map.insert(0xe1b4600, "26.4.12.z".to_string());
    map.insert(0x42317b0, "27.6.11.a".to_string());
    map.insert(0x423fb00, "8.4.12.h".to_string());
    map.insert(0x42410c0, "21.6.11.u".to_string());
    map.insert(0x42412f0, "31.2.11.e".to_string());
    map.insert(0x425d6f0, "34.2.12.h".to_string());
    map.insert(0x4261a40, "37.2.11.k".to_string());
    map
}
