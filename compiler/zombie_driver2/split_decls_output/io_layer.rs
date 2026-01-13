// Split-Decls Layer: IO
// IO Signature: input → output
// Generated from: .

// function: parse_modular_key
fn parse_modular_key (key : & str) -> (u32 , u32) { let parts : Vec < & str > = key . split ('.') . collect () ; if parts . len () >= 2 { (parts [0] . parse () . unwrap_or (1) , parts [1] . parse () . unwrap_or (2)) } else { (1 , 2) } }

// function: load_parser_functions
fn load_parser_functions () -> Result < HashMap < String , u64 > , Box < dyn std :: error :: Error > > { let mut functions = HashMap :: new () ; functions . insert ("parse_crate_attrs" . to_string () , 0x217b7d38e290cad0) ; functions . insert ("rustc_parse_file" . to_string () , 0x4231b30) ; functions . insert ("syn_parse_expr" . to_string () , 0x4231b70) ; if let Ok (content) = fs :: read_to_string ("rustc_addresses.rs") { for line in content . lines () { if line . contains ("parse") && line . contains ("0x") { if let Some (addr_str) = extract_hex_address (& line) { if let Ok (addr) = u64 :: from_str_radix (& addr_str , 16) { let func_name = extract_function_name (& line) . unwrap_or ("unknown" . to_string ()) ; functions . insert (func_name , addr) ; } } } } } Ok (functions) }

// function: dump_parser_data
fn dump_parser_data (pid : i32 , func_name : & str , address : u64 ,) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("📊 PARSER DATA DUMP: {}" , func_name) ; let registers = read_registers (pid) ? ; println ! ("   RDI (arg1): 0x{:x}" , registers . rdi) ; println ! ("   RSI (arg2): 0x{:x}" , registers . rsi) ; println ! ("   RDX (arg3): 0x{:x}" , registers . rdx) ; if let Ok (string_data) = read_string_from_memory (pid , registers . rdi) { println ! ("   String arg1: {:?}" , string_data) ; } dump_stack_data (pid , registers . rsp) ? ; let timestamp = std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) ? . as_secs () ; let dump_file = format ! ("parser_dump_{}_{}.json" , func_name , timestamp) ; let dump_data = serde_json :: json ! ({ "function" : func_name , "address" : format ! ("0x{:x}" , address) , "timestamp" : timestamp , "registers" : { "rdi" : format ! ("0x{:x}" , registers . rdi) , "rsi" : format ! ("0x{:x}" , registers . rsi) , "rdx" : format ! ("0x{:x}" , registers . rdx) , "rsp" : format ! ("0x{:x}" , registers . rsp) , } }) ; fs :: write (& dump_file , dump_data . to_string ()) ? ; println ! ("   💾 Saved to: {}" , dump_file) ; Ok (()) }

// function: read_registers
fn read_registers (pid : i32) -> Result < UserRegs , Box < dyn std :: error :: Error > > { let mut regs : UserRegs = unsafe { std :: mem :: zeroed () } ; unsafe { if ptrace (libc :: PTRACE_GETREGS , pid , std :: ptr :: null_mut :: < libc :: c_void > () , & mut regs as * mut _ as * mut libc :: c_void ,) == - 1 { return Err ("Failed to read registers" . into ()) ; } } Ok (regs) }

// function: read_string_from_memory
fn read_string_from_memory (pid : i32 , address : u64) -> Result < String , Box < dyn std :: error :: Error > > { let mut result = String :: new () ; let mut addr = address ; for _ in 0 .. 256 { unsafe { let word = ptrace (PTRACE_PEEKTEXT , pid , addr as * mut libc :: c_void , std :: ptr :: null_mut :: < libc :: c_void > () ,) ; if word == - 1 { break ; } let bytes = word . to_le_bytes () ; for & byte in & bytes { if byte == 0 { return Ok (result) ; } if byte . is_ascii () { result . push (byte as char) ; } else { break ; } } addr += 8 ; } } Ok (result) }

// function: parse_modular_key
fn parse_modular_key (key : & str) -> (u32 , u32) { let parts : Vec < & str > = key . split ('.') . collect () ; if parts . len () >= 2 { (parts [0] . parse () . unwrap_or (1) , parts [1] . parse () . unwrap_or (2)) } else { (1 , 2) } }

