use clap::{command, Arg, ArgMatches};
use std::collections::VecDeque;
use std::fs;
#[tokio::main]
async fn main() {
    let match_result: ArgMatches = command!()
        .arg(Arg::new("file_path").short('f').long("file").required(true))
        .arg(Arg::new("site").short('s').long("site").required(true))
        .get_matches();
    let dana: String = match_result
        .get_one::<String>("file_path")
        .unwrap()
        .to_owned();
    let page: String = match_result.get_one::<String>("site").unwrap().to_owned();
    let dane = fs::read_to_string(dana).unwrap();
    let mut v: VecDeque<String> = dane.lines().map(|line| line.trim().to_owned()).collect();
    for i in v.iter() {
        match reqwest::get(page.clone() + i).await {
            Ok(res) => println!("dziala"),
            Err(err) => println!("nie dziala -> {err}"),
        }
    }
}
