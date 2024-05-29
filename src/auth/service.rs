use std::sync::Arc;
use crate::auth::auth_utils::{hash_password, verify_password};
use crate::auth::model::{SigninRequest, SignupRequest};
use crate::user::repo::UserRepo;
use crate::err::errors::AppError;
use crate::user::model::User;
use crate::utils::id_generator::generate_id;

pub struct AuthService {
    repo: Arc<UserRepo>,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepo>) -> Self {
        Self { repo: user_repo }
    }

    pub async fn sign_up(&self, req: SignupRequest) -> Result<i64, AppError> {
        let user_id = generate_id();
        let password = hash_password(&req.password);
        let new_user = User {
            user_id,
            login: req.login.clone(),
            password,
        };
        let result = self.repo.repo_create_user(&new_user).await;
        match result {
            Ok(_) => Ok(user_id),
            Err(e) => Err(e),
        }
    }

    pub async fn sign_in(&self, req: &SigninRequest) -> Option<String> {
        if let Ok(user) = self.repo.find_user_by_login(&req.login).await {
            if verify_password(&req.password, &user.password) {
                return Some("dummy_jwt_token".to_string());
            }
        }
        None
    }
}
