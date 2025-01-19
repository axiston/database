//! Implements type-safe enumerations for database queries.

/// Implements a type-safe `EmailAction` enumeration.
#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::EmailAction"]
pub enum EmailTypeForm {
    #[db_rename = "confirm_email"]
    ConfirmEmail,
    #[db_rename = "update_email"]
    UpdateEmail,
    #[db_rename = "reset_password"]
    ResetPassword,
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

/// Implements a type-safe `PermissionRole` enumeration.
#[derive(Debug, Clone, Copy, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::PermissionRole"]
pub enum PermissionRoleForm {
    #[db_rename = "owner"]
    Owner,
    #[db_rename = "member"]
    Member,
}
