use uuid::Uuid;

use crate::model::enums::student_status::StudentStatus;

pub struct CreateStudentDto {
    pub last_name: Vec<u8>,
    pub first_name: Vec<u8>,
    pub middle_name: Option<Vec<u8>>,
    pub group_id: Uuid,
    pub status: StudentStatus,
}

#[derive(Default)]
pub struct UpdateStudentDto {
    pub last_name: Option<Vec<u8>>,
    pub first_name: Option<Vec<u8>>,
    pub middle_name: Option<Option<Vec<u8>>>,
    pub group_id: Option<Uuid>,
    pub status: Option<StudentStatus>,
}
