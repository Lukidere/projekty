use reqwest::blocking::Client;
use serde::Deserialize;
use std::{error::Error, time::Duration};
use std::collections::HashSet;
use trust_dns_resolver::{config::{ResolverConfig,ResolverOpts},Resolver};
use crate::subdomains;
#[derive(Deserialize,Debug,Clone)]
struct CrtDane {
    pub name_value:String
}
pub struct Subdomain {
    pub domain:String,
    pub open_ports:Vec<Port>
}
pub struct Port {
    pub port:u16,
    pub is_open:bool
}
pub fn enumerate(http_client:Client,target:&str) -> Result<Vec<Subdomain>, Box<dyn Error>> {


    let response: Vec<CrtDane> = http_client.get(&format!("https://crt.sh/?q=%25.{}&output=json",target)).send()?.json()?;
    let mut subdomains:HashSet<String> = response.
        into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split("\n")
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
    .filter(|subdomain:&String| subdomain != target)
    .filter(|subdomain: &String| !subdomain.contains("*"))
    .collect();
    subdomains.insert(target.to_string());
    let subdomains:Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports:Vec::new()
        })
        .filter(resolves)
        .collect();

    Ok(subdomains)
}
pub fn resolves(domain:&Subdomain) -> bool{
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);
    let dns_resolver = Resolver::new(ResolverConfig::default(),opts).expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
