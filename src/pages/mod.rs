pub mod home;
pub mod login;
pub mod otp;
pub mod register;
pub mod reset_password;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    OtpValidation,
    Register,
    Reset,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::OtpValidation => "/otp",
            Self::Register => "/register",
            Self::Reset => "/reset",
        }
    }
}
