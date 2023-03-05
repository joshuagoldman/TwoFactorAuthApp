pub mod home;
pub mod login;
pub mod register;
pub mod otp;

pub use self::{home::*, login::*, register::*, otp::*};

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,

    OtpValidation,

    Register,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::OtpValidation => "/otp",
            Self::Register => "/register",
        }
    }
}
