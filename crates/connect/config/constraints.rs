//! Includes a comprehensive list of all constraint violations.
//!

/// Comprehensive list of all constraint violations.
///
/// Includes both unique constraint violations and
/// foreign key constraint violations.
///
/// Unfortunately, constraints cannot be automatically generated
/// from the database definition (unlike entities), so the list
/// must be maintained manually.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[must_use = "constraints do nothing unless you use them"]
pub enum ConstraintViolation {
    // TODO: Remove Unknown,
    #[default]
    Unknown,
}

impl ConstraintViolation {
    /// Returns a new [`ConstraintViolation`].
    pub fn new(constraint: &str) -> Option<Self> {
        todo!()
    }

    /// Parses a [`ConstraintViolation`] from a unique constraint violation.
    fn parse_unique_constraint_violation(constraint: &str) -> Option<Self> {
        match constraint {
            _ => None,
        }
    }

    /// Parses a [`ConstraintViolation`] from a foreign key constraint violation.
    fn parse_foreign_constraint_violation(constraint: &str) -> Option<Self> {
        match constraint {
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ConstraintViolation, DatabaseResult};

    #[test]
    fn unique_constraint_violation() -> DatabaseResult<()> {
        let _ = ConstraintViolation::new();
        Ok(())
    }

    #[test]
    fn foreign_key_constraint_violation() -> DatabaseResult<()> {
        let _ = ConstraintViolation::new();
        Ok(())
    }
}
