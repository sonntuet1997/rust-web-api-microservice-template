use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::errors::CoreError;

/// Represents pagination parameters.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationEntity {
    /// Start index for pagination.
    pub start: usize,
    /// End index for pagination.
    pub end: usize,
    /// Optional sorting criteria.
    pub sort: Option<Vec<String>>,
}

impl PaginationEntity {
    /// Constructs a `PaginationEntity` from query parameters.
    ///
    /// # Arguments
    ///
    /// * `query` - A HashMap containing query parameters.
    ///
    /// # Returns
    ///
    /// A Result containing the constructed `PaginationEntity` or an `Error` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use rust_core::entities::pagination_entity::PaginationEntity;
    ///
    /// let mut query_params = HashMap::new();
    /// query_params.insert("start".to_string(), "0".to_string());
    /// query_params.insert("end".to_string(), "10".to_string());
    ///
    /// match PaginationEntity::from_query(&query_params) {
    ///     Ok(pagination_entity) => {
    ///         println!("Parsed pagination entity: {:?}", pagination_entity);
    ///     }
    ///     Err(err) => {
    ///         eprintln!("Failed to parse pagination entity: {:?}", err);
    ///     }
    /// }
    /// ```
    pub fn from_query(query: &HashMap<String, String>) -> Result<Self, CoreError> {
        let start = query
            .get("start")
            .unwrap_or(&"0".to_string())
            .parse::<usize>()?;
        let end = query
            .get("end")
            .unwrap_or(&"10".to_string())
            .parse::<usize>()?;
        Ok(PaginationEntity {
            start,
            end,
            sort: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_entity_from_query() {
        // Test case 1: Valid query parameters
        let mut query_params_1 = HashMap::new();
        query_params_1.insert("start".to_string(), "0".to_string());
        query_params_1.insert("end".to_string(), "10".to_string());

        match PaginationEntity::from_query(&query_params_1) {
            Ok(pagination_entity) => {
                assert_eq!(pagination_entity.start, 0);
                assert_eq!(pagination_entity.end, 10);
                assert_eq!(pagination_entity.sort, None);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_pagination_entity_from_query_parse_parameters_failed() {
        // Test case 2: Parse parameters failed
        let mut query_params_2 = HashMap::new();
        query_params_2.insert("start".to_string(), "abs".to_string());
        query_params_2.insert("end".to_string(), "10".to_string());
        match PaginationEntity::from_query(&query_params_2) {
            Ok(_) => {
                panic!("Expected an error, but got Ok");
            }
            Err(err) => match err {
                CoreError::ParseError(_) => {}
                _ => {
                    panic!("Expected ParseError error, but got {:?}", err);
                }
            },
        }
    }
}
