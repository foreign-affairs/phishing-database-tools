#!/bin/bash

# Check if domain and expected value are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <domain> <expected_value>"
    exit 1
fi

domain=$1
expected_value=$2

# Fetch the DNS TXT record
dns_txt_record=$(dig @9.9.9.10 +short TXT _phishingdb.$domain)

# Compare the DNS TXT record with the expected value
if [[ "$dns_txt_record" == "\"$expected_value\"" ]]; then
    echo "The DNS TXT record matches the expected value."
else
    echo "The DNS TXT record does not match the expected value."
fi
