use reqwest;
use serde_json::Value;

fn main() {
    let url: &str = "https://blockchain.info/ticker";
    let response = match reqwest::blocking::get(url) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    };

    let json: Value = serde_json::from_str(&response).unwrap();
    println!("1 BTC = {} €", json["EUR"]["last"]);
}
