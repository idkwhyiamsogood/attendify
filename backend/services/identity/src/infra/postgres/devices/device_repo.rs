use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::infra::postgres::none_if_not_found;
use crate::model::params::devices::CreateDeviceParams;
use crate::model::entities::devices::{
    ActiveModel, Column, Entity as Device, Model as DeviceModel,
};
use crate::model::enums::device_status::DeviceStatus;

use super::device_repo_interface::IDeviceRepo;

pub struct DeviceRepo {
    db: DatabaseConnection,
}

impl DeviceRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn update(&self, mut active_model: ActiveModel) -> Result<Option<DeviceModel>, DbErr> {
        active_model.updated_at = Set(Utc::now());
        none_if_not_found(active_model.update(&self.db).await)
    }
}

#[async_trait]
impl IDeviceRepo for DeviceRepo {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<DeviceModel>, DbErr> {
        Device::find_by_id(id).one(&self.db).await
    }

    async fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<DeviceModel>, DbErr> {
        Device::find()
            .filter(Column::UserId.eq(user_id))
            .all(&self.db)
            .await
    }

    async fn get_active_by_user_id(&self, user_id: Uuid) -> Result<Vec<DeviceModel>, DbErr> {
        Device::find()
            .filter(Column::UserId.eq(user_id))
            .filter(Column::Status.eq(DeviceStatus::Active))
            .all(&self.db)
            .await
    }

    async fn get_by_device_id_hash(
        &self,
        device_id_hash: Vec<u8>,
    ) -> Result<Option<DeviceModel>, DbErr> {
        Device::find()
            .filter(Column::DeviceIdHash.eq(device_id_hash))
            .one(&self.db)
            .await
    }

    async fn create(&self, device: CreateDeviceParams) -> Result<DeviceModel, DbErr> {
        ActiveModel {
            id: Set(Uuid::now_v7()),
            user_id: Set(device.user_id),
            platform: Set(device.platform),
            status: Set(device.status),
            device_id_hash: Set(device.device_id_hash),
            public_key_spki: Set(device.public_key_spki),
            key_id: Set(device.key_id),

            ..Default::default()
        }
        .insert(&self.db)
        .await
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: DeviceStatus,
    ) -> Result<Option<DeviceModel>, DbErr> {
        self.update(ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        })
        .await
    }

    async fn update_key(
        &self,
        id: Uuid,
        public_key_spki: Option<Vec<u8>>,
        key_id: Option<String>,
    ) -> Result<Option<DeviceModel>, DbErr> {
        self.update(ActiveModel {
            id: Set(id),
            public_key_spki: Set(public_key_spki),
            key_id: Set(key_id),
            ..Default::default()
        })
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let res = Device::delete_by_id(id).exec(&self.db).await?;
        Ok(res.rows_affected > 0)
    }
}
