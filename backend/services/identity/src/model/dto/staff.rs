use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::dto::double_option;
use crate::model::enums::staff_status::StaffStatus;

/// ФИО открытым текстом — шифрует и считает bidx сервис
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStaffDto {
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStaffDto {
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    /// `null` удаляет отчество
    #[serde(default, deserialize_with = "double_option")]
    pub middle_name: Option<Option<String>>,
    pub status: Option<StaffStatus>,
}

/// ФИО уже расшифрованы сервисом, поэтому `From<Model>` нет
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffResponseDto {
    pub id: Uuid,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub status: StaffStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
