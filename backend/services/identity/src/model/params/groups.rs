use uuid::Uuid;

use crate::model::enums::group_status::GroupStatus;

/// Новая учебная группа (`groups`).
pub struct CreateGroupParams {
    /// Шифр группы, например `ИСП-21`
    pub code: String,
    /// FK на `specialties`
    pub specialty_id: Uuid,
    /// Курс, `1..=6`; диапазон проверяет сервис
    pub course: i16,
    /// FK на `staff`; `None` — куратор не назначен
    pub curator_id: Option<Uuid>,
    /// Начальный статус выбирает сервис, обычно `Active`
    pub status: GroupStatus,
}

/// Частичное обновление группы. `None` — поле не меняется.
#[derive(Default)]
pub struct UpdateGroupParams {
    pub code: Option<String>,
    pub specialty_id: Option<Uuid>,
    /// `1..=6`; диапазон проверяет сервис
    pub course: Option<i16>,
    /// `Some(None)` снимает куратора, `Some(Some(id))` назначает
    pub curator_id: Option<Option<Uuid>>,
    pub status: Option<GroupStatus>,
}
