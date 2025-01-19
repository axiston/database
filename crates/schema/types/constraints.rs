//! Includes a list of all constraint violations.

use std::ops::Deref;

/// Comprehensive list of all constraint violations.
///
/// Includes both unique constraint violations and
/// foreign key constraint violations.
///
/// Unfortunately, constraints cannot be automatically generated
/// from the database definition (unlike entities), so the list
/// must be maintained manually.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[must_use = "constraints do nothing unless you use them"]
pub enum ConstraintViolation {
    // Accounts table constraints.
    AccountsNonEmptyName,
    AccountsNonEmptyEmail,
    AccountsNonEmptyPassword,
    AccountsUpdatedAfterCreated,
    AccountsDeletedAfterCreated,
    AccountsDeletedAfterUpdated,

    // Account Sessions table constraints.
    SessionsRegionAlnum,
    SessionsExpiredAfterIssued,
    SessionsDeletedAfterIssued,

    // Account Permissions table constraints.
    PermissionsUpdatedAfterCreated,
    PermissionsDeletedAfterCreated,
    PermissionsDeletedAfterUpdated,

    // Account Actions table constraints.
    ActionsExpiredAfterIssued,
    ActionsUsedAfterIssued,
}

impl ConstraintViolation {
    /// Returns a new [`ConstraintViolation`].
    pub fn new(constraint: &str) -> Option<Self> {
        Some(match constraint {
            // Accounts table constraints.
            "accounts_non_empty_display_name" => Self::AccountsNonEmptyName,
            "accounts_non_empty_email_address" => Self::AccountsNonEmptyEmail,
            "accounts_non_empty_password_hash" => Self::AccountsNonEmptyPassword,
            "accounts_updated_after_created" => Self::AccountsUpdatedAfterCreated,
            "accounts_deleted_after_created" => Self::AccountsDeletedAfterCreated,
            "accounts_deleted_after_updated" => Self::AccountsDeletedAfterUpdated,

            // Account Sessions table constraints.
            "account_sessions_region_alphanumeric" => Self::SessionsRegionAlnum,
            "account_sessions_expired_after_issued" => Self::SessionsExpiredAfterIssued,
            "account_sessions_deleted_after_issued" => Self::SessionsDeletedAfterIssued,

            // Account Permissions table constraints.
            "account_permissions_updated_after_created" => Self::PermissionsUpdatedAfterCreated,
            "account_permissions_deleted_after_created" => Self::PermissionsDeletedAfterCreated,
            "account_permissions_deleted_after_updated" => Self::PermissionsDeletedAfterUpdated,

            // Account Actions table constraints.
            "account_actions_expired_after_issued" => Self::ActionsExpiredAfterIssued,
            "account_actions_used_after_issued" => Self::ActionsUsedAfterIssued,

            _ => return None,
        })
    }

    /// Returns the constraint name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            // Accounts table constraints.
            Self::AccountsNonEmptyName => "accounts_non_empty_display_name",
            Self::AccountsNonEmptyEmail => "accounts_non_empty_email_address",
            Self::AccountsNonEmptyPassword => "accounts_non_empty_password_hash",
            Self::AccountsUpdatedAfterCreated => "accounts_updated_after_created",
            Self::AccountsDeletedAfterCreated => "accounts_deleted_after_created",
            Self::AccountsDeletedAfterUpdated => "accounts_deleted_after_updated",

            // Account Sessions table constraints.
            Self::SessionsRegionAlnum => "account_sessions_region_alphanumeric",
            Self::SessionsExpiredAfterIssued => "account_sessions_expired_after_issued",
            Self::SessionsDeletedAfterIssued => "account_sessions_deleted_after_issued",

            // Account Permissions table constraints.
            Self::PermissionsUpdatedAfterCreated => "account_permissions_updated_after_created",
            Self::PermissionsDeletedAfterCreated => "account_permissions_deleted_after_created",
            Self::PermissionsDeletedAfterUpdated => "account_permissions_deleted_after_updated",

            // Account Actions table constraints.
            Self::ActionsExpiredAfterIssued => "account_actions_expired_after_issued",
            Self::ActionsUsedAfterIssued => "account_actions_used_after_issued",
        }
    }
}

impl Deref for ConstraintViolation {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg(test)]
mod test {
    use crate::constraints::ConstraintViolation;

    #[test]
    fn parse_constraint_violation() {
        assert_eq!(
            ConstraintViolation::new("accounts_non_empty_display_name"),
            Some(ConstraintViolation::AccountsNonEmptyName)
        );
        assert_eq!(ConstraintViolation::new("unknown_constraint"), None);
    }

    #[test]
    fn stringify_constraint_violation() {
        assert_eq!(
            ConstraintViolation::AccountsNonEmptyName.as_str(),
            "accounts_non_empty_display_name"
        );
    }
}
