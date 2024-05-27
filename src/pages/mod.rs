pub mod home;
pub mod login;
pub mod otp;
pub mod page_not_found;
pub mod password_verification;
pub mod register;

#[derive(Debug, Clone, Copy, Default, PartialEq, Hash, Eq)]
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
