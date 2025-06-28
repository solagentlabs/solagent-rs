use {
    anyhow::{Context, Result},
    dotenv::dotenv,
    solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer},
    std::env,
    thiserror::Error,
};

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Environment variable '{0}' not found")]
    EnvVarNotFound(String),
    #[error("Invalid base58 private key")]
    InvalidBase58Key,
    #[error("Invalid private key bytes")]
    InvalidPrivateKeyBytes,
    #[error("File operation failed: {0}")]
    FileError(String),
}

/// Represents a wallet containing a keypair, its corresponding public key, and an RPC URL.
#[derive(Debug)]
pub struct SolAgentWallet {
    /// The keypair associated with the wallet. This contains the private key.
    pub keypair: Keypair,
    /// The public key associated with the wallet.
    pub pubkey: Pubkey,
    /// The RPC URL for interacting with the Solana blockchain.
    pub rpc_url: String,
}

impl Default for SolAgentWallet {
    /// Creates a new wallet with a randomly generated keypair and the default RPC URL.
    fn default() -> Self {
        Self::new("https://api.mainnet-beta.solana.com")
    }
}

impl SolAgentWallet {
    /// Creates a new wallet with a randomly generated keypair and the specified RPC URL.
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - The RPC URL for interacting with the Solana blockchain.
    ///
    /// # Returns
    ///
    /// * `SolAgentWallet` - A new wallet instance.
    pub fn new(rpc_url: &str) -> Self {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        Self { keypair, pubkey, rpc_url: rpc_url.to_string() }
    }

    /// Creates a wallet from a private key stored in an environment variable and the specified RPC URL.
    ///
    /// This function reads the environment variable specified by `variable_name`,
    /// decodes the base58 encoded private key, and creates a `SolAgentWallet` instance.
    ///
    /// # Arguments
    ///
    /// * `variable_name` - The name of the environment variable containing the private key.
    /// * `rpc_url` - The RPC URL for interacting with the Solana blockchain.
    ///
    /// # Returns
    ///
    /// * `Ok(SolAgentWallet)` - If the wallet was successfully created.
    /// * `Err(String)` - If the environment variable is not found or the private key is invalid.
    pub fn from_env(variable_name: &str, rpc_url: &str) -> Result<Self> {
        dotenv().ok(); // Load environment variables from .env file (if present)

        let private_key =
            env::var(variable_name).with_context(|| format!("Environment variable '{}' not found", variable_name))?;

        let mut wallet = Self::from_base58(&private_key)?;
        wallet.rpc_url = rpc_url.to_string();
        Ok(wallet)
    }

    /// Creates a wallet from a base58 encoded private key and the specified RPC URL.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The base58 encoded private key.
    /// * `rpc_url` - The RPC URL for interacting with the Solana blockchain.
    ///
    /// # Returns
    ///
    /// * `Ok(SolAgentWallet)` - If the wallet was successfully created.
    /// * `Err(String)` - If the private key is invalid or not properly encoded.
    pub fn from_base58_with_url(private_key: &str, rpc_url: &str) -> Result<Self> {
        let mut wallet = Self::from_base58(private_key)?;
        wallet.rpc_url = rpc_url.to_string();
        Ok(wallet)
    }

    /// Creates a wallet from a base58 encoded private key.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The base58 encoded private key.
    ///
    /// # Returns
    ///
    /// * `Ok(SolAgentWallet)` - If the wallet was successfully created.
    /// * `Err(String)` - If the private key is invalid or not properly encoded.
    pub fn from_base58(private_key: &str) -> Result<Self> {
        let secret_key = bs58::decode(private_key).into_vec().map_err(|_| WalletError::InvalidBase58Key)?;

        let keypair = Keypair::from_bytes(&secret_key).map_err(|_| WalletError::InvalidPrivateKeyBytes)?;

        let pubkey = keypair.pubkey();
        Ok(Self {
            keypair,
            pubkey,
            rpc_url: String::new(), // Default to an empty string; can be set later
        })
    }

    /// Returns the base58 encoded private key of the wallet.
    pub fn to_base58(&self) -> String {
        self.keypair.to_base58_string()
    }

    /// Saves the wallet's private key to a file.
    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let private_key = self.to_base58();
        std::fs::write(file_path, private_key)
            .with_context(|| format!("Failed to save wallet to file: {}", file_path))?;
        Ok(())
    }

    /// Loads a wallet from a private key file and sets the RPC URL.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file containing the private key.
    /// * `rpc_url` - The RPC URL for interacting with the Solana blockchain.
    ///
    /// # Returns
    ///
    /// * `Ok(SolAgentWallet)` - If the wallet was successfully loaded.
    /// * `Err(String)` - If the file could not be read or the private key is invalid.
    pub fn from_file_with_url(file_path: &str, rpc_url: &str) -> Result<Self> {
        let private_key =
            std::fs::read_to_string(file_path).with_context(|| format!("Failed to read wallet file: {}", file_path))?;
        let mut wallet = Self::from_base58(&private_key)?;
        wallet.rpc_url = rpc_url.to_string();
        Ok(wallet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_wallet_from_env_valid() -> Result<()> {
        let keypair = Keypair::new();
        let private_key = keypair.to_base58_string();
        env::set_var("TEST_PRIVATE_KEY", &private_key);

        let rpc_url = "https://api.testnet.solana.com";
        let wallet = SolAgentWallet::from_env("TEST_PRIVATE_KEY", rpc_url)?;
        assert_eq!(wallet.pubkey, keypair.pubkey());
        assert_eq!(wallet.rpc_url, rpc_url);

        env::remove_var("TEST_PRIVATE_KEY");
        Ok(())
    }

    #[test]
    fn test_wallet_from_env_not_found() {
        let rpc_url = "https://api.testnet.solana.com";
        let result = SolAgentWallet::from_env("NON_EXISTENT_VARIABLE", rpc_url);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Environment variable 'NON_EXISTENT_VARIABLE' not found");
    }

    #[test]
    fn test_wallet_creation() {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet = SolAgentWallet::new(rpc_url);
        assert_ne!(wallet.pubkey, Pubkey::default());
        assert_eq!(wallet.rpc_url, rpc_url);
    }

    #[test]
    fn test_wallet_from_base58_with_url_valid() -> Result<()> {
        let original_keypair = Keypair::new();
        let private_key = original_keypair.to_base58_string();
        let rpc_url = "https://api.testnet.solana.com";
        let wallet = SolAgentWallet::from_base58_with_url(&private_key, rpc_url)?;
        assert_eq!(wallet.pubkey, original_keypair.pubkey());
        assert_eq!(wallet.rpc_url, rpc_url);
        Ok(())
    }

    #[test]
    fn test_wallet_from_base58_invalid_base58() {
        let result = SolAgentWallet::from_base58("invalid_key");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid base58 private key");
    }

    #[test]
    fn test_wallet_from_base58_invalid_bytes() {
        let invalid_bytes = bs58::encode([0u8; 10]).into_string();
        let result = SolAgentWallet::from_base58(&invalid_bytes);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid private key bytes");
    }

    #[test]
    fn test_wallet_to_base58() -> Result<()> {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet = SolAgentWallet::new(rpc_url);
        let base58_key = wallet.to_base58();
        assert!(!base58_key.is_empty());

        let wallet2 = SolAgentWallet::from_base58_with_url(&base58_key, rpc_url)?;
        assert_eq!(wallet.pubkey, wallet2.pubkey);
        assert_eq!(wallet.rpc_url, wallet2.rpc_url);
        Ok(())
    }
}
