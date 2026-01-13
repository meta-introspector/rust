# OCI Rust Client for solfunmeme

Simple Rust client to interact with Oracle Cloud Infrastructure APIs.

## Quick Start

```bash
cd oci-client
cargo run
```

## Configuration

Create `~/.oci/config` with your credentials:

```ini
[DEFAULT]
user=ocid1.user.oc1..your-user-ocid
fingerprint=your-key-fingerprint
tenancy=ocid1.tenancy.oc1..your-tenancy-ocid
region=us-ashburn-1
key_file=~/.oci/oci_api_key.pem
```

## Target Instance Configuration

**OCID**: `ocid1.instanceconfiguration.oc1.iad.aaaaaaaaabzhyygoc5clndba7tpuskdlkl2weivohvjkl65s5cvobuvywcrq`

This will retrieve the solfunmeme instance configuration and save it locally for deployment planning.

## Features

- Retrieve instance configurations
- Parse OCI responses
- Save configuration as JSON
- Minimal dependencies
- Pure Rust implementation

## Next Steps

Once we have the configuration, we can:
1. Create instances from the config
2. Deploy the P2P server
3. Manage the solfunmeme cloud infrastructure
