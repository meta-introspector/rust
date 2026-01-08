# Zombie Rustc Driver Scripts

## Quick Start
```bash
./build_zombie.sh      # Build with sccache
./zombie.sh --help     # Test zombie driver
./zombie_network.sh    # Start P2P network
```

## Scripts

### `zombie.sh` - Main Driver
Wrapper for zombie rustc driver with all environment setup:
- Automatic sccache integration
- Proper library paths
- Host triple configuration

### `zombie_network.sh` - P2P Server
Starts zombie network in server mode:
- Listens on port 4001
- Gossip protocol for compilation data
- Multi-interface binding

### `build_zombie.sh` - Build System
Optimized build with sccache:
- Release mode compilation
- Codegen backend setup
- Environment validation

### `run_zombie_tests.sh` - Test Suite
Comprehensive testing:
- Network mode tests
- No-network compilation
- Diagnose mode
- Full integration tests

## Usage Examples
```bash
# Compile with zombie infection
./zombie.sh test.rs --crate-type bin

# Disable network for debugging
./zombie.sh --no-net test.rs --crate-type bin

# Diagnose mode
./zombie.sh --diagnose test.rs --crate-type bin

# Start network server
./zombie_network.sh
```
