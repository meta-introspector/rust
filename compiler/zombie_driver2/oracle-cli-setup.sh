#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Quick Oracle Cloud VM setup commands

# Create compute instance (run in OCI CLI or Cloud Shell)
oci compute instance launch \
  --availability-domain <AD-NAME> \
  --compartment-id <COMPARTMENT-OCID> \
  --shape VM.Standard.A1.Flex \
  --shape-config '{"ocpus": 2, "memoryInGBs": 8}' \
  --image-id <UBUNTU-22.04-ARM-IMAGE-OCID> \
  --subnet-id <SUBNET-OCID> \
  --ssh-authorized-keys-file ~/.ssh/id_rsa.pub \
  --display-name "unified-p2p-server"

# Add security list rule (replace SECURITY-LIST-OCID)
oci network security-list update \
  --security-list-id <SECURITY-LIST-OCID> \
  --ingress-security-rules '[{
    "source": "0.0.0.0/0",
    "protocol": "6",
    "tcpOptions": {"destinationPortRange": {"min": 8080, "max": 8080}}
  }]'
