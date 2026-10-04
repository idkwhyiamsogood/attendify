use uuid::Uuid;

use crate::model::enums::{user_kind::UserKind, user_status::UserStatus};

pub struct CreateUserDto {
    pub kind: UserKind,
    pub status: UserStatus,
    pub student_id: Option<Uuid>,
    pub staff_id: Option<Uuid>,
    pub login: String,
    pub password_hash: Option<String>,
}
