use libc::{PTRACE_ATTACH, PTRACE_CONT, PTRACE_DETACH, PTRACE_PEEKTEXT, PTRACE_POKETEXT, ptrace};
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 PTRACE PARSER FUNCTION INTERCEPTOR");
    println!("====================================");

    // Load target parser functions from rustc_addresses.rs
    let parser_functions = load_parser_functions()?;
    println!("📋 Loaded {} parser functions to intercept", parser_functions.len());

    // Start rustc process
    let mut rustc_process = Command::new("rustc")
        .arg("test_parse.rs")
        .arg("-o")
        .arg("test_output")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let pid = rustc_process.id() as i32;
    println!("🎯 Attached to rustc process PID: {}", pid);

    // Attach ptrace
    unsafe {
        if ptrace(
            PTRACE_ATTACH,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        ) == -1
        {
            return Err("Failed to attach ptrace".into());
        }
    }

    // Wait for process to stop
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Install breakpoints on parser functions
    let mut breakpoints = HashMap::new();
    for (name, address) in &parser_functions {
        if let Ok(original_instruction) = install_breakpoint(pid, *address) {
            breakpoints.insert(*address, (name.clone(), original_instruction));
            println!("🔧 Breakpoint installed: {} at 0x{:x}", name, address);
        }
    }

    println!("🚀 Starting execution with {} breakpoints...", breakpoints.len());

    // Continue execution
    unsafe {
        ptrace(
            PTRACE_CONT,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        );
    }

    // Monitor for breakpoint hits
    let mut hit_count = 0;
    loop {
        let status = wait_for_signal(pid)?;

        if status == 5 {
            // SIGTRAP - breakpoint hit
            let pc = get_program_counter(pid)?;

            if let Some((func_name, original_instruction)) = breakpoints.get(&pc) {
                hit_count += 1;
                println!("🎯 HIT #{}: {} at 0x{:x}", hit_count, func_name, pc);

                // Dump parser data
                dump_parser_data(pid, func_name, pc)?;

                // Restore original instruction temporarily
                restore_instruction(pid, pc, *original_instruction)?;

                // Single step past the instruction
                single_step(pid)?;

                // Reinstall breakpoint
                install_breakpoint(pid, pc)?;
            }

            // Continue execution
            unsafe {
                ptrace(
                    PTRACE_CONT,
                    pid,
                    std::ptr::null_mut::<libc::c_void>(),
                    std::ptr::null_mut::<libc::c_void>(),
                );
            }
        } else {
            // Process exited
            break;
        }
    }

    println!("✅ Intercepted {} parser function calls", hit_count);

    // Detach and cleanup
    unsafe {
        ptrace(
            PTRACE_DETACH,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        );
    }

    let _ = rustc_process.wait();
    Ok(())
}

fn load_parser_functions() -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
    let mut functions = HashMap::new();

    // Key parser functions from our analysis
    functions.insert("parse_crate_attrs".to_string(), 0x217b7d38e290cad0); // From rustc_addresses.rs
    functions.insert("rustc_parse_file".to_string(), 0x4231b30); // Example address
    functions.insert("syn_parse_expr".to_string(), 0x4231b70); // Example address

    // Load more from rustc_addresses.rs if available
    if let Ok(content) = fs::read_to_string("rustc_addresses.rs") {
        for line in content.lines() {
            if line.contains("parse") && line.contains("0x") {
                // Parse address from enum definition
                if let Some(addr_str) = extract_hex_address(&line) {
                    if let Ok(addr) = u64::from_str_radix(&addr_str, 16) {
                        let func_name =
                            extract_function_name(&line).unwrap_or("unknown".to_string());
                        functions.insert(func_name, addr);
                    }
                }
            }
        }
    }

    Ok(functions)
}

fn install_breakpoint(pid: i32, address: u64) -> Result<u64, Box<dyn std::error::Error>> {
    unsafe {
        // Read original instruction
        let original = ptrace(
            PTRACE_PEEKTEXT,
            pid,
            address as *mut libc::c_void,
            std::ptr::null_mut::<libc::c_void>(),
        );
        if original == -1 {
            return Err("Failed to read memory".into());
        }

        // Install INT3 breakpoint (0xCC)
        let breakpoint = (original & !0xFF) | 0xCC;
        if ptrace(
            PTRACE_POKETEXT,
            pid,
            address as *mut libc::c_void,
            breakpoint as *mut libc::c_void,
        ) == -1
        {
            return Err("Failed to install breakpoint".into());
        }

        Ok(original as u64)
    }
}

fn restore_instruction(
    pid: i32,
    address: u64,
    original: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if ptrace(PTRACE_POKETEXT, pid, address as *mut libc::c_void, original as *mut libc::c_void)
            == -1
        {
            return Err("Failed to restore instruction".into());
        }
    }
    Ok(())
}

