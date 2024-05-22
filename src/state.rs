use sqlx::postgres::PgPool;

pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        AppState { db_pool }
    }
}
