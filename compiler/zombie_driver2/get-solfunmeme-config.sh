#!/bin/bash

# Disable GNU pinentry
export GPG_TTY=""
export PINENTRY_USER_DATA="USE_CURSES=0"

# Retrieve Oracle Cloud instance configuration
# OCID: ocid1.instanceconfiguration.oc1.iad.aaaaaaaaabzhyygoc5clndba7tpuskdlkl2weivohvjkl65s5cvobuvywcrq

INSTANCE_CONFIG_OCID="ocid1.instanceconfiguration.oc1.iad.aaaaaaaaabzhyygoc5clndba7tpuskdlkl2weivohvjkl65s5cvobuvywcrq"

echo "🔍 Retrieving instance configuration details..."

# Get the instance configuration
oci compute-management instance-configuration get \
  --instance-configuration-id $INSTANCE_CONFIG_OCID \
  --output table

echo ""
echo "📋 Getting detailed JSON configuration..."

# Get full JSON details
oci compute-management instance-configuration get \
  --instance-configuration-id $INSTANCE_CONFIG_OCID > solfunmeme-instance-config.json

echo "✅ Configuration saved to: solfunmeme-instance-config.json"

# Show key details
echo ""
echo "🔧 Key configuration details:"
cat solfunmeme-instance-config.json | jq '.data | {
  "display-name": .["display-name"],
  "shape": .["instance-details"]["launch-details"].shape,
  "image-id": .["instance-details"]["launch-details"]["source-details"]["image-id"],
  "compartment-id": .["compartment-id"]
}'
