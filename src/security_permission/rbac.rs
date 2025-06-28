// Role-Based Access Control (RBAC) using Casbin

use casbin::Enforcer;

// RBAC configuration structure
pub struct RBAC {
    enforcer: Enforcer,
}

impl RBAC {
    pub fn new() -> Self {
        RBAC {
            enforcer: Enforcer::new("rbac_model.conf", "rbac_policy.csv").unwrap(),
        }
    }
}