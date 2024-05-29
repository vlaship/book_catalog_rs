use std::sync::Arc;
use crate::user::repo::UserRepo;
use crate::err::errors::AppError;
use crate::user::model::User;

pub struct UserService {
    repo: Arc<UserRepo>,
}

impl UserService {
    pub fn new(user_repo: Arc<UserRepo>) -> Self {
        Self { repo: user_repo }
    }

    pub async fn list_users(&self) -> Result<Vec<String>, AppError> {
        self.repo.list_users().await
    }

    pub async fn get_user_by_login(
        &self,
        user_login: String,
    ) -> Result<User, AppError> {
        self.repo.find_user_by_login(&user_login).await
    }
}
