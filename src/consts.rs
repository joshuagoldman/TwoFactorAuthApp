use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{misc::ApiState, pages::Page};

lazy_static! {
    pub static ref DEFAULT_API_URL: String = std::env!("DEFAULT_API_URL").to_string();
}

lazy_static! {
    pub static ref API_TOKEN_STORAGE_KEY: String = std::env!("API_TOKEN_STORAGE_KEY").to_string();
}

lazy_static! {
    pub static ref API_TOKEN_OTP_KEY: String = std::env!("API_TOKEN_OTP_KEY").to_string();
}

lazy_static! {
    pub static ref USER_NAME_FIELD_STR: String = "User Name".to_string();
}

lazy_static! {
    pub static ref FIRST_NAME_FIELD_STR: String = "First Name".to_string();
}

lazy_static! {
    pub static ref LAST_NAME_FIELD_STR: String = "Last Name".to_string();
}

lazy_static! {
    pub static ref EMAIL_FIELD_STR: String = "Email".to_string();
}

lazy_static! {
    pub static ref PASSWORD_FIELD_STR: String = "Password".to_string();
}

lazy_static! {
    pub static ref REPEAT_PASSWORD_FIELD_STR: String = "Repeat Password".to_string();
}

lazy_static! {
    pub static ref CURRENT_PASSWORD_FIELD_STR: String = "Current Password".to_string();
}

lazy_static! {
    pub static ref NEW_PASSWORD_FIELD_STR: String = "New Password".to_string();
}

lazy_static! {
    pub static ref REPEAT_NEW_PASSWORD_FIELD_STR: String = "Repeat New Password".to_string();
}
