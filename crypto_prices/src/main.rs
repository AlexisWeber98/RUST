use serde::Deserialize;
use std::io;
use thiserror::Error;
use ureq;

#[derive(Deserialize)]
struct CoinResponse {
    time: String,
    asset_id_base: String,
    asset_id_quote: String,
    rate: f64,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Request error: {0}")]
    Request(#[from] ureq::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

fn get_precio(coin: &str) -> Result<CoinResponse, MyError> {
    let url = format!("https://rest.coinapi.io/v1/exchangerate/{}/USD/APIKEY-6078B0CB-3230-4022-90FF-D27263E7B269", coin);
    let body: String = ureq::get(&url).call()?.into_string()?;
    let coin_response: CoinResponse = serde_json::from_str(&body)?;
    Ok(coin_response)
}

fn main() -> Result<(), MyError> {
    let mut coin = String::new();
    println!("¿Que crypto deseas consultar? recuerda usar su simbolo en mayúsculas (ej: BTC )");
    io::stdin().read_line(&mut coin)?;
    let coin = coin.trim(); // Eliminar posibles espacios en blanco o nuevas líneas

    match get_precio(&coin) {
        Ok(precio) => println!(
            "El precio de {} es: {} {}",
            coin, precio.rate, precio.asset_id_quote
        ),
        Err(error) => println!("Hubo un error: {}", error),
    }
    Ok(())
}
