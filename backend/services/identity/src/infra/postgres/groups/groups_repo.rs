use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::{NotSet, Set};
use uuid::Uuid;

use crate::infra::postgres::none_if_not_found;
use crate::model::dto::groups::{CreateGroupDto, UpdateGroupDto};
use crate::model::entities::groups::{ActiveModel, Column, Entity as Group, Model as GroupModel};

use super::groups_repo_interface::IGroupsRepo;

pub struct GroupsRepo {
    db: DatabaseConnection,
}

impl GroupsRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl IGroupsRepo for GroupsRepo {
    async fn get_all(&self) -> Result<Vec<GroupModel>, DbErr> {
        Group::find().all(&self.db).await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<GroupModel>, DbErr> {
        Group::find_by_id(id).one(&self.db).await
    }

    async fn get_by_code(&self, code: &str) -> Result<Option<GroupModel>, DbErr> {
        Group::find()
            .filter(Column::Code.eq(code))
            .one(&self.db)
            .await
    }

    async fn get_by_specialty_id(&self, specialty_id: Uuid) -> Result<Vec<GroupModel>, DbErr> {
        Group::find()
            .filter(Column::SpecialtyId.eq(specialty_id))
            .all(&self.db)
            .await
    }

    async fn get_by_curator_id(&self, curator_id: Uuid) -> Result<Vec<GroupModel>, DbErr> {
        Group::find()
            .filter(Column::CuratorId.eq(curator_id))
            .all(&self.db)
            .await
    }

    async fn create(&self, group: CreateGroupDto) -> Result<GroupModel, DbErr> {
        ActiveModel {
            id: Set(Uuid::now_v7()),
            code: Set(group.code),
            specialty_id: Set(group.specialty_id),
            course: Set(group.course),
            curator_id: Set(group.curator_id),
            status: Set(group.status),

            ..Default::default()
        }
        .insert(&self.db)
        .await
    }

    async fn update(&self, id: Uuid, group: UpdateGroupDto) -> Result<Option<GroupModel>, DbErr> {
        let active_model = ActiveModel {
            id: Set(id),
            code: group.code.map_or(NotSet, Set),
            specialty_id: group.specialty_id.map_or(NotSet, Set),
            course: group.course.map_or(NotSet, Set),
            curator_id: group.curator_id.map_or(NotSet, Set),
            status: group.status.map_or(NotSet, Set),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        none_if_not_found(active_model.update(&self.db).await)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let res = Group::delete_by_id(id).exec(&self.db).await?;
        Ok(res.rows_affected > 0)
    }
}
