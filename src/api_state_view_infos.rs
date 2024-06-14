use std::rc::Rc;

use leptos::{create_rw_signal, view, IntoView, View};

use crate::{
    api::{
        authorized_api::AuthorizedApi, otp_authorized_api::OtpAuthorizedApi,
        unauthorized_api::UnauthorizedApi,
    },
    misc::ApiStateViewInfo,
    pages::{
        self, home::Home, login::view::Login, otp::Otp,
        password_verification::view::PasswordVerification, register::view::Register, Page,
    },
};

pub fn get_unauth_view() -> Rc<impl Fn(UnauthorizedApi) -> View + 'static> {
    let unauth_view_func = move |unauth_api: UnauthorizedApi| {
        view! {
            <Login unauth_api
            />
        }
    };
    let unauth_view_func = Rc::new(unauth_view_func);
    unauth_view_func
}

pub fn get_login_view() -> ApiStateViewInfo<View> {
    let unauth_view_func = move |unauth_api: UnauthorizedApi| {
        view! {
            <Login unauth_api
            />
        }
    };
    let unauth_view_func = Rc::new(unauth_view_func);

    let page = Page::Login;

    ApiStateViewInfo {
        page,
        view: crate::misc::ApiStateView::UnAuth(unauth_view_func),
    }
}

pub fn get_register_view() -> ApiStateViewInfo<View> {
    let register_view_func = move |unatuhorized_api: UnauthorizedApi| {
        view! {
            <Register unatuhorized_api
            />
        }
    };
    let register_view_func = Rc::new(register_view_func);

    let page = Page::Register;

    ApiStateViewInfo {
        page,
        view: crate::misc::ApiStateView::UnAuth(register_view_func),
    }
}

pub fn get_otp_view(
    unauth_view_func: Rc<impl Fn(UnauthorizedApi) -> View + 'static>,
) -> ApiStateViewInfo<View> {
    let otp_auth_view_func = move |otp_auth_api: OtpAuthorizedApi| {
        view! {
            <Otp otp_auth_api ></Otp>
        }
        .into_view()
    };
    let otp_auth_view_func = Rc::new(otp_auth_view_func);

    let page = Page::OtpValidation;

    ApiStateViewInfo {
        page,
        view: crate::misc::ApiStateView::OTPAuth(unauth_view_func, otp_auth_view_func),
    }
}

pub fn get_home_view(
    unauth_view_func: Rc<impl Fn(UnauthorizedApi) -> View + 'static>,
) -> ApiStateViewInfo<View> {
    let profile_info = create_rw_signal(None);
    let auth_view_func_home = move |authorized_api: AuthorizedApi| {
        view! {
            <Home authorized_api
                  profile_info
            />
        }
    };
    let auth_view_func_home = Rc::new(auth_view_func_home);

    let page = Page::Home;

    ApiStateViewInfo {
        page,
        view: crate::misc::ApiStateView::Auth(unauth_view_func, auth_view_func_home),
    }
}

pub fn get_reset_view(
    unauth_view_func: Rc<impl Fn(UnauthorizedApi) -> View + 'static>,
) -> ApiStateViewInfo<View> {
    let auth_view_func_reset = move |authorized_api: AuthorizedApi| {
        view! {
            <PasswordVerification authorized_api
                  action_type= pages::password_verification::misc::PassVerificationAction::ResetPassword
            />
        }
    };
    let auth_view_func_reset = Rc::new(auth_view_func_reset);

    let page = Page::Reset;

    ApiStateViewInfo {
        page,
        view: crate::misc::ApiStateView::Auth(unauth_view_func, auth_view_func_reset),
    }
}

pub fn get_delete_view(
    unauth_view_func: Rc<impl Fn(UnauthorizedApi) -> View + 'static>,
) -> ApiStateViewInfo<View> {
    let auth_view_func_delete = move |authorized_api: AuthorizedApi| {
        view! {
            <PasswordVerification authorized_api
                  action_type= pages::password_verification::misc::PassVerificationAction::DeleteAccount
            />
        }
    };
    let auth_view_func_delete = Rc::new(auth_view_func_delete);

    let page = Page::Reset;

    ApiStateViewInfo {
        page,
        view: crate::misc::ApiStateView::Auth(unauth_view_func, auth_view_func_delete),
    }
}
