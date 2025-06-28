use solana_client::rpc_client::RpcClient;

// Solana RPC structure
pub struct SolanaRPC {
    cluster: Vec<RpcClient>,
}

impl SolanaRPC {
    pub fn new() -> Self {
        SolanaRPC {
            cluster: vec![RpcClient::new("https://api.devnet.solana.com".to_string())],
        }
    }
}