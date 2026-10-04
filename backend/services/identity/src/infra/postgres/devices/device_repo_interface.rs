use async_trait::async_trait;
use sea_orm::DbErr;
use uuid::Uuid;

use crate::model::dto::devices::CreateDeviceDto;
use crate::model::entities::devices::Model as DeviceModel;
use crate::model::enums::device_status::DeviceStatus;

#[async_trait]
pub trait IDeviceRepo: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<DeviceModel>, DbErr>;
    async fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<DeviceModel>, DbErr>;
    async fn get_active_by_user_id(&self, user_id: Uuid) -> Result<Vec<DeviceModel>, DbErr>;
    async fn get_by_device_id_hash(
        &self,
        device_id_hash: Vec<u8>,
    ) -> Result<Option<DeviceModel>, DbErr>;
    async fn create(&self, device: CreateDeviceDto) -> Result<DeviceModel, DbErr>;
    async fn update_status(
        &self,
        id: Uuid,
        status: DeviceStatus,
    ) -> Result<Option<DeviceModel>, DbErr>;
    /// ключ и его id меняются только вместе; `None` в обоих — ключ отвязан
    async fn update_key(
        &self,
        id: Uuid,
        public_key_spki: Option<Vec<u8>>,
        key_id: Option<String>,
    ) -> Result<Option<DeviceModel>, DbErr>;
    /// `false`, если записи с таким id не было
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;
}
