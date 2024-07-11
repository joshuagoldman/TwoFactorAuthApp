use leptos::{component, create_action, view, IntoView, Show, Signal, SignalGet};

use crate::{
    api::unauthorized_api::UnauthorizedApi,
    api_state::check_user_logged_in::check_user_logged_in,
    consts::DEFAULT_API_URL,
    misc::{go_to_page, ApiSignals, ApiStateView, ApiStateViewInfo},
};

#[component]
pub fn ApiStateCheckView<F>(view_info: ApiStateViewInfo<F>) -> impl IntoView
where
    F: IntoView + 'static + Clone,
{
    let api_signals = ApiSignals::new();
    let check_logged_in = create_action(move |api_set_signals: &ApiSignals| {
        check_user_logged_in(api_set_signals.clone(), view_info.page)
    });

    check_logged_in.dispatch(api_signals);

    let view_signal = Signal::derive(move || {
        let view = match view_info.view.clone() {
            ApiStateView::UnAuth(view_func_unauth) => {
                if let Some(unauth_api) = api_signals.unauth.get() {
                    view_func_unauth(unauth_api)
                } else {
                    let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                    view_func_unauth(unauth_api)
                }
            }
            ApiStateView::OTPAuth(view_func_unauth, view_func_otp_auth) => {
                if let Some(otpAuth) = api_signals.otpauth.get() {
                    view_func_otp_auth(otpAuth)
                } else if let Some(unauth_api) = api_signals.unauth.get() {
                    go_to_page(crate::pages::Page::Login);
                    view_func_unauth(unauth_api)
                } else {
                    let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                    go_to_page(crate::pages::Page::Login);
                    view_func_unauth(unauth_api)
                }
            }
            ApiStateView::Auth(view_func_unauth, view_func_auth) => {
                if let Some(authApi) = api_signals.auth.get() {
                    view_func_auth(authApi)
                } else if let Some(unauth_api) = api_signals.unauth.get() {
                    go_to_page(crate::pages::Page::Login);
                    view_func_unauth(unauth_api)
                } else {
                    go_to_page(crate::pages::Page::Login);
                    let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                    view_func_unauth(unauth_api)
                }
            }
        };
        view
    });

    view! {
        <Show when = move || api_signals.is_resolved.get()
                fallback = move || {
                    view! { <div style="color:white;">{"Loading..."}</div>}
                }
        >
            {move || view_signal.get()}
        </Show>
    }
}
