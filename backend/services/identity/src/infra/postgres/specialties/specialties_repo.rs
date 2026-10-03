use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

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
            id: Set(Uuid::new_v4()),
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
        let mut active_model = ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        if let Some(name) = name {
            active_model.name = Set(name);
        }
        active_model.updated_at = Set(Utc::now());

        match active_model.update(&self.db).await {
            Ok(model) => Ok(Some(model)),
            Err(DbErr::RecordNotUpdated) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<Uuid, DbErr> {
        Specialty::delete_by_id(id).exec(&self.db).await.map(|_| id)
    }
}
