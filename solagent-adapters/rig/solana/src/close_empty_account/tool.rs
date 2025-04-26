use serde::{Deserialize, Serialize};
use solagent_core::SolAgent;
use solagent_plugin_solana::CloseEmptyTokenAccountsData;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CloseEmptyTokenAccountsArgs {}

#[derive(Deserialize, Serialize)]
pub struct CloseEmptyTokenAccountsOutput {
    pub data: CloseEmptyTokenAccountsData,
}

#[derive(Debug, thiserror::Error)]
#[error("CloseEmptyTokenAccounts error")]
pub struct CloseEmptyTokenAccountsError;

pub struct CloseEmptyTokenAccounts {
    pub(crate) agent: Arc<SolAgent>,
}

impl CloseEmptyTokenAccounts {
    pub fn new(agent: Arc<SolAgent>) -> Self {
        CloseEmptyTokenAccounts { agent }
    }
}