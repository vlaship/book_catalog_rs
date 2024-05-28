use crate::user::model::User;
use log::error;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{Error, Pool, Postgres, query, Row};

//language=SQL
const SELECT_BY_LOGIN: &'static str = "SELECT id, login, password FROM book_catalog_rs.users WHERE login = $1";
//language=SQL
const SELECT_USERS: &'static str = "SELECT login FROM book_catalog_rs.users";
//language=SQL
const INSERT_USER: &'static str = "INSERT INTO book_catalog_rs.users (id, login, password) VALUES ($1, $2, $3)";

pub async fn repo_create_user(
    pool: &Pool<Postgres>,
    user: &User,
) -> Result<(), Error> {
    let id = user.user_id;
    let login = user.login.clone();
    let password = user.password.clone();

    let result = query(INSERT_USER)
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

pub async fn repo_find_user_by_login(pool: &PgPool, user_login: &str) -> Result<User, Error> {
    let user = query(SELECT_BY_LOGIN)
        .bind(user_login)
        .map(|row: PgRow| User {
            user_id: row.get(0),
            login: row.get(1),
            password: row.get(2),
        })
        .fetch_one(pool)
        .await?;
    Ok(user)
}


pub async fn repo_list_users(pool: &PgPool) -> Result<Vec<String>, Error> {
    let logins = query(SELECT_USERS)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.get::<String, _>(0))
        .collect();

    Ok(logins)
}
