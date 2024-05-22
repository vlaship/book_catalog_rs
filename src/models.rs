use serde::{Deserialize, Serialize};
use validator_derive::Validate; // Import the Validate derive macro

#[derive(Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(length(min = 3, max = 20))]
    pub login: String,
    
    #[validate(length(min = 12, max = 100))]
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub id: String,
}

#[derive(Deserialize, Validate)]
pub struct SigninRequest {
    #[validate(length(min = 3, max = 20))]
    pub login: String,
    
    #[validate(length(min = 12, max = 100))]
    pub password: String,
}

#[derive(Serialize)]
pub struct SigninResponse {
    pub jwt: String,
}
