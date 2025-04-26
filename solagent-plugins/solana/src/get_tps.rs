use solagent_core::SolAgent;
use anyhow::Result;

/// Gets the transactions per second (TPS) from the Solana network.
///
/// # Parameters
///
/// - `solagent`: An instance of `SolAgent`.
///
/// # Returns
///
/// A `Result` containing the TPS as a `f64`, or an error if fetching performance samples fails.
pub async fn get_tps(solagent: &SolAgent) -> Result<f64> {
    // Fetch recent performance samples
    let limit = 1;
    let perf_samples = solagent.rpc_client.get_recent_performance_samples(Some(limit))?;

    let mut tps = 0.0;
    // Check if there are any samples available
    if !perf_samples.is_empty() {
        // Calculate TPS
        let num_transactions = perf_samples[0].num_transactions;
        let sample_period_secs = perf_samples[0].sample_period_secs;

        tps = num_transactions as f64 / sample_period_secs as f64;

        return Ok(tps);
    }

    Ok(tps)
}
