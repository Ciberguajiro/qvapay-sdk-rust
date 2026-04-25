use crate::client::QvaPayClient;
use crate::error::SdkError;
use crate::models::{TransactionTransferRequest, TransactionTransferResponse};

impl QvaPayClient {
    /// Obtiene la lista de cambio de moneda.
    pub async fn transfer_balance(
        &self,
        request: TransactionTransferRequest,
    ) -> Result<TransactionTransferResponse, SdkError> {
        self.post("/transaction/transfer", &request).await
    }
}
