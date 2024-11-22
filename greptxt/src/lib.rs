use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Ehm what the sigma")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Ehm what the sigma")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let wynik = if config.ignore_case {
        search_case_insensitive(&config.query,&content)
    } else {
        search(&config.query,&content)
    };
    for line in wynik {
        println!("{line}");
    }
    Ok(())
}


pub fn search<'a>(query: &str, v: &'a str) -> Vec<&'a str> {
    v
        .lines()
        .filter(|item| item.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, v: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut wynik: Vec<&str> = Vec::new();
    for line in v.lines() {
        if line.to_lowercase().contains(&query) {
            wynik.push(line)
        } else {
            continue;
        }
    }
    wynik
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn nie_znajduje() {
//         let zapytanie = "cos";
//         let v: Vec<&str> = vec!["jablko", "gruszka", "ananas"];
//         let pustyvec: Vec<&str> = vec![];
//         assert_eq!(search(zapytanie, v), pustyvec);
//     }
//     #[test]
//     fn znajduje() {
//         let zapytanie = "kaczka";
//         let v: Vec<&str> = vec!["kaczka", "ges", "jablko"];
//         let porownywany: Vec<&str> = vec!["kaczka"];
//         assert_eq!(search(zapytanie, v), porownywany);
//     }
// }
