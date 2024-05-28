use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!("./src/db/migrations")
        .run(pool)
        .await
        .expect("Failed to migrate database.");
}
