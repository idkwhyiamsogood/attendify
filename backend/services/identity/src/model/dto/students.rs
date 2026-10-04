use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::dto::double_option;
use crate::model::enums::student_status::StudentStatus;

/// ФИО открытым текстом — шифрует сервис
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStudentDto {
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub group_id: Uuid,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStudentDto {
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    /// `null` удаляет отчество
    #[serde(default, deserialize_with = "double_option")]
    pub middle_name: Option<Option<String>>,
    pub group_id: Option<Uuid>,
    pub status: Option<StudentStatus>,
}

/// ФИО уже расшифрованы сервисом, поэтому `From<Model>` нет
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentResponseDto {
    pub id: Uuid,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub group_id: Uuid,
    pub status: StudentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
