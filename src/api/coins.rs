use crate::client::QvaPayClient;
use crate::error::SdkError;
use crate::models::CoinsResponse;

impl QvaPayClient {
    /// Obtiene la lista de cambio de moneda.
    pub async fn get_coins(&self) -> Result<Vec<CoinsResponse>, SdkError> {
        self.get("/coins").await
    }
}
