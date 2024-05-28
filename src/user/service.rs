use crate::user::repo::UserRepo;
use actix_web::web::Path;
use crate::err::errors::AppError;
use crate::user::model::User;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepo,
}

impl UserService {
    pub fn new(user_repo: UserRepo) -> Self {
        Self { repo: user_repo }
    }

    pub async fn list_users(&self) -> Result<Vec<String>, AppError> {
        self.repo.list_users().await
    }

    pub async fn get_user_by_login(
        &self,
        user_login: &Path<String>,
    ) -> Result<User, AppError> {
        self.repo.find_user_by_login(&user_login).await
    }
}
