use base64::{engine::general_purpose::STANDARD as base64, Engine as _};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::Mint;
use std::str::FromStr;

use crate::{config::Config, crypto::CryptoService, error::AppError, models::*, utils};

pub struct SolanaService {
    rpc_client: RpcClient,
    crypto_service: CryptoService,
}

impl SolanaService {
    pub fn new(config: Config) -> Result<Self, AppError> {
        let commitment = match config.commitment.as_str() {
            "processed" => CommitmentConfig::processed(),
            "confirmed" => CommitmentConfig::confirmed(),
            "finalized" => CommitmentConfig::finalized(),
            _ => CommitmentConfig::confirmed(),
        };

        Ok(Self {
            rpc_client: RpcClient::new_with_commitment(config.rpc_url, commitment),
            crypto_service: CryptoService::new(),
        })
    }

    pub fn generate_keypair(&self) -> KeypairResponse {
        let keypair = Keypair::new();
        KeypairResponse {
            pubkey: keypair.pubkey().to_string(),
            secret: bs58::encode(&keypair.to_bytes()).into_string(),
        }
    }

    pub async fn get_balance(&self, request: BalanceRequest) -> Result<BalanceResponse, AppError> {
        let pubkey = utils::parse_pubkey(&request.address)?;

        if let Some(token_mint) = request.token_mint {
            self.get_token_balance(pubkey, &token_mint).await
        } else {
            self.get_sol_balance(pubkey).await
        }
    }

    async fn get_sol_balance(&self, pubkey: Pubkey) -> Result<BalanceResponse, AppError> {
        let balance = self.rpc_client.get_balance(&pubkey)?;
        Ok(BalanceResponse {
            balance,
            decimals: None,
        })
    }

    async fn get_token_balance(
        &self,
        owner: Pubkey,
        mint_str: &str,
    ) -> Result<BalanceResponse, AppError> {
        let mint_pubkey = utils::parse_pubkey(mint_str)?;
        let token_account = get_associated_token_address(&owner, &mint_pubkey);

        let balance = self.rpc_client.get_token_account_balance(&token_account)?;
        Ok(BalanceResponse {
            balance: balance.amount.parse().unwrap_or(0),
            decimals: Some(balance.decimals),
        })
    }

    pub async fn create_token(
        &self,
        request: CreateTokenRequest,
    ) -> Result<InstructionResponse, AppError> {
        let mint_authority = utils::parse_pubkey(&request.mint_authority)?;
        let mint = utils::parse_pubkey(&request.mint)?;

        let rent = self
            .rpc_client
            .get_minimum_balance_for_rent_exemption(Mint::LEN)?;

        let create_account_ix = system_instruction::create_account(
            &mint_authority,
            &mint,
            rent,
            Mint::LEN as u64,
            &spl_token::id(),
        );

        let init_mint_ix = spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &mint,
            &mint_authority,
            Some(&mint_authority),
            request.decimals,
        )
        .map_err(|e| AppError::Transaction(e.to_string()))?;

