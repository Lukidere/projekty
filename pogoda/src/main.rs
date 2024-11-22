use futures::{executor::block_on, future::BoxFuture, Future};
use openweather::{Language, LocationSpecifier, Unit};
use std::io::{self, stdin};
fn main() {
    let mut kraj = String::new();
    let mut miasto = String::new();
    println!("Podaj kraj w którym chcesz sprawdzić pogode");
    stdin().read_line(&mut kraj);
    println!("Podaj miasto w którym chcesz sprawdzić pogode");
    stdin().read_line(&mut miasto);
    block_on(wyswietl(miasto, kraj));
}

async fn callback(
    miasto: String,
    kraj: String,
) -> Result<openweather::WeatherReportCurrent, openweather::Error> {
    let lokacja = LocationSpecifier::CityAndCountryName {
        city: miasto,
        country: kraj,
    };
    return openweather::get_current_weather(
        &lokacja,
        "api-key",
        &openweather::Settings {
            unit: Some(Unit::Metric),
            lang: Some(Language::Polish),
        },
    );
}
async fn wyswietl(miasto: String, kraj: String) {
    let dane = match callback(miasto, kraj).await {
        Ok(val) => println!(
            "W miescie {}, jest teraz {:#?} stopni celsjusza i pogoda jest {:#?}",
            val.name, val.main.temp, val.weather[0].main
        ),
        Err(err) => panic!("Wystapil blad dokladne dane: {}", err),
    };
}
