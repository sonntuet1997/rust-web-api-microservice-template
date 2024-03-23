use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use rust_core::common::errors::CoreError;
use rust_core::entities::question::{QuestionEntity, QuestionId};
use rust_core::entities::question_filter::QuestionFilter;
use rust_core::ports::question::QuestionPort;

#[derive(Clone, Debug)]
pub struct QuestionInMemoryRepository {
    pub questions: Arc<RwLock<HashMap<QuestionId, QuestionEntity>>>,
}

impl Default for QuestionInMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl QuestionInMemoryRepository {
    pub fn new() -> Self {
        QuestionInMemoryRepository {
            questions: Default::default(),
        }
    }
}

#[async_trait]
impl QuestionPort for QuestionInMemoryRepository {
    async fn add(&self, question: QuestionEntity) -> Result<QuestionEntity, CoreError> {
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question.clone());
        Ok(question.clone())
    }

    async fn update(&self, question: QuestionEntity) -> Result<QuestionEntity, CoreError> {
        self.get(&question.id).await?;
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question.clone());
        Ok(question.clone())
    }

    async fn delete(&self, question_id: &QuestionId) -> Result<(), CoreError> {
        self.get(question_id).await?;
        self.questions.write().await.remove(question_id);
        Ok(())
    }

    async fn get(&self, question_id: &QuestionId) -> Result<QuestionEntity, CoreError> {
        Ok(self
            .questions
            .read()
            .await
            .get(question_id)
            .ok_or(CoreError::NotFound)?
            .clone())
    }

    async fn list(
        &self,
        question_filter: &QuestionFilter,
    ) -> Result<Vec<QuestionEntity>, CoreError> {
        Ok(self
            .questions
            .read()
            .await
            .values()
            .skip(question_filter.pagination.start)
            .take(question_filter.pagination.end - question_filter.pagination.start)
            .cloned()
            .collect::<Vec<_>>())
    }
}
