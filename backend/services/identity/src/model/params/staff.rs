use crate::model::enums::staff_status::StaffStatus;

/// Новый сотрудник (`staff`).
///
/// ФИО — шифротекст AES-256-GCM, шифрует сервис.
pub struct CreateStaffParams {
    pub last_name: Vec<u8>,
    pub first_name: Vec<u8>,
    pub middle_name: Option<Vec<u8>>,
    /// Blind index фамилии для поиска; считается сервисом от открытой фамилии
    pub last_name_bidx: Vec<u8>,
    /// Начальный статус выбирает сервис, обычно `Active`
    pub status: StaffStatus,
}

/// Частичное обновление сотрудника. `None` — поле не меняется.
#[derive(Default)]
pub struct UpdateStaffParams {
    /// `(шифротекст фамилии, её bidx)` — меняются только вместе,
    /// иначе поиск по bidx перестанет находить запись
    pub last_name: Option<(Vec<u8>, Vec<u8>)>,
    /// Шифротекст
    pub first_name: Option<Vec<u8>>,
    /// Шифротекст; `Some(None)` удаляет отчество
    pub middle_name: Option<Option<Vec<u8>>>,
    pub status: Option<StaffStatus>,
}
