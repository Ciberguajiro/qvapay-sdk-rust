use serde::Deserialize;

/// Information about the merchant application.
#[derive(Deserialize, Debug, Clone)]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub tick: String,
    pub logo: String,
    pub price: String,
    pub change_24h: String,
}

/// Information about the merchant application.
#[derive(Deserialize, Debug, Clone)]
pub struct CoinsResponse {
    pub id: String,
    pub name: String,
    pub coins: Vec<Coin>,
}
