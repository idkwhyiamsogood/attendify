use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "student_status")]
pub enum StudentStatus {
    #[sea_orm(string_value = "studying")]
    Studying,

    #[sea_orm(string_value = "academic_leave")]
    AcademicLeave,

    #[sea_orm(string_value = "expelled")]
    Expelled,

    #[sea_orm(string_value = "graduated")]
    Graduated,

    #[sea_orm(string_value = "transferred_out")]
    TransferredOut,
}
