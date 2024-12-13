//! Includes a comprehensive list of all constraint violations.

use std::ops::Deref;

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
    // TODO: Remove Self::Unknown.
    #[default]
    Unknown,
}

impl ConstraintViolation {
    /// Returns a new [`ConstraintViolation`].
    pub fn new(constraint: &str) -> Option<Self> {
        match constraint {
            _ => None,
        }
    }

    /// Returns the constraint name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            ConstraintViolation::Unknown => "unknown",
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
        let _ = ConstraintViolation::new("unknown_constraint");
    }

    #[test]
    fn stringify_constraint_violation() {
        let _ = ConstraintViolation::Unknown.as_str();
    }
}
