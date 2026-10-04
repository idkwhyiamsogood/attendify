use uuid::Uuid;

use crate::model::enums::student_status::StudentStatus;

/// Новый студент (`students`).
///
/// ФИО — шифротекст AES-256-GCM, шифрует сервис.
pub struct CreateStudentParams {
    pub last_name: Vec<u8>,
    pub first_name: Vec<u8>,
    pub middle_name: Option<Vec<u8>>,
    /// FK на `groups`
    pub group_id: Uuid,
    /// Начальный статус выбирает сервис, обычно `Studying`
    pub status: StudentStatus,
}

/// Частичное обновление студента. `None` — поле не меняется.
#[derive(Default)]
pub struct UpdateStudentParams {
    /// Шифротекст
    pub last_name: Option<Vec<u8>>,
    /// Шифротекст
    pub first_name: Option<Vec<u8>>,
    /// Шифротекст; `Some(None)` удаляет отчество
    pub middle_name: Option<Option<Vec<u8>>>,
    /// Перевод в другую группу
    pub group_id: Option<Uuid>,
    pub status: Option<StudentStatus>,
}
