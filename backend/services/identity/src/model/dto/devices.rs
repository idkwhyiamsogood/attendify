use uuid::Uuid;

use crate::model::enums::{device_status::DeviceStatus, platform::Platform};

pub struct CreateDeviceDto {
    pub user_id: Uuid,
    pub platform: Platform,
    pub status: DeviceStatus,
    pub device_id_hash: Vec<u8>,
    pub public_key_spki: Option<Vec<u8>>,
    pub key_id: Option<String>,
}
