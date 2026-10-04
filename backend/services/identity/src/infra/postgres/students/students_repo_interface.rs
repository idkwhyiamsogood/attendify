use async_trait::async_trait;
use sea_orm::DbErr;
use uuid::Uuid;

use crate::model::params::students::{CreateStudentParams, UpdateStudentParams};
use crate::model::entities::students::Model as StudentModel;

#[async_trait]
pub trait IStudentsRepo: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<StudentModel>, DbErr>;
    async fn get_by_group_id(&self, group_id: Uuid) -> Result<Vec<StudentModel>, DbErr>;
    async fn create(&self, student: CreateStudentParams) -> Result<StudentModel, DbErr>;
    async fn update(
        &self,
        id: Uuid,
        student: UpdateStudentParams,
    ) -> Result<Option<StudentModel>, DbErr>;
    /// `false`, если записи с таким id не было
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;
}
