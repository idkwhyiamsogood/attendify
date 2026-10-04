pub mod devices;
pub mod groups;
pub mod specialties;
pub mod staff;
pub mod students;
pub mod users;

use serde::{Deserialize, Deserializer};

/// Для `Option<Option<T>>` в update-запросах:
/// поля нет → `None` (не трогаем), `null` → `Some(None)` (обнуляем), значение → `Some(Some(v))`.
/// Использовать вместе с `#[serde(default)]`, иначе отсутствующее поле — ошибка.
pub(crate) fn double_option<'de, T, D>(de: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::<T>::deserialize(de).map(Some)
}
