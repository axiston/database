//! Implements type-safe enumerations for database queries.

/// Implements a type-safe `TokenAction` enumeration.
#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::TokenAction"]
pub enum TokenActionForm {
    #[db_rename = "activate_account"]
    ActivateAccount,
    #[db_rename = "deactivate_account"]
    DeactivateAccount,
    #[db_rename = "update_email"]
    UpdateEmail,
    #[db_rename = "reset_password"]
    ResetPassword,
    #[db_rename = "pending_invite"]
    PendingInvite,
}

/// Implements a type-safe `InviteStatus` enumeration.
#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::InviteStatus"]
pub enum InviteStatusForm {
    #[db_rename = "pending"]
    Pending,
    #[db_rename = "accepted"]
    Accepted,
    #[db_rename = "declined"]
    Declined,
    #[db_rename = "canceled"]
    Canceled,
}

/// Implements a type-safe `ProjectRole` enumeration.
#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ProjectRole"]
pub enum ProjectRoleForm {
    #[db_rename = "owner"]
    Owner,
    #[db_rename = "member"]
    Member,
}
