use crate::user::model::User;
use log::error;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{query, Row};
use crate::err::errors::AppError;

//language=SQL
const SELECT_BY_LOGIN: &'static str = "SELECT id, login, password FROM book_catalog_rs.users WHERE login = $1";
//language=SQL
const SELECT_USERS: &'static str = "SELECT login FROM book_catalog_rs.users";
//language=SQL
const INSERT_USER: &'static str = "INSERT INTO book_catalog_rs.users (id, login, password) VALUES ($1, $2, $3)";

pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn repo_create_user(
        &self,
        user: &User,
    ) -> Result<i64, AppError> {
        let id = user.user_id;
        let login = user.login.clone();
        let password = user.password.clone();

        let result = query(INSERT_USER)
            .bind(id)
            .bind(login.clone())
            .bind(password)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(id),
            Err(e) => {
                error!("Failed to create user with login: [{}] err: {}", login, e.to_string());
                Err(AppError::CreateUserError(login))
            }
        }
    }

    pub async fn find_user_by_login(
        &self,
        login: &str,
    ) -> Result<User, AppError> {
        let result = query(SELECT_BY_LOGIN)
            .bind(login)
            .map(|row: PgRow| User {
                user_id: row.get(0),
                login: row.get(1),
                password: row.get(2),
            })
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("Failed to find user by login: [{}] err: {}", login, e.to_string());
                Err(AppError::FindUserError(login.to_string()))
            }
        }
    }

    pub async fn list_users(&self) -> Result<Vec<String>, AppError> {
        let result = query(SELECT_USERS)
            .fetch_all(&self.pool)
            .await
            .map(|rows| {
                rows.into_iter()
                    .map(|row| row.get::<String, _>(0))
                    .collect::<Vec<String>>()
            });

        match result {
            Ok(logins) => Ok(logins),
            Err(e) => {
                error!("Failed to list users. err: {}", e.to_string());
                Err(AppError::ListUsersError())
            }
        }
    }
}
