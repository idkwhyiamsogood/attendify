use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::{NotSet, Set};
use uuid::Uuid;

use crate::infra::postgres::none_if_not_found;
use crate::model::dto::staff::{CreateStaffDto, UpdateStaffDto};
use crate::model::entities::staff::{ActiveModel, Column, Entity as Staff, Model as StaffModel};

use super::staff_repo_interface::IStaffRepo;

pub struct StaffRepo {
    db: DatabaseConnection,
}

impl StaffRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl IStaffRepo for StaffRepo {
    async fn get_all(&self) -> Result<Vec<StaffModel>, DbErr> {
        Staff::find().all(&self.db).await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<StaffModel>, DbErr> {
        Staff::find_by_id(id).one(&self.db).await
    }

    async fn get_by_last_name_bidx(&self, bidx: Vec<u8>) -> Result<Vec<StaffModel>, DbErr> {
        Staff::find()
            .filter(Column::LastNameBidx.eq(bidx))
            .all(&self.db)
            .await
    }

    async fn create(&self, staff: CreateStaffDto) -> Result<StaffModel, DbErr> {
        ActiveModel {
            id: Set(Uuid::now_v7()),
            last_name: Set(staff.last_name),
            first_name: Set(staff.first_name),
            middle_name: Set(staff.middle_name),
            last_name_bidx: Set(staff.last_name_bidx),
            status: Set(staff.status),

            ..Default::default()
        }
        .insert(&self.db)
        .await
    }

    async fn update(&self, id: Uuid, staff: UpdateStaffDto) -> Result<Option<StaffModel>, DbErr> {
        let (last_name, last_name_bidx) = staff
            .last_name
            .map_or((NotSet, NotSet), |(name, bidx)| (Set(name), Set(bidx)));

        let active_model = ActiveModel {
            id: Set(id),
            last_name,
            last_name_bidx,
            first_name: staff.first_name.map_or(NotSet, Set),
            middle_name: staff.middle_name.map_or(NotSet, Set),
            status: staff.status.map_or(NotSet, Set),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        none_if_not_found(active_model.update(&self.db).await)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let res = Staff::delete_by_id(id).exec(&self.db).await?;
        Ok(res.rows_affected > 0)
    }
}
