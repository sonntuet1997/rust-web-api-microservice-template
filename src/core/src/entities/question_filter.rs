use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::errors::CoreError;
use crate::entities::filter_entity::FilterEntity;
use crate::entities::pagination_entity::PaginationEntity;

/// Represents filters for querying questions.
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionFilter {
    /// Pagination parameters for the query.
    pub pagination: PaginationEntity,
}

impl QuestionFilter {
    /// Constructs a `QuestionFilter` from query parameters.
    ///
    /// # Arguments
    ///
    /// * `query` - A HashMap containing query parameters.
    ///
    /// # Returns
    ///
    /// A Result containing the constructed `QuestionFilter` or an `Error` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use rust_core::entities::question_filter::QuestionFilter;
    ///
    /// let mut query_params = HashMap::new();
    /// query_params.insert("start".to_string(), "0".to_string());
    /// query_params.insert("end".to_string(), "10".to_string());
    ///
    /// match QuestionFilter::from_query(&query_params) {
    ///     Ok(question_filter) => {
    ///         println!("Parsed question filter: {:?}", question_filter);
    ///     }
    ///     Err(err) => {
    ///         eprintln!("Failed to parse question filter: {:?}", err);
    ///     }
    /// }
    /// ```
    pub fn from_query(query: &HashMap<String, String>) -> Result<Self, CoreError> {
        Ok(QuestionFilter {
            pagination: PaginationEntity::from_query(query)?,
        })
    }
}

impl FilterEntity<QuestionFilter> for QuestionFilter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_filter_from_query() {
        // Test case 1: Valid query parameters
        let mut query_params_1 = HashMap::new();
        query_params_1.insert("start".to_string(), "0".to_string());
        query_params_1.insert("end".to_string(), "10".to_string());

        match QuestionFilter::from_query(&query_params_1) {
            Ok(question_filter) => {
                assert_eq!(question_filter.pagination.start, 0);
                assert_eq!(question_filter.pagination.end, 10);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_question_filter_from_query_parse_error() {
        // Test case 2: Parse parameters error
        let mut query_params_2 = HashMap::new();
        query_params_2.insert("start".to_string(), "asd".to_string());
        query_params_2.insert("end".to_string(), "10".to_string());

        match QuestionFilter::from_query(&query_params_2) {
            Ok(_) => {
                panic!("Expected an error, but got Ok");
            }
            Err(err) => match err {
                CoreError::ParseError(_) => {}
                _ => {
                    panic!("Expected MissingParameters error, but got {:?}", err);
                }
            },
        }
    }
}
