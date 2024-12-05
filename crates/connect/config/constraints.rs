//! Unfortunately, constraints cannot be automatically generated
//! from the database definition (unlike entities), so the list
//! must be maintained manually.

use sea_orm::SqlErr;

/// TODO.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ConstraintViolation {
    // TODO: Rem unknown,
    #[default]
    Unknown,
}

impl ConstraintViolation {
    /// Parses an error from unsuccessful SQL query.
    pub fn new(error: SqlErr) -> Option<Self> {
        match error {
            SqlErr::UniqueConstraintViolation(constraint) => {
                Self::parse_unique_constraint_violation(constraint.as_str())
            }
            SqlErr::ForeignKeyConstraintViolation(constraint) => {
                Self::parse_foreign_constraint_violation(constraint.as_str())
            }
            _ => None,
        }
    }

    /// Parses a [`ConstraintViolation`] from a [`SqlErr::UniqueConstraintViolation`].
    fn parse_unique_constraint_violation(constraint: &str) -> Option<Self> {
        match constraint {
            "" => Some(Self::Unknown),
            _ => None,
        }
    }

    /// Parses a [`ConstraintViolation`] from a [`SqlErr::ForeignKeyConstraintViolation`].
    fn parse_foreign_constraint_violation(constraint: &str) -> Option<Self> {
        match constraint {
            "" => Some(Self::Unknown),
            _ => None,
        }
    }
}
