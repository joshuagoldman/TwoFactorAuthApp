pub mod home;
pub mod login;
pub mod otp;
pub mod password_verification;
pub mod register;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    OtpValidation,
    Register,
    Reset,
    Delete,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::OtpValidation => "/otp",
            Self::Register => "/register",
            Self::Reset => "/reset",
            Self::Delete => "/delete",
        }
    }
}