fn dump_parser_data(
    pid: i32,
    func_name: &str,
    address: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 PARSER DATA DUMP: {}", func_name);

    // Read registers to get function arguments
    let registers = read_registers(pid)?;

    // Dump function arguments (first 6 args in RDI, RSI, RDX, RCX, R8, R9)
    println!("   RDI (arg1): 0x{:x}", registers.rdi);
    println!("   RSI (arg2): 0x{:x}", registers.rsi);
    println!("   RDX (arg3): 0x{:x}", registers.rdx);

    // Try to read string data from arguments
    if let Ok(string_data) = read_string_from_memory(pid, registers.rdi) {
        println!("   String arg1: {:?}", string_data);
    }

    // Dump stack data
    dump_stack_data(pid, registers.rsp)?;

    // Save to file
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();

    let dump_file = format!("parser_dump_{}_{}.json", func_name, timestamp);
    let dump_data = serde_json::json!({
        "function": func_name,
        "address": format!("0x{:x}", address),
        "timestamp": timestamp,
        "registers": {
            "rdi": format!("0x{:x}", registers.rdi),
            "rsi": format!("0x{:x}", registers.rsi),
            "rdx": format!("0x{:x}", registers.rdx),
            "rsp": format!("0x{:x}", registers.rsp),
        }
    });

    fs::write(&dump_file, dump_data.to_string())?;
    println!("   💾 Saved to: {}", dump_file);

    Ok(())
}

#[repr(C)]
struct UserRegs {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbp: u64,
    rbx: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rax: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    orig_rax: u64,
    rip: u64,
    cs: u64,
    eflags: u64,
    rsp: u64,
    ss: u64,
}

fn read_registers(pid: i32) -> Result<UserRegs, Box<dyn std::error::Error>> {
    let mut regs: UserRegs = unsafe { std::mem::zeroed() };
    unsafe {
        if ptrace(
            libc::PTRACE_GETREGS,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            &mut regs as *mut _ as *mut libc::c_void,
        ) == -1
        {
            return Err("Failed to read registers".into());
        }
    }
    Ok(regs)
}

fn read_string_from_memory(pid: i32, address: u64) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();
    let mut addr = address;

    for _ in 0..256 {
        // Max 256 chars
        unsafe {
            let word = ptrace(
                PTRACE_PEEKTEXT,
                pid,
                addr as *mut libc::c_void,
                std::ptr::null_mut::<libc::c_void>(),
            );
            if word == -1 {
                break;
            }

            let bytes = word.to_le_bytes();
            for &byte in &bytes {
                if byte == 0 {
                    return Ok(result);
                }
                if byte.is_ascii() {
                    result.push(byte as char);
                } else {
                    break;
                }
            }
            addr += 8;
        }
    }

    Ok(result)
}

fn dump_stack_data(pid: i32, rsp: u64) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Stack dump (16 words):");
    for i in 0..16 {
        let addr = rsp + (i * 8);
        unsafe {
            let word = ptrace(
                PTRACE_PEEKTEXT,
                pid,
                addr as *mut libc::c_void,
                std::ptr::null_mut::<libc::c_void>(),
            );
            if word != -1 {
                println!("     [RSP+{}]: 0x{:016x}", i * 8, word);
            }
        }
    }
    Ok(())
}

fn wait_for_signal(pid: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut status: i32 = 0;
    unsafe {
        libc::waitpid(pid, &mut status, 0);
    }
    Ok(libc::WSTOPSIG(status))
}

fn single_step(pid: i32) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if ptrace(
            libc::PTRACE_SINGLESTEP,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        ) == -1
        {
            return Err("Failed to single step".into());
        }
    }

    let mut status: i32 = 0;
    unsafe {
        libc::waitpid(pid, &mut status, 0);
    }
    Ok(())
}

fn get_program_counter(pid: i32) -> Result<u64, Box<dyn std::error::Error>> {
    let regs = read_registers(pid)?;
    Ok(regs.rip)
}

fn extract_hex_address(line: &str) -> Option<String> {
    if let Some(start) = line.find("0x") {
        let addr_part = &line[start + 2..];
        if let Some(end) = addr_part.find(|c: char| !c.is_ascii_hexdigit()) {
            Some(addr_part[..end].to_string())
        } else {
            Some(addr_part.to_string())
        }
    } else {
        None
    }
}

fn extract_function_name(line: &str) -> Option<String> {
    if let Some(start) = line.find("parse") {
        let name_part = &line[start..];
        if let Some(end) = name_part.find(|c: char| c.is_whitespace() || c == '=' || c == ',') {
            Some(name_part[..end].to_string())
        } else {
            Some(name_part.to_string())
        }
    } else {
        None
    }
}
