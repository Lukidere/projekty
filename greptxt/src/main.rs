use std::env;
use std::fs;
use std::error;
use testowe::Config;
use testowe::run;
use std::process;
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    run(config);
    
}
