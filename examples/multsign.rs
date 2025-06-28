use solagent::{
    tool_system::ToolRegistry,
    solana_integration::SolanaRPC,
};

#[tokio::main]
async fn main() {
    let rpc = SolanaRPC::new("https://api.devnet.solana.com");
    let registry = ToolRegistry::new();
    registry.register_tool("create_multisig", CreateMultisigTool::new(rpc));

    let result = registry.execute("create_multisig", { signers: vec!["pubkey1", "pubkey2"], threshold: 2 }).await;
    println!("Multisig Created: {}", result.unwrap());
}