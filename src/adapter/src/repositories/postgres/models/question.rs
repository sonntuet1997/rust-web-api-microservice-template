use std::{
    io::{Error, ErrorKind},
    time::SystemTime,
};

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use rust_core::entities::question::{QuestionEntity, QuestionId};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize, Selectable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = super::super::schema::questions)]
#[cfg_attr(feature = "postgres", derive(diesel::pg::Pg))]
pub struct QuestionModel {
    pub id: i32,
    /// Title of the question.
    pub title: String,
    /// Content of the question.
    pub content: String,
    /// Optional tags associated with the question.
    pub tags: Option<Vec<Option<String>>>,

    pub created_on: SystemTime,
}

impl TryFrom<QuestionEntity> for QuestionModel {
    type Error = Error;

    fn try_from(entity: QuestionEntity) -> Result<QuestionModel, Self::Error> {
        let id = entity
            .id
            .0
            .parse()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid ID"))?;

        Ok(QuestionModel {
            id,
            title: entity.title,
            content: entity.content,
            tags: entity.tags.map(|v| v.into_iter().map(Some).collect()),
            created_on: SystemTime::now(),
        })
    }
}

impl Into<QuestionEntity> for QuestionModel {
    fn into(self) -> QuestionEntity {
        QuestionEntity {
            id: QuestionId(self.id.to_string()),
            title: self.title,
            content: self.content,
            tags: self.tags.map(|v| v.into_iter().flatten().collect()),
        }
    }
}
