use async_trait::async_trait;
use sea_orm::DbErr;
use uuid::Uuid;

use crate::model::params::staff::{CreateStaffParams, UpdateStaffParams};
use crate::model::entities::staff::Model as StaffModel;

#[async_trait]
pub trait IStaffRepo: Send + Sync {
    async fn get_all(&self) -> Result<Vec<StaffModel>, DbErr>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<StaffModel>, DbErr>;
    async fn get_by_last_name_bidx(&self, bidx: Vec<u8>) -> Result<Vec<StaffModel>, DbErr>;
    async fn create(&self, staff: CreateStaffParams) -> Result<StaffModel, DbErr>;
    async fn update(&self, id: Uuid, staff: UpdateStaffParams) -> Result<Option<StaffModel>, DbErr>;
    /// `false`, если записи с таким id не было
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;
}
