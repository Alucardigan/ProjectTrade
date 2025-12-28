use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth0UserInfo {
    pub sub: String, // User ID
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub user_email: String,
    pub user_name: String,
}
