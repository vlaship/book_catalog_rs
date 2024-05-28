use crate::state::AppState;
use crate::user::repo::repo_list_users;
use actix_web::web;
use sqlx::Error;

pub async fn svc_list_users(data: &web::Data<AppState>) -> Result<Vec<String>, Error> {
    repo_list_users(&data.db_pool).await
}
