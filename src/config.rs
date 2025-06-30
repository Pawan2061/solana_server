#[derive(Clone, Debug)]
pub struct Config {
    pub rpc_url: String,
    pub commitment: String,
    pub timeout_seconds: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            rpc_url: std::env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()),
            commitment: std::env::var("SOLANA_COMMITMENT")
                .unwrap_or_else(|_| "confirmed".to_string()),
            timeout_seconds: std::env::var("TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        }
    }
}
