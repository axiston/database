//! TODO.
//!

#[derive(Debug, Clone, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::EmailType"]
pub enum EmailTypeForm {
    #[db_rename = "confirm_email"]
    ConfirmEmail,
    #[db_rename = "update_email"]
    UpdateEmail,
    #[db_rename = "reset_password"]
    ResetPassword,
}

#[derive(Debug, Clone, diesel_derive_enum::DbEnum)]
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
