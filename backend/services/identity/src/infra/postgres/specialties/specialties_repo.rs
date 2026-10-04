use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::ActiveValue::{NotSet, Set};
use uuid::Uuid;

use crate::infra::postgres::none_if_not_found;
use crate::model::entities::specialties::{
    ActiveModel, Entity as Specialty, Model as SpecialtyModel,
};

use super::specialties_repo_interface::ISpecialtiesRepo;

pub struct SpecialtiesRepo {
    db: DatabaseConnection,
}

impl SpecialtiesRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ISpecialtiesRepo for SpecialtiesRepo {
    async fn get_specialties(&self) -> Result<Vec<SpecialtyModel>, DbErr> {
        Specialty::find().all(&self.db).await
    }

    async fn get_specialty_by_id(&self, id: Uuid) -> Result<Option<SpecialtyModel>, DbErr> {
        Specialty::find_by_id(id).one(&self.db).await
    }

    async fn create(&self, name: String) -> Result<SpecialtyModel, DbErr> {
        ActiveModel {
            id: Set(Uuid::now_v7()),
            name: Set(name),

            ..Default::default()
        }
        .insert(&self.db)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
    ) -> Result<Option<SpecialtyModel>, DbErr> {
        let active_model = ActiveModel {
            id: Set(id),
            name: name.map_or(NotSet, Set),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        none_if_not_found(active_model.update(&self.db).await)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let res = Specialty::delete_by_id(id).exec(&self.db).await?;
        Ok(res.rows_affected > 0)
    }
}
