use crate::types::{
    auth::{UserInfoWrapper, UserUpdateInfoWrapper},
    error::Error,
};

use super::requests::{request_get, request_put};

// Get current user info
pub async fn user() -> Result<UserInfoWrapper, Error> {
    request_get::<UserInfoWrapper>("/user".to_string()).await
}

// Save info of current user
pub async fn update(user_update_info: UserUpdateInfoWrapper) -> Result<UserInfoWrapper, Error> {
    request_put::<UserUpdateInfoWrapper, UserInfoWrapper>("/user".to_string(), user_update_info)
        .await
}
