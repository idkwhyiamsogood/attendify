use crate::model::enums::staff_status::StaffStatus;

pub struct CreateStaffDto {
    pub last_name: Vec<u8>,
    pub first_name: Vec<u8>,
    pub middle_name: Option<Vec<u8>>,
    pub last_name_bidx: Vec<u8>,
    pub status: StaffStatus,
}

#[derive(Default)]
pub struct UpdateStaffDto {
    pub last_name: Option<(Vec<u8>, Vec<u8>)>,
    pub first_name: Option<Vec<u8>>,
    pub middle_name: Option<Option<Vec<u8>>>,
    pub status: Option<StaffStatus>,
}
