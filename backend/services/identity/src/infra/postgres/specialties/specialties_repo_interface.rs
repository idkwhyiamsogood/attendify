use crate::model::entities::specialties::Model as SpecialtyModel;
use async_trait::async_trait;
use sea_orm::DbErr;
use uuid::Uuid;

#[async_trait]
pub trait ISpecialtiesRepo: Send + Sync {
    async fn get_specialties(&self) -> Result<Vec<SpecialtyModel>, DbErr>;
    async fn get_specialty_by_id(&self, id: Uuid) -> Result<Option<SpecialtyModel>, DbErr>;
    async fn create(&self, name: String) -> Result<SpecialtyModel, DbErr>;
    async fn update(&self, id: Uuid, name: Option<String>) -> Result<Option<SpecialtyModel>, DbErr>;
    async fn delete(&self, id: Uuid) -> Result<Uuid, DbErr>;
}
