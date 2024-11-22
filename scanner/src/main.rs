mod common_ports;
mod ports;
use rayon::prelude::*;
use ports::scan_ports;
use crate::common_ports::MOST_COMMON_PORTS_100;
use std::{collections::HashSet,thread,time::Duration,net::{SocketAddr,ToSocketAddrs,TcpStream}};
use std::error::Error;
mod subdomains;
use subdomains::{Subdomain,enumerate};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::env;
use reqwest::{blocking::Client, redirect};
fn main() -> Result<(), Box<dyn Error>>{
    let timeout = Duration::from_secs(10);
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: cargo run -- <address.com>");
        return Ok(())
    }
    let addr = &args[1];
    let http_client = Client::builder().timeout(timeout).redirect(redirect::Policy::limited(4)).build().unwrap();
    let pool = rayon::ThreadPoolBuilder::new().
        num_threads(256)
        .build()
        .unwrap();
    pool.install(move || {
        let scan_result:Vec<Subdomain> = enumerate(http_client, addr)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();
        for subdomain in scan_result {
            println!("{}",&subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}",port.port)
            }
            println!();
        }
    });
    Ok(())
}


