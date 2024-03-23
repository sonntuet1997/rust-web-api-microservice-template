use crate::common::errors::CoreError;
use crate::entities::question::{QuestionEntity, QuestionId};
use crate::entities::question_filter::QuestionFilter;

use async_trait::async_trait;

#[async_trait]
pub trait QuestionPort {
    async fn add(&self, question: QuestionEntity) -> Result<QuestionEntity, CoreError>;
    async fn update(&self, question: QuestionEntity) -> Result<QuestionEntity, CoreError>;
    async fn delete(&self, question_id: &QuestionId) -> Result<(), CoreError>;
    async fn get(&self, question_id: &QuestionId) -> Result<QuestionEntity, CoreError>;
    async fn list(
        &self,
        question_filter: &QuestionFilter,
    ) -> Result<Vec<QuestionEntity>, CoreError>;
}
