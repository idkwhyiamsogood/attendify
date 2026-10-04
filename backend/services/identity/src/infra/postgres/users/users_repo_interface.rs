use async_trait::async_trait;
use sea_orm::DbErr;
use uuid::Uuid;

use crate::model::params::users::CreateUserParams;
use crate::model::entities::users::Model as UserModel;
use crate::model::enums::user_status::UserStatus;

#[async_trait]
pub trait IUsersRepo: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<UserModel>, DbErr>;
    async fn get_by_login(&self, login: &str) -> Result<Option<UserModel>, DbErr>;
    async fn get_by_student_id(&self, student_id: Uuid) -> Result<Option<UserModel>, DbErr>;
    async fn get_by_staff_id(&self, staff_id: Uuid) -> Result<Option<UserModel>, DbErr>;
    async fn create(&self, user: CreateUserParams) -> Result<UserModel, DbErr>;
    async fn update_login(&self, id: Uuid, login: String) -> Result<Option<UserModel>, DbErr>;
    async fn update_status(&self, id: Uuid, status: UserStatus)
        -> Result<Option<UserModel>, DbErr>;
    /// `None` сбрасывает пароль (например, при повторной активации)
    async fn update_password_hash(
        &self,
        id: Uuid,
        password_hash: Option<String>,
    ) -> Result<Option<UserModel>, DbErr>;
    /// `false`, если записи с таким id не было
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;
}
