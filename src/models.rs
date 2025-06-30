use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Deserialize)]
pub struct GenerateKeypairRequest {}

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Deserialize)]
pub struct BalanceRequest {
    pub address: String,
    #[serde(default)]
    pub token_mint: Option<String>,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub balance: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<u8>,
}

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub mint_address: String,
    pub transaction_signature: String,
}

#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct MintTokenResponse {
    pub transaction_signature: String,
    pub recipient_token_account: String,
}

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub transaction_signature: String,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub transaction_signature: String,
}
