use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::entities::devices::Model as DeviceModel;
use crate::model::enums::{device_status::DeviceStatus, platform::Platform};

/// `user_id` берётся из токена, а не из тела запроса.
/// `device_id` хеширует сервис; ключ — SPKI в base64, декодирует сервис.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDeviceDto {
    pub platform: Platform,
    pub device_id: String,
    pub public_key_spki: Option<String>,
    pub key_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDeviceStatusDto {
    pub status: DeviceStatus,
}

/// Без `device_id_hash` и самого ключа
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceResponseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub platform: Platform,
    pub status: DeviceStatus,
    pub key_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<DeviceModel> for DeviceResponseDto {
    fn from(m: DeviceModel) -> Self {
        Self {
            id: m.id,
            user_id: m.user_id,
            platform: m.platform,
            status: m.status,
            key_id: m.key_id,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}
