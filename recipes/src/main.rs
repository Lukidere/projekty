use reqwest::{self, blocking, Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashSet,
    error::Error,
    io::{self, stdin},
    thread::sleep,
    time::Duration,
};
#[derive(Serialize, Deserialize, Clone)]
struct przepis {
    title: String,
    ingredients: String,
    servings: String,
    instructions: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut input: String = String::new();
    println!("Podaj przepis który chcesz poznać");
    io::stdin().read_line(&mut input).expect("nie dziala input");
    let client = Client::new();
    let response = client
        .get(format!(
            "https://api.api-ninjas.com/v1/recipe?query={}",
            input.trim()
        ))
        .header("X-Api-Key", "api-key")
        .send()
        .await
        .unwrap();
    let dane: Vec<przepis> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    if dane.is_empty() {
        println!("no data found");
        return Ok(());
    };
    let jedzenie = if dane.len() == 1 {
        dane[0].clone()
    } else {
        for (index, dana) in dane.iter().enumerate() {
            println!("{}:{}", index, dana.title)
        }
        println!("wybierz jeden");

        input = String::new();
        io::stdin().read_line(&mut input).expect("zly wybor");
        let wybor: usize = input.trim().parse::<usize>().expect("nie podano cyfry");
        dane[wybor].clone()
    };
    println!("wybrano: {}", jedzenie.title);
    jedzenie.ingredients.split("|").for_each(|dana: &str| {
        println!("{}", dana);
        sleep(Duration::from_millis(10))
    });
    jedzenie.instructions.split(".").for_each(|smiec: &str| {
        println!("{}", smiec);
        sleep(Duration::from_secs(1))
    });
    Ok(())
}
