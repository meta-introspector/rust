# Standard Operating Procedures: Zombie Rustc Driver

## SOP-001: Initial Environment Setup

### Prerequisites
- Rust toolchain (stable)
- Git version control
- Isolated development environment (VM/container recommended)
- sccache installed (`cargo install sccache`)

### Setup Steps
1. **Create Containment Directory:**
   ```bash
   mkdir -p /tmp/zombie_quarantine
   export ZOMBIE_QUARANTINE="/tmp/zombie_quarantine"
   ```

2. **Clone/Navigate to Project:**
   ```bash
   cd /path/to/zombie_driver
   ```

3. **Verify Dependencies:**
   ```bash
   cargo check
   ```

## SOP-002: Build Process

### Standard Build
```bash
# Set environment
export RUSTC_WRAPPER=""  # Disable wrapper during zombie build

# Clean build
cargo clean

# Build with sccache acceleration
RUSTC_WRAPPER=sccache cargo build

# Verify build artifacts
ls -la target/debug/zombie_rustc_driver
```

### Build Verification
```bash
# Test basic functionality
./target/debug/zombie_rustc_driver --help

# Verify zombie infection messages
echo 'fn main() {}' > test.rs
./target/debug/zombie_rustc_driver test.rs
rm test.rs
```

## SOP-003: Testing Procedures

### Unit Testing
```bash
# Run all tests
cargo test

# Run specific test module
cargo test zombie_callbacks
```

### Integration Testing
```bash
# Test network mode (safe - localhost only)
./target/debug/zombie_rustc_driver --network-mode &
ZOMBIE_PID=$!

# Verify network listening
netstat -ln | grep 4001

# Terminate test
kill $ZOMBIE_PID
```

### Compilation Testing
```bash
# Create test file
cat > safe_test.rs << 'EOF'
fn main() {
    println!("Hello zombie world!");
}
EOF

# Test compilation with zombie driver
./target/debug/zombie_rustc_driver safe_test.rs -o zombie_test

# Verify output
./zombie_test

# Cleanup
rm safe_test.rs zombie_test
```

## SOP-004: Deployment Procedures

### Local Development Deployment
```bash
# Copy to local bin (optional)
cp target/debug/zombie_rustc_driver ~/.local/bin/

# Create wrapper script
cat > ~/.local/bin/zombie-rustc << 'EOF'
#!/bin/bash
export LD_LIBRARY_PATH="$HOME/.local/lib:$LD_LIBRARY_PATH"
exec ~/.local/bin/zombie_rustc_driver "$@"
EOF

chmod +x ~/.local/bin/zombie-rustc
```

### Network Node Deployment
```bash
# Start zombie network node
./target/debug/zombie_rustc_driver --network-mode > zombie.log 2>&1 &
echo $! > zombie.pid

# Monitor network activity
tail -f zombie.log

# Check peer connections
grep "Discovered zombie peer" zombie.log
```

### Multi-Node Network Setup
```bash
# Terminal 1: Start first zombie node
./target/debug/zombie_rustc_driver --network-mode

# Terminal 2: Start second zombie node  
./target/debug/zombie_rustc_driver --network-mode

# Terminal 3: Test compilation with network
echo 'fn test() { let x = 42; }' > network_test.rs
./target/debug/zombie_rustc_driver network_test.rs
```

## SOP-005: Monitoring and Maintenance

### Health Checks
```bash
# Check zombie process status
ps aux | grep zombie_rustc_driver

# Monitor network connections
ss -tulpn | grep 4001

# Check compilation data flow
grep "Broadcasting compilation data" zombie.log
```

### Log Analysis
```bash
# View zombie infection messages
grep "🧟" zombie.log

# Monitor peer discovery
grep "peer" zombie.log

# Check compilation statistics
grep "AST nodes" zombie.log
```

### Performance Monitoring
```bash
# Monitor resource usage
top -p $(pgrep zombie_rustc_driver)

# Check network bandwidth
iftop -i lo  # For localhost testing

# Monitor compilation times
grep "compilation_time_ms" zombie.log
```

## SOP-006: Troubleshooting

### Common Issues

**Build Failures:**
```bash
# Clear cache and rebuild
cargo clean
rm -rf target/
cargo build
```

**Network Issues:**
```bash
# Check port availability
netstat -ln | grep 4001

# Kill existing zombie processes
pkill -f zombie_rustc_driver

# Restart with verbose logging
RUST_LOG=debug ./target/debug/zombie_rustc_driver --network-mode
```

**Library Path Issues:**
```bash
# Find required libraries
ldd target/debug/zombie_rustc_driver

# Set library path
export LD_LIBRARY_PATH="./target/debug:./target/debug/deps:$LD_LIBRARY_PATH"
```

### Emergency Procedures
```bash
# Kill all zombie processes
pkill -9 -f zombie_rustc_driver

# Clean quarantine area
rm -rf /tmp/zombie_quarantine/*

# Reset network state
sudo netstat -tulpn | grep 4001 | awk '{print $7}' | cut -d'/' -f1 | xargs -r kill
```

## SOP-007: Shutdown Procedures

### Graceful Shutdown
```bash
# Send SIGTERM to zombie processes
kill $(cat zombie.pid)

# Wait for graceful shutdown
sleep 5

# Verify termination
ps aux | grep zombie_rustc_driver
```

### Force Shutdown
```bash
# Force kill if needed
kill -9 $(cat zombie.pid)

# Clean up pid file
rm -f zombie.pid

# Clean up logs
mv zombie.log zombie.log.$(date +%Y%m%d_%H%M%S)
```

### Environment Cleanup
```bash
# Remove temporary files
rm -rf /tmp/zombie_quarantine/*

# Clear environment variables
unset ZOMBIE_QUARANTINE
unset RUSTC_WRAPPER

# Archive logs
tar -czf zombie_logs_$(date +%Y%m%d).tar.gz *.log
```

## SOP-008: Version Control and Updates

### Code Updates
```bash
# Commit changes
git add .
git commit -m "🧟‍♂️ Update: [description]"

# Tag releases
git tag -a v0.1.0 -m "Initial zombie release"
```

### Dependency Updates
```bash
# Update Cargo.lock
cargo update

# Check for security advisories
cargo audit

# Test after updates
cargo test
```

---

**⚠️ SAFETY REMINDER:** Always follow SCP containment procedures when working with zombie rustc driver. Never deploy to production systems without proper isolation and monitoring.
