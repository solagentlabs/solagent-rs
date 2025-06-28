use serde::{Deserialize, Serialize};
use solagent_core::SolAgent;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
#[error("GetTps error")]
pub struct GetTpsError;

#[derive(Deserialize)]
pub struct GetTpsArgs {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTpsOutput {
    pub tps: f64,
}

#[derive(Clone)]
pub struct GetTps {
    pub(crate) solagent: Arc<SolAgent>,
}

impl GetTps {
    pub fn new(solagent: Arc<SolAgent>) -> Self {
        GetTps { solagent }
    }
}
