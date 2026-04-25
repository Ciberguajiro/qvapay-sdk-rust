use serde::{Deserialize, Serialize};

/// Information about the merchant application.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionTransferRequest {
    pub amount: String,
    pub to: String,
    pub pin: String,
    pub description: Option<String>,
}

/// Information about the merchant application.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionTransferResponse {
    pub success: bool,
    pub message: String,
    pub transaction: String,
}
