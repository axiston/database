//! Data layer (queries and forms).

use serde::{Deserialize, Serialize};

pub mod account_permissions;
pub mod account_sessions;
pub mod account_tokens;
pub mod accounts;
pub mod workflow_executions;
pub mod workflow_schedules;
pub mod workflow_webhooks;
pub mod workflows;
pub mod workspace_invites;
pub mod workspace_members;
pub mod workspace_schedules;
pub mod workspace_webhooks;
pub mod workspaces;

/// TODO.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QueryOrderBy {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}
