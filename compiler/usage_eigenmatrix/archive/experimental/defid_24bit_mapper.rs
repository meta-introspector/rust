/// 24-bit DefId Mapping Model with paged memory
/// Maps DefIds to 24-bit space using 4KB pages

const PAGE_SIZE: usize = 4096; // 4KB pages
const TOTAL_PAGES: usize = 4096; // 16MB total = 4096 pages
const RUSTC_24BIT_SPACE: usize = 0x1000000; // 2^24 = 16,777,216

struct DefId24BitMapper {
    // Paged memory - only allocate pages as needed
    pages: Vec<Option<Box<[u8; PAGE_SIZE]>>>,
}

impl DefId24BitMapper {
    fn new() -> Self {
        Self {
            pages: vec![None; TOTAL_PAGES],
        }
    }

    /// Map DefId to 24-bit space preserving numerical relationships
    fn map_defid_to_24bit(&self, defid_value: u64) -> u32 {
        match defid_value {
            0 => 0x000001,
            1 => 0x000002,
            2 => 0x000003,
            3 => 0x000005,
            4 => 0x000007,
            5 => 0x00000B,
            6 => 0x00000D,
            7 => 0x000011,
            8 => 0x000013,
            9 => 0x000017,
            _ => {
                let scaled = (defid_value % 0xFFFFF0) + 0x000020;
                scaled as u32 & 0xFFFFFF
            }
        }
    }

    /// Get page index and offset for 24-bit address
    fn get_page_offset(&self, addr: u32) -> (usize, usize) {
        let page_idx = (addr as usize) / PAGE_SIZE;
        let offset = (addr as usize) % PAGE_SIZE;
        (page_idx, offset)
    }

    /// Allocate page if not exists
    fn ensure_page(&mut self, page_idx: usize) {
        if page_idx < TOTAL_PAGES && self.pages[page_idx].is_none() {
            self.pages[page_idx] = Some(Box::new([0u8; PAGE_SIZE]));
        }
    }

    /// Set DefId at 24-bit location
    fn set_defid(&mut self, addr: u32, value: u8) {
        let (page_idx, offset) = self.get_page_offset(addr);
        self.ensure_page(page_idx);
        
        if let Some(ref mut page) = self.pages[page_idx] {
            page[offset] = value;
        }
    }

    /// Get DefId at 24-bit location  
    fn get_defid(&self, addr: u32) -> u8 {
        let (page_idx, offset) = self.get_page_offset(addr);
        
        if page_idx < TOTAL_PAGES {
            if let Some(ref page) = self.pages[page_idx] {
                return page[offset];
            }
        }
        0
    }

    /// Check if location is occupied
    fn is_occupied(&self, addr: u32) -> bool {
        self.get_defid(addr) != 0
    }
}

fn main() {
    let mut mapper = DefId24BitMapper::new();
    
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    
    println!("24-bit Rustc Space: {} locations ({} pages of {}KB)", 
        RUSTC_24BIT_SPACE, TOTAL_PAGES, PAGE_SIZE / 1024);
    println!("DefId -> 24-bit Cell Mapping:");
    
    // Initialize prime locations
    mapper.set_defid(0x000001, 1); // 0 (24% of rustc)
    
    for &prime in &primes {
        let cell = mapper.map_defid_to_24bit(prime);
        let occupied_before = mapper.is_occupied(cell);
        
        println!("const PRIME_{} = {}; -> Cell 0x{:06X} [{}]", 
            prime, prime, cell, if occupied_before { "OCCUPIED" } else { "FREE" });
        
        mapper.set_defid(cell, prime as u8);
        
        let (page_idx, offset) = mapper.get_page_offset(cell);
        println!("  -> Page {} offset 0x{:03X}", page_idx, offset);
    }
    
    println!("\nOccupied locations:");
    for addr in [0x000001, 0x000003, 0x000005, 0x00000B, 0x000011] {
        let value = mapper.get_defid(addr);
        if value != 0 {
            let (page_idx, offset) = mapper.get_page_offset(addr);
            println!("  [0x{:06X}] = {} (Page {} offset 0x{:03X})", 
                addr, value, page_idx, offset);
        }
    }
    
    // Count allocated pages
    let allocated_pages = mapper.pages.iter().filter(|p| p.is_some()).count();
    println!("\nPages allocated: {} / {} ({:.1}%)", 
        allocated_pages, TOTAL_PAGES, 
        (allocated_pages as f64 / TOTAL_PAGES as f64) * 100.0);
}
