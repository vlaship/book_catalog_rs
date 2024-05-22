use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: i64,
    pub login: String,
    pub password: String,
}
