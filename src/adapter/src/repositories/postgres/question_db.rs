use async_trait::async_trait;
use deadpool_diesel::postgres::Pool;
use diesel::{
    delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use rust_core::common::errors::CoreError;
use rust_core::entities::question::{QuestionEntity, QuestionId};
use rust_core::entities::question_filter::QuestionFilter;
use rust_core::ports::question::QuestionPort;

use crate::repositories::postgres::models::question::QuestionModel;
use crate::repositories::postgres::schema::questions::dsl::questions;
use crate::repositories::postgres::schema::questions::id;

// NOTE: path relative to Cargo.toml
pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/repositories/postgres/migrations");

#[derive(Clone)]
pub struct QuestionDBRepository {
    pub db: Pool,
}

impl QuestionDBRepository {
    pub fn new(db: Pool) -> Self {
        QuestionDBRepository { db }
    }
}

#[async_trait]
impl QuestionPort for QuestionDBRepository {
    async fn add(&self, question: QuestionEntity) -> Result<QuestionEntity, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let question =
                    QuestionModel::try_from(question).map_err(|_| CoreError::InternalError)?;
                let response = insert_into(questions)
                    .values(&question)
                    .get_result::<QuestionModel>(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError,
                    })
                    .unwrap();
                Ok(response.into())
            })
            .await
            .unwrap()
    }

    async fn update(&self, question: QuestionEntity) -> Result<QuestionEntity, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let question =
                    QuestionModel::try_from(question).map_err(|_| CoreError::InternalError)?;
                let response = update(questions.filter(id.eq(question.id)))
                    .set(&question)
                    .get_result::<QuestionModel>(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError,
                    })?
                    .into();

                Ok(response)
            })
            .await
            .unwrap()
    }

    async fn delete(&self, question_id: &QuestionId) -> Result<(), CoreError> {
        let question_id: i32 = question_id.to_string().parse()?;
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let _ = delete(questions.filter(id.eq(question_id)))
                    .execute(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError,
                    })?;

                Ok(())
            })
            .await
            .unwrap()
    }

    async fn get(&self, question_id: &QuestionId) -> Result<QuestionEntity, CoreError> {
        let question_id: i32 = question_id.to_string().parse()?;
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = questions
                    .select(QuestionModel::as_select())
                    .find(question_id)
                    .first(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError,
                    })?
                    .into();

                Ok(response)
            })
            .await
            .unwrap()
    }

    async fn list(
        &self,
        _question_filter: &QuestionFilter,
    ) -> Result<Vec<QuestionEntity>, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let question_list = questions
                    .select(QuestionModel::as_select())
                    .load(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError,
                    })?;

                Ok(question_list
                    .into_iter()
                    .map(|l| l.into())
                    .collect::<Vec<_>>())
            })
            .await
            .unwrap()
    }
}
