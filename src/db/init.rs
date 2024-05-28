use std::env;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn init_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.")
}
