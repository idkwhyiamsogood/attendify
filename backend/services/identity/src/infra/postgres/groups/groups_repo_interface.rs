use async_trait::async_trait;
use sea_orm::DbErr;
use uuid::Uuid;

use crate::model::params::groups::{CreateGroupParams, UpdateGroupParams};
use crate::model::entities::groups::Model as GroupModel;

#[async_trait]
pub trait IGroupsRepo: Send + Sync {
    async fn get_all(&self) -> Result<Vec<GroupModel>, DbErr>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<GroupModel>, DbErr>;
    async fn get_by_code(&self, code: &str) -> Result<Option<GroupModel>, DbErr>;
    async fn get_by_specialty_id(&self, specialty_id: Uuid) -> Result<Vec<GroupModel>, DbErr>;
    async fn get_by_curator_id(&self, curator_id: Uuid) -> Result<Vec<GroupModel>, DbErr>;
    async fn create(&self, group: CreateGroupParams) -> Result<GroupModel, DbErr>;
    async fn update(&self, id: Uuid, group: UpdateGroupParams) -> Result<Option<GroupModel>, DbErr>;
    /// `false`, если записи с таким id не было
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;
}
