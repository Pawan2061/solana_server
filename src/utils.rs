use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
};
use std::str::FromStr;

use crate::error::AppError;

pub fn keypair_from_base58(private_key: &str) -> Result<Keypair, AppError> {
    let bytes = bs58::decode(private_key)
        .into_vec()
        .map_err(|e| AppError::Keypair(format!("Invalid base58 private key: {}", e)))?;

    Keypair::from_bytes(&bytes)
        .map_err(|e| AppError::Keypair(format!("Invalid keypair bytes: {}", e)))
}

pub fn parse_pubkey(address: &str) -> Result<Pubkey, AppError> {
    Pubkey::from_str(address)
        .map_err(|e| AppError::InvalidInput(format!("Invalid public key: {}", e)))
}

pub fn parse_signature(signature: &str) -> Result<Signature, AppError> {
    Signature::from_str(signature)
        .map_err(|e| AppError::InvalidInput(format!("Invalid signature: {}", e)))
}
