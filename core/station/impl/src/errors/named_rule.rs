use orbit_essentials::api::DetailableError;
use thiserror::Error;

/// Container for named rule errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum NamedRuleError {
    /// The named rule was not found.
    #[error("The named rule with id {id} was not found.")]
    NotFound {
        /// The named rule id.
        id: String,
    },

    // The named rule name is invalid.
    #[error("The named rule name must be between {min_length} and {max_length} characters.")]
    InvalidName {
        min_length: usize,
        max_length: usize,
    },

    // The named rule description is invalid.
    #[error(
        "The named rule description must be between {min_length} and {max_length} characters."
    )]
    InvalidDescription {
        min_length: usize,
        max_length: usize,
    },

    // Invalid rule.
    #[error("The rule is invalid.")]
    InvalidRule { error: String },

    // The rule is incompatible with a linked request policy.
    #[error("The rule is incompatible with a linked request policy with id {policy_id}.")]
    IncompatibleWithLinkedPolicy { policy_id: String, error: String },

    // The named rule already exists.
    #[error("The named rule already exists.")]
    AlreadyExists { name: String },

    // The named rule cannot be removed because it is in use.
    #[error("The named rule cannot be removed because it is in use.")]
    InUse,

    // The named rule has a circular reference.
    #[error("The named rule has a circular reference.")]
    CircularReference,

    // The named rule with id `{id}` already exists.
    #[error(r#"The named rule with id `{id}` already exists."#)]
    IdAlreadyExists { id: String },
}

impl DetailableError for NamedRuleError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            NamedRuleError::NotFound { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }

            NamedRuleError::InvalidName {
                min_length,
                max_length,
            } => {
                details.insert("min_length".to_string(), min_length.to_string());
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }

            NamedRuleError::InvalidDescription {
                min_length,
                max_length,
            } => {
                details.insert("min_length".to_string(), min_length.to_string());
                details.insert("max_length".to_string(), max_length.to_string());
                Some(details)
            }

            NamedRuleError::InvalidRule { error } => {
                details.insert("error".to_string(), error.to_string());
                Some(details)
            }

            NamedRuleError::AlreadyExists { name } => {
                details.insert("name".to_string(), name.to_string());
                Some(details)
            }

            NamedRuleError::InUse => None,

            NamedRuleError::CircularReference => None,

            NamedRuleError::IdAlreadyExists { id } => {
                details.insert("id".to_string(), id.to_string());
                Some(details)
            }

            NamedRuleError::IncompatibleWithLinkedPolicy { policy_id, error } => {
                details.insert("policy_id".to_string(), policy_id.to_string());
                details.insert("error".to_string(), error.to_string());
                Some(details)
            }
        }
    }
}
