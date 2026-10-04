use uuid::Uuid;

use crate::model::enums::group_status::GroupStatus;

pub struct CreateGroupDto {
    pub code: String,
    pub specialty_id: Uuid,
    pub course: i16,
    pub curator_id: Option<Uuid>,
    pub status: GroupStatus,
}

#[derive(Default)]
pub struct UpdateGroupDto {
    pub code: Option<String>,
    pub specialty_id: Option<Uuid>,
    pub course: Option<i16>,
    pub curator_id: Option<Option<Uuid>>,
    pub status: Option<GroupStatus>,
}
