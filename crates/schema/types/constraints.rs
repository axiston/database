//! Comprehensive list of all constraint violations.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Comprehensive list of all constraint violations.
///
/// This includes unique constraint violations as well as foreign key
/// constraint violations for various tables, including workspaces.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use = "constraints do nothing unless they are used"]
pub enum ConstraintViolation {
    #[strum(serialize = "accounts_non_empty_display_name")]
    AccountsNonEmptyName,
    #[strum(serialize = "accounts_non_empty_email_address")]
    AccountsNonEmptyEmail,
    #[strum(serialize = "accounts_non_empty_password_hash")]
    AccountsNonEmptyPassword,
    #[strum(serialize = "accounts_updated_after_created")]
    AccountsUpdatedAfterCreated,
    #[strum(serialize = "accounts_deleted_after_created")]
    AccountsDeletedAfterCreated,
    #[strum(serialize = "accounts_deleted_after_updated")]
    AccountsDeletedAfterUpdated,

    #[strum(serialize = "account_sessions_region_alphanumeric")]
    AccountSessionsRegionAlnum,
    #[strum(serialize = "account_sessions_expired_after_issued")]
    AccountSessionsExpiredAfterIssued,
    #[strum(serialize = "account_sessions_deleted_after_issued")]
    AccountSessionsDeletedAfterIssued,

    #[strum(serialize = "account_permissions_updated_after_created")]
    AccountPermissionsUpdatedAfterCreated,
    #[strum(serialize = "account_permissions_deleted_after_created")]
    AccountPermissionsDeletedAfterCreated,
    #[strum(serialize = "account_permissions_deleted_after_updated")]
    AccountPermissionsDeletedAfterUpdated,

    #[strum(serialize = "account_tokens_expired_after_issued")]
    AccountTokensExpiredAfterIssued,
    #[strum(serialize = "account_tokens_used_after_issued")]
    AccountTokensUsedAfterIssued,
    #[strum(serialize = "account_tokens_expired_after_used")]
    AccountTokensExpiredAfterUsed,

    #[strum(serialize = "workspaces_unique_name")]
    WorkspacesUniqueName,
    #[strum(serialize = "workspaces_non_empty_name")]
    WorkspacesNonEmptyName,
    #[strum(serialize = "workspaces_metadata_limit")]
    WorkspacesMetadataLimit,
    #[strum(serialize = "workspaces_updated_after_created")]
    WorkspacesUpdatedAfterCreated,
    #[strum(serialize = "workspaces_deleted_after_created")]
    WorkspacesDeletedAfterCreated,
    #[strum(serialize = "workspaces_deleted_after_updated")]
    WorkspacesDeletedAfterUpdated,

    #[strum(serialize = "workflows_unique_display_name")]
    WorkflowsUniqueName,
    #[strum(serialize = "workflows_non_empty_display_name")]
    WorkflowsNonEmptyName,
    #[strum(serialize = "workflows_metadata_limit")]
    WorkflowsMetadataLimit,
    #[strum(serialize = "workflows_input_graph_limit")]
    WorkflowsInputGraphLimit,
    #[strum(serialize = "workflows_runtime_meta_limit")]
    WorkflowsRuntimeMetaLimit,
    #[strum(serialize = "workflows_updated_after_created")]
    WorkflowsUpdatedAfterCreated,
    #[strum(serialize = "workflows_deleted_after_created")]
    WorkflowsDeletedAfterCreated,
    #[strum(serialize = "workflows_deleted_after_updated")]
    WorkflowsDeletedAfterUpdated,

    #[strum(serialize = "workflow_schedules_pkey")]
    WorkflowSchedulesUniquePair,

    #[strum(serialize = "workflow_webhooks_pkey")]
    WorkflowWebhooksUniquePair,

    #[strum(serialize = "workflow_executions_output_graph_limit")]
    WorkflowExecutionsOutputGraphLimit,
    #[strum(serialize = "workflow_executions_runtime_meta_limit")]
    WorkflowExecutionsRuntimeMetaLimit,
    #[strum(serialize = "workflow_executions_ended_after_started")]
    WorkflowExecutionsEndedAfterStarted,
    #[strum(serialize = "workflow_executions_deleted_after_started")]
    WorkflowExecutionsDeletedAfterStarted,
    #[strum(serialize = "workflow_executions_deleted_after_ended")]
    WorkflowExecutionsDeletedAfterEnded,
}

impl ConstraintViolation {
    /// Creates a new [`ConstraintViolation`] from the constraint name.
    pub fn new(constraint: &str) -> Option<Self> {
        constraint.parse().ok()
    }
}

#[cfg(test)]
mod test {
    use crate::constraints::ConstraintViolation;

    #[test]
    fn parse_constraint_violation() {
        assert_eq!(
            ConstraintViolation::new("workspaces_unique_name"),
            Some(ConstraintViolation::WorkspacesUniqueName)
        );
        assert_eq!(ConstraintViolation::new("unknown_constraint"), None);
    }

    #[test]
    fn stringify_constraint_violation() {
        assert_eq!(
            ConstraintViolation::WorkspacesUniqueName.to_string(),
            "workspaces_unique_name"
        );
    }
}
