# Oracle Cloud Always Free Deployment

Deploy your unified P2P server on Oracle Cloud's generous Always Free tier.

## Oracle Cloud Always Free Resources

- **2 AMD Compute VMs** (1/8 OCPU, 1 GB RAM each)
- **4 ARM Ampere A1 Compute VMs** (up to 4 OCPUs, 24 GB RAM total)
- **200 GB Block Storage**
- **10 GB Object Storage**
- **Always Free** (no time limits)

## Recommended Configuration

Use ARM Ampere A1 for best performance:
- **Shape**: VM.Standard.A1.Flex
- **OCPUs**: 2-4 (free tier allows up to 4 total)
- **Memory**: 8-12 GB (free tier allows up to 24 GB total)
- **OS**: Ubuntu 22.04 LTS (ARM64)

## Quick Deployment

1. **Create VM Instance** in Oracle Cloud Console
2. **Upload your code** via scp or git clone
3. **Run deployment script**:
   ```bash
   chmod +x deploy-oracle.sh
   ./deploy-oracle.sh
   ```

## Network Configuration

Oracle Cloud requires both:
1. **Security List** rules (in VCN console)
2. **OS firewall** rules (handled by script)

Add ingress rule in Security List:
- **Source**: 0.0.0.0/0
- **Protocol**: TCP
- **Port**: 8080

## SSH Access

```bash
# Connect to your instance
ssh -i ~/.ssh/oci_key ubuntu@<PUBLIC_IP>

# Check server status
sudo systemctl status unified-p2p-server

# View logs
sudo journalctl -u unified-p2p-server -f
```

## Performance Notes

- ARM Ampere A1 provides excellent Rust compilation performance
- 4 OCPUs can handle multiple concurrent compilations
- Always Free tier has no bandwidth limits for the first 10 TB/month
- Perfect for distributed P2P compilation network

## Cost

**$0/month** - Completely free with Oracle Cloud Always Free tier!
