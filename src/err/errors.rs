use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to create user: {0}")]
    CreateUserError(String),

    #[error("Failed to find user by login: {0}")]
    FindUserError(String),

    #[error("Failed to list users")]
    ListUsersError(),

    #[error("Bad Request")]
    BadRequest(),

    #[error("Unauthorized")]
    Unauthorized(),

    #[error("Not Found")]
    NotFound(),

    #[error("Internal Server Error")]
    InternalServerError(),
}
