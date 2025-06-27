use super::ConfigField;
use std::collections::HashSet;

use syn::{Token, punctuated::Punctuated};

#[derive(Debug)]
pub enum ConfigValidationError {
    Duplicate { key: String },
    KeyPathConflict { key: String },
}

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate(
        fields: &Punctuated<ConfigField, Token![,]>,
    ) -> Result<(), ConfigValidationError> {
        Self::validate_duplicate_keys(fields)?;
        Self::validate_key_path_conflicts(fields)?;
        Ok(())
    }

    fn validate_duplicate_keys(
        fields: &Punctuated<ConfigField, Token![,]>,
    ) -> Result<(), ConfigValidationError> {
        let mut seen_keys = HashSet::new();
        for field in fields {
            let key = field
                .path
                .segments
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(".");

            if !seen_keys.insert(key.clone()) {
                return Err(ConfigValidationError::Duplicate { key });
            }
        }
        Ok(())
    }

    fn validate_key_path_conflicts(
        fields: &Punctuated<ConfigField, Token![,]>,
    ) -> Result<(), ConfigValidationError> {
        let paths: Vec<String> = fields
            .iter()
            .map(|field| {
                field
                    .path
                    .segments
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
            })
            .collect();

        for path in &paths {
            let has_children = paths
                .iter()
                .any(|other| other != path && other.starts_with(&format!("{}.", path)));

            if has_children {
                return Err(ConfigValidationError::KeyPathConflict { key: path.clone() });
            }
        }
        Ok(())
    }
}
