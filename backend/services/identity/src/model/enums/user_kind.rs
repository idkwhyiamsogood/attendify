use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_kind")]
pub enum UserKind {
    #[sea_orm(string_value = "student")]
    Student,

    #[sea_orm(string_value = "staff")]
    Staff,

    #[sea_orm(string_value = "admin")]
    Admin,
}
