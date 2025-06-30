use axum::{
    body::Bytes,
    extract::State,
    http::{header, HeaderMap},
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::from_slice;

use crate::{error::AppError, models::*, AppResult, AppState};

async fn parse_json<T>(body: Bytes) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    from_slice(&body).map_err(|e| AppError::JsonError(e.to_string()))
}

pub async fn generate_keypair(
    State(service): State<AppState>,
    body: Bytes,
) -> AppResult<KeypairResponse> {
    let _request: GenerateKeypairRequest = parse_json(body).await?;
    let response = service.generate_keypair();
    Ok(Json(crate::models::ApiResponse::success(response)))
}

pub async fn query_balance(
    State(service): State<AppState>,
    body: Bytes,
) -> AppResult<BalanceResponse> {
    let request: BalanceRequest = parse_json(body).await?;
    let response = service.get_balance(request).await?;
    Ok(Json(crate::models::ApiResponse::success(response)))
}

pub async fn create_token(
    State(service): State<AppState>,
    body: Bytes,
) -> AppResult<InstructionResponse> {
    let request: CreateTokenRequest = parse_json(body).await?;
    let response = service.create_token(request).await?;
    Ok(Json(crate::models::ApiResponse::success(response)))
}

pub async fn mint_token(
    State(service): State<AppState>,
    body: Bytes,
) -> AppResult<InstructionResponse> {
    let request: MintTokenRequest = parse_json(body).await?;
    let response = service.mint_token(request).await?;
    Ok(Json(crate::models::ApiResponse::success(response)))
}

pub async fn sign_message(
    State(service): State<AppState>,
    body: Bytes,
) -> AppResult<SignMessageResponse> {
    let request: SignMessageRequest = parse_json(body).await?;
    let response = service.sign_message(request)?;
    Ok(Json(crate::models::ApiResponse::success(response)))
}

pub async fn verify_message(
    State(service): State<AppState>,
    body: Bytes,
) -> AppResult<VerifyMessageResponse> {
    let request: VerifyMessageRequest = parse_json(body).await?;
    let response = service.verify_message(request)?;
    Ok(Json(crate::models::ApiResponse::success(response)))
}

pub async fn send_sol(
    State(service): State<AppState>,
    body: Bytes,
) -> AppResult<InstructionResponse> {
    let request: SendSolRequest = parse_json(body).await?;
    let response = service.send_sol(request).await?;
    Ok(Json(crate::models::ApiResponse::success(response)))
}

pub async fn send_token(
    State(service): State<AppState>,
    body: Bytes,
) -> AppResult<InstructionResponse> {
    let request: SendTokenRequest = parse_json(body).await?;
    let response = service.send_token(request).await?;
    Ok(Json(crate::models::ApiResponse::success(response)))
}
