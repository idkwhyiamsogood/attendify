use uuid::Uuid;

use crate::model::enums::{user_kind::UserKind, user_status::UserStatus};

/// Новая учётная запись (`users`).
///
/// Инвариант (проверяется в `before_save`): у `Student` задан только `student_id`,
/// у `Staff`/`Admin` — только `staff_id`.
pub struct CreateUserParams {
    pub kind: UserKind,
    /// Начальный статус выбирает сервис, обычно `PendingActivation`
    pub status: UserStatus,
    /// FK на `students`, только для `kind = Student`
    pub student_id: Option<Uuid>,
    /// FK на `staff`, только для `kind = Staff | Admin`
    pub staff_id: Option<Uuid>,
    pub login: String,
    /// Готовый хеш пароля; `None` — пароль ещё не задан (до активации)
    pub password_hash: Option<String>,
}
