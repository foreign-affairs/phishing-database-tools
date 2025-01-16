import sys
import dns.resolver

def fetch_dns_txt(domain):
    resolver = dns.resolver.Resolver()
    resolver.nameservers = ['9.9.9.10']
    query = f"_phishingdb.{domain}"
    try:
        answers = resolver.resolve(query, 'TXT')
        return [rdata.to_text().strip('"') for rdata in answers]
    except dns.resolver.NoAnswer:
        return []

def main():
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <domain> <expected_value>")
        sys.exit(1)

    domain = sys.argv[1]
    expected_value = sys.argv[2]

    dns_txt_records = fetch_dns_txt(domain)
    if expected_value in dns_txt_records:
        print("The DNS TXT record matches the expected value.")
    else:
        print("The DNS TXT record does not match the expected value.")

if __name__ == "__main__":
    main()
