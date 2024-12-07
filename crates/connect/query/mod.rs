//! Set of traits encapsulating all queries.
//!

mod account_emails;
mod account_permissions;
mod account_sessions;
mod accounts;
mod project_invites;
mod project_members;
mod projects;
mod workflows;

pub mod account_queries {
    //! Queries for `accounts` and `account_` tables.

    pub use super::account_emails::*;
    pub use super::account_permissions::*;
    pub use super::account_sessions::*;
    pub use super::accounts::*;
}

pub mod project_queries {
    //! Queries for `projects` and `project_` tables.

    pub use super::project_invites::*;
    pub use super::project_members::*;
    pub use super::projects::*;
}

pub mod workflow_queries {
    //! Queries for `workflows` and `workflow_` tables.
}
