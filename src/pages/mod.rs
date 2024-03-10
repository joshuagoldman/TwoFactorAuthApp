pub mod home;
pub mod login;
pub mod otp;
pub mod register;

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
