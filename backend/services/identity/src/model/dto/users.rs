use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::entities::users::Model as UserModel;
use crate::model::enums::{user_kind::UserKind, user_status::UserStatus};

/// Пароля нет: пользователь создаётся в `pending_activation` и задаёт его при активации
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    pub kind: UserKind,
    pub student_id: Option<Uuid>,
    pub staff_id: Option<Uuid>,
    pub login: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserDto {
    pub login: Option<String>,
    pub status: Option<UserStatus>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordDto {
    pub old_password: String,
    pub new_password: String,
}

/// Без `password_hash`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponseDto {
    pub id: Uuid,
    pub kind: UserKind,
    pub status: UserStatus,
    pub student_id: Option<Uuid>,
    pub staff_id: Option<Uuid>,
    pub login: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserModel> for UserResponseDto {
    fn from(m: UserModel) -> Self {
        Self {
            id: m.id,
            kind: m.kind,
            status: m.status,
            student_id: m.student_id,
            staff_id: m.staff_id,
            login: m.login,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}