        Ok(InstructionResponse {
            program_id: spl_token::id().to_string(),
            accounts: vec![
                AccountInfo {
                    pubkey: mint_authority.to_string(),
                    is_signer: true,
                    is_writable: true,
                },
                AccountInfo {
                    pubkey: mint.to_string(),
                    is_signer: true,
                    is_writable: true,
                },
            ],
            instruction_data: base64.encode(&[create_account_ix.data, init_mint_ix.data].concat()),
        })
    }

    pub async fn mint_token(
        &self,
        request: MintTokenRequest,
    ) -> Result<InstructionResponse, AppError> {
        let authority = utils::parse_pubkey(&request.authority)?;
        let mint_pubkey = utils::parse_pubkey(&request.mint)?;
        let destination = utils::parse_pubkey(&request.destination)?;

        let destination_token_account = get_associated_token_address(&destination, &mint_pubkey);

        let create_ata_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                &authority,
                &destination,
                &mint_pubkey,
                &spl_token::id(),
            );

        let mint_ix = spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint_pubkey,
            &destination_token_account,
            &authority,
            &[&authority],
            request.amount,
        )
        .map_err(|e| AppError::Transaction(e.to_string()))?;

        Ok(InstructionResponse {
            program_id: spl_token::id().to_string(),
            accounts: vec![
                AccountInfo {
                    pubkey: authority.to_string(),
                    is_signer: true,
                    is_writable: false,
                },
                AccountInfo {
                    pubkey: mint_pubkey.to_string(),
                    is_signer: false,
                    is_writable: true,
                },
                AccountInfo {
                    pubkey: destination_token_account.to_string(),
                    is_signer: false,
                    is_writable: true,
                },
            ],
            instruction_data: base64.encode(&[create_ata_ix.data, mint_ix.data].concat()),
        })
    }

    pub fn sign_message(
        &self,
        request: SignMessageRequest,
    ) -> Result<SignMessageResponse, AppError> {
        let keypair = utils::keypair_from_base58(&request.secret)?;
        let signature = keypair.sign_message(request.message.as_bytes());

        Ok(SignMessageResponse {
            signature: base64.encode(signature.as_ref()),
            public_key: keypair.pubkey().to_string(),
            message: request.message,
        })
    }

    pub fn verify_message(
        &self,
        request: VerifyMessageRequest,
    ) -> Result<VerifyMessageResponse, AppError> {
        let pubkey = utils::parse_pubkey(&request.pubkey)?;
        let signature = utils::parse_signature(&request.signature)?;

        let valid = signature.verify(pubkey.as_ref(), request.message.as_bytes());
        Ok(VerifyMessageResponse {
            valid,
            message: request.message,
            pubkey: request.pubkey,
        })
    }

    pub async fn send_sol(&self, request: SendSolRequest) -> Result<InstructionResponse, AppError> {
        let from = utils::parse_pubkey(&request.from)?;
        let to = utils::parse_pubkey(&request.to)?;

        let transfer_ix = system_instruction::transfer(&from, &to, request.lamports);

        Ok(InstructionResponse {
            program_id: solana_sdk::system_program::id().to_string(),
            accounts: vec![
                AccountInfo {
                    pubkey: from.to_string(),
                    is_signer: true,
                    is_writable: true,
                },
                AccountInfo {
                    pubkey: to.to_string(),
                    is_signer: false,
                    is_writable: true,
                },
            ],
            instruction_data: base64.encode(transfer_ix.data),
        })
    }

    pub async fn send_token(
        &self,
        request: SendTokenRequest,
    ) -> Result<InstructionResponse, AppError> {
        let owner = utils::parse_pubkey(&request.owner)?;
        let mint_pubkey = utils::parse_pubkey(&request.mint)?;
        let destination = utils::parse_pubkey(&request.destination)?;

        let owner_token_account = get_associated_token_address(&owner, &mint_pubkey);
        let destination_token_account = get_associated_token_address(&destination, &mint_pubkey);

        let transfer_ix = spl_token::instruction::transfer(
            &spl_token::id(),
            &owner_token_account,
            &destination_token_account,
            &owner,
            &[&owner],
            request.amount,
        )
        .map_err(|e| AppError::Transaction(e.to_string()))?;

        Ok(InstructionResponse {
            program_id: spl_token::id().to_string(),
            accounts: vec![
                AccountInfo {
                    pubkey: owner_token_account.to_string(),
                    is_signer: false,
                    is_writable: true,
                },
                AccountInfo {
                    pubkey: destination_token_account.to_string(),
                    is_signer: false,
                    is_writable: true,
                },
                AccountInfo {
                    pubkey: owner.to_string(),
                    is_signer: true,
                    is_writable: false,
                },
            ],
            instruction_data: base64.encode(transfer_ix.data),
        })
    }
}
