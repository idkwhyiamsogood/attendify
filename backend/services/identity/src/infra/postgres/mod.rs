pub mod users;
pub mod devices;
pub mod specialties;
pub mod staff;
pub mod students;
pub mod groups;

use sea_orm::DbErr;

/// `ActiveModel::update` возвращает `RecordNotUpdated`, если строки с таким id нет.
/// Превращаем это в `Ok(None)`, остальные ошибки пробрасываем.
pub(crate) fn none_if_not_found<T>(res: Result<T, DbErr>) -> Result<Option<T>, DbErr> {
    match res {
        Ok(model) => Ok(Some(model)),
        Err(DbErr::RecordNotUpdated) => Ok(None),
        Err(e) => Err(e),
    }
}
