use crate::models::ApiResponse;
use axum::{http::StatusCode, response::Json as ResponseJson};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Solana RPC error: {0}")]
    SolanaRpc(#[from] solana_client::client_error::ClientError),
    #[error("Keypair error: {0}")]
    Keypair(String),
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Transaction failed: {0}")]
    Transaction(String),
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("JSON error: {0}")]
    JsonError(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            AppError::InvalidInput(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Keypair(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::SolanaRpc(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AppError::Transaction(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::JsonError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let response = ApiResponse::<()>::error(error_message);
        (status, ResponseJson(response)).into_response()
    }
}
