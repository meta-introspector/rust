#!/usr/bin/env python3

import oci
import requests
import logging
from urllib3.util.retry import Retry
from requests.adapters import HTTPAdapter

# Enable debug logging to see all HTTP requests
logging.basicConfig(level=logging.DEBUG)
logging.getLogger("urllib3").setLevel(logging.DEBUG)
logging.getLogger("requests").setLevel(logging.DEBUG)

# Custom adapter to log request details
class LoggingHTTPAdapter(HTTPAdapter):
    def send(self, request, **kwargs):
        print(f"\n🔍 REQUEST DETAILS:")
        print(f"Method: {request.method}")
        print(f"URL: {request.url}")
        print(f"Headers:")
        for name, value in request.headers.items():
            print(f"  {name}: {value}")
        if request.body:
            print(f"Body: {request.body}")
        print()
        
        response = super().send(request, **kwargs)
        
        print(f"🔍 RESPONSE:")
        print(f"Status: {response.status_code}")
        print(f"Headers: {dict(response.headers)}")
        print(f"Body: {response.text[:500]}...")
        print()
        
        return response

def main():
    print("🔍 Testing OCI Python SDK with request tracing...")
    
    # Load config from our OCI config file
    config = oci.config.from_file("/home/mdupont/.solfunmeme-keys/oci_config")
    
    print("📋 Config loaded:")
    for key, value in config.items():
        if 'key' not in key.lower():  # Don't print the private key
            print(f"  {key}: {value}")
    print()
    
    # Create identity client with custom session for logging
    session = requests.Session()
    session.mount('https://', LoggingHTTPAdapter())
    
    # Create identity client
    identity_client = oci.identity.IdentityClient(config)
    
    # Monkey patch the session to use our logging adapter
    identity_client.base_client.session = session
    
    try:
        print("🔍 Making request to get tenancy...")
        tenancy = identity_client.get_tenancy(config['tenancy'])
        print(f"✅ Success! Tenancy name: {tenancy.data.name}")
        
        print("\n🔍 Now trying Resource Manager...")
        # Try Resource Manager
        resource_manager_client = oci.resource_manager.ResourceManagerClient(config)
        resource_manager_client.base_client.session = session
        
        stacks = resource_manager_client.list_stacks(
            compartment_id=config['tenancy'],
            lifecycle_state="ACTIVE"
        )
        print(f"✅ Found {len(stacks.data)} active stacks")
        for stack in stacks.data:
            print(f"  - {stack.display_name} ({stack.id})")
            
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    main()
