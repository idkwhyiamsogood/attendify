use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::infra::postgres::none_if_not_found;
use crate::model::params::users::CreateUserParams;
use crate::model::entities::users::{ActiveModel, Column, Entity as User, Model as UserModel};
use crate::model::enums::user_status::UserStatus;

use super::users_repo_interface::IUsersRepo;

pub struct UsersRepo {
    db: DatabaseConnection,
}

impl UsersRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn update(&self, mut active_model: ActiveModel) -> Result<Option<UserModel>, DbErr> {
        active_model.updated_at = Set(Utc::now());
        none_if_not_found(active_model.update(&self.db).await)
    }
}

#[async_trait]
impl IUsersRepo for UsersRepo {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<UserModel>, DbErr> {
        User::find_by_id(id).one(&self.db).await
    }

    async fn get_by_login(&self, login: &str) -> Result<Option<UserModel>, DbErr> {
        User::find()
            .filter(Column::Login.eq(login))
            .one(&self.db)
            .await
    }

    async fn get_by_student_id(&self, student_id: Uuid) -> Result<Option<UserModel>, DbErr> {
        User::find()
            .filter(Column::StudentId.eq(student_id))
            .one(&self.db)
            .await
    }

    async fn get_by_staff_id(&self, staff_id: Uuid) -> Result<Option<UserModel>, DbErr> {
        User::find()
            .filter(Column::StaffId.eq(staff_id))
            .one(&self.db)
            .await
    }

    async fn create(&self, user: CreateUserParams) -> Result<UserModel, DbErr> {
        ActiveModel {
            id: Set(Uuid::now_v7()),
            kind: Set(user.kind),
            status: Set(user.status),
            student_id: Set(user.student_id),
            staff_id: Set(user.staff_id),
            login: Set(user.login),
            password_hash: Set(user.password_hash),

            ..Default::default()
        }
        .insert(&self.db)
        .await
    }

    async fn update_login(&self, id: Uuid, login: String) -> Result<Option<UserModel>, DbErr> {
        self.update(ActiveModel {
            id: Set(id),
            login: Set(login),
            ..Default::default()
        })
        .await
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: UserStatus,
    ) -> Result<Option<UserModel>, DbErr> {
        self.update(ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        })
        .await
    }

    async fn update_password_hash(
        &self,
        id: Uuid,
        password_hash: Option<String>,
    ) -> Result<Option<UserModel>, DbErr> {
        self.update(ActiveModel {
            id: Set(id),
            password_hash: Set(password_hash),
            ..Default::default()
        })
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        let res = User::delete_by_id(id).exec(&self.db).await?;
        Ok(res.rows_affected > 0)
    }
}
