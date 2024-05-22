use crate::state::AppState;
use actix_web::web;
use crate::db::users::list_users as db_list_users;
use sqlx::Error;

pub async fn list_users(data: &web::Data<AppState>) -> Result<Vec<String>, Error> {
    db_list_users(&data.db_pool).await
}
