use chrono::{DateTime, Utc};
use reqwest;
use serde_json::Value;
use std::time::SystemTime;

fn main() {
    let time: SystemTime = SystemTime::now();
    let dt: DateTime<Utc> = time.clone().into();
    let date_time = dt.format("%d.%m.%Y %H:%M");
    let json: Value = fetch_ticker();
    println!("1 BTC = {} € ({date_time})", json["EUR"]["last"]);
}

fn fetch_ticker() -> Value {
    let url: &str = "https://blockchain.info/ticker";
    let response = match reqwest::blocking::get(url) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    return serde_json::from_str(&response).unwrap();
}
