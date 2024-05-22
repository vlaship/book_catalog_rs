use crate::user::model::User;
use log::error;
use sqlx::postgres::PgPool;
use sqlx::{Error, Row};

pub async fn create_user(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user: &User,
) -> Result<(), sqlx::Error> {
    let id = user.user_id;
    let login = user.login.clone();
    let password = user.password.clone();

    let result = sqlx::query("INSERT INTO book_catalog.users (id, login, password) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(login)
        .bind(password)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to create user: {}", e.to_string());
            Err(e)
        }
    }
}

pub async fn find_user_by_login(pool: &PgPool, user_login: &str) -> Result<User, Error> {
    let user = sqlx::query("SELECT id, login, password FROM book_catalog.users WHERE login = $1")
        .bind(user_login)
        .map(|row: sqlx::postgres::PgRow| User {
            user_id: row.get(0),
            login: row.get(1),
            password: row.get(2),
        })
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<String>, Error> {
    let logins = sqlx::query("SELECT login FROM book_catalog.users")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.get::<String, _>(0))
        .collect();

    Ok(logins)
}
