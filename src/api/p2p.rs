use crate::client::QvaPayClient;
use crate::error::SdkError;
use crate::models::{CreateP2PRequest, GetP2PResponse};

impl QvaPayClient {
    /// Obtiene las ofertas p2p
    pub async fn get_p2ps(&self) -> Result<GetP2PResponse, SdkError> {
        self.get("/p2p").await
    }

    /// Crea una oferta p2p
    pub async fn create_p2p(&self, request: CreateP2PRequest) -> Result<GetP2PResponse, SdkError> {
        self.post("/p2p/create", &request).await
    }

    /// Aplicar una oferta p2p
    pub async fn apply_p2p(&self, uuid: String) -> Result<GetP2PResponse, SdkError> {
        self.post(&format!("/p2p/{}/apply", uuid), &{}).await
    }
}
