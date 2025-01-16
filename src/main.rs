use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <domain> <expected_value>", args[0]);
        std::process::exit(1);
    }

    let domain = format!("_phishingdb.{}", args[1]);
    let expected_value = &args[2];

    let resolver_config = ResolverConfig::from_parts(
        None,
        Vec::new(),
        NameServerConfigGroup::from_ips_clear(
            &[std::net::IpAddr::V4(std::net::Ipv4Addr::new(9, 9, 9, 10))],
            53,
            true,
        ),
    );

    let resolver = TokioAsyncResolver::tokio(resolver_config, ResolverOpts::default());

    match resolver.txt_lookup(domain).await {
        Ok(response) => {
            let records = response.iter().map(|r| r.to_string()).collect::<Vec<_>>();
            if records.contains(&expected_value.to_string()) {
                println!("The DNS TXT record matches the expected value.");
            } else {
                println!("The DNS TXT record does not match the expected value.");
            }
        }
        Err(err) => {
            eprintln!("DNS query failed: {}", err);
            std::process::exit(1);
        }
    }
}