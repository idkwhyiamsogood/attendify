use uuid::Uuid;

use crate::model::enums::{device_status::DeviceStatus, platform::Platform};

/// Новое устройство пользователя (`devices`).
pub struct CreateDeviceParams {
    /// FK на `users`; сервис берёт его из токена, а не из запроса
    pub user_id: Uuid,
    pub platform: Platform,
    /// Начальный статус выбирает сервис, обычно `Active`
    pub status: DeviceStatus,
    /// Хеш идентификатора устройства; сырой `device_id` в БД не хранится
    pub device_id_hash: Vec<u8>,
    /// Публичный ключ устройства в DER (SPKI), уже декодированный из base64
    pub public_key_spki: Option<Vec<u8>>,
    /// Идентификатор ключа; задаётся вместе с `public_key_spki`
    pub key_id: Option<String>,
}
