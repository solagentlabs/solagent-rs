use solana_client::{
    client_error::ClientError, rpc_client::RpcClient};
use solagent_wallet_solana::SolAgentWallet;

/// Gets the transactions per second (TPS) from the Solana network.
///
/// # Parameters
///
/// - `wallet`: An instance of `SolAgentWallet`.
///
/// # Returns
///
/// A `Result` containing the TPS as a `f64`, or an error if fetching performance samples fails.
pub async fn get_tps(wallet: &SolAgentWallet) -> Result<f64, ClientError> {
    let client = RpcClient::new(wallet.rpc_url.clone());

    // Fetch recent performance samples
    let limit = 1;
    let perf_samples = client.get_recent_performance_samples(Some(limit))?;

    // Check if there are any samples available
    if !perf_samples.is_empty() {
        // Calculate TPS
        let num_transactions = perf_samples[0].num_transactions;
        let sample_period_secs = perf_samples[0].sample_period_secs;

        let tps = num_transactions as f64 / sample_period_secs as f64;

        return Ok(tps);
    }

    Ok(0.0)
}
