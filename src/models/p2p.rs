use serde::{Deserialize, Serialize};

/// Respuesta de la lista p2p
#[derive(Deserialize, Debug, Clone)]
pub struct GetP2PResponse {
    pub offers: Vec<P2P>,
}

/// p2p
#[derive(Deserialize, Debug, Clone)]
pub struct P2P {
    pub uuid: String,
    pub type_1: String,
    pub coin: String,
    pub amount: f32,
    pub receive: f32,
    pub status: String,
    pub only_kyc: i32,
    pub only_vip: i32,
    pub private: i32,
    pub message: String,
    pub created_at: String,
    pub updated_at: String,
    pub User: UserP2P,
    pub Coin: CoinP2P,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserP2P {
    pub uuid: String,
    pub username: String,
    pub name: String,
    pub image: String,
    pub kyc: bool,
    pub vip: bool,
    pub golden_check: bool,
}

/// Moneda del p2p
#[derive(Deserialize, Debug, Clone)]
pub struct CoinP2P {
    pub tic: String,
    pub name: String,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateP2PRequest {
    //	Tipo de oferta: buy o sell
    pub type_1: String,
    // | number	Tick de la moneda (ej: BANK_CUP) o ID numérico
    pub coin: String,
    //	Cantidad en QUSD (0.1 - 100,000)
    pub amount: i32,
    //	Cantidad a recibir en la moneda seleccionada (0.1 - 1,000,000)
    pub receive: i32,
    //	Datos de pago según los campos de la moneda (ej: cuenta bancaria)
    pub details: Vec<DetailsP2PRequest>,
    //		1 para restringir a usuarios con KYC verificado
    pub only_kyc: i32,
    //		1 para restringir a usuarios VIP
    pub only_vip: i32,
    //		1 para crear oferta privada (no se publica en Telegram)
    pub private: i32,
    //	Mensaje público de la oferta (máx. 79 caracteres, URLs eliminadas)
    pub message: String,
    //	URL de webhook para notificaciones de estado
    pub webhook: String,
    //	Etiquetas para la oferta (máx. 10 tags)
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateP2PResponse {
    //	Tipo de oferta: buy o sell
    pub msg: String,
    pub p2p: P2P,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailsP2PRequest {
    pub name: String,
    pub value: String,
}
