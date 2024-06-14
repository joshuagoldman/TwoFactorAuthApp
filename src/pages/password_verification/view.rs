use leptos::{
    component, create_action, create_rw_signal, leptos_dom::logging::console_log, view, IntoView,
    RwSignal, Show, Signal, SignalGet, SignalUpdate,
};

use crate::{
    api::{api_boundary::ResultHandler, authorized_api::AuthorizedApi},
    components::{
        fields_error::TextFieldErrors, password_verification_form::PasswordVerificationForm,
    },
    pages::password_verification::{
        misc::{
            get_action_to_perform, get_action_to_perform_title, get_is_action_enabled_signal,
            get_page_title, PassVerificationActionData,
        },
        pass_verification_field_signals::get_password_verification_form_signals,
    },
};

use super::misc::PassVerificationAction;

#[component]
pub fn PasswordVerification(
    authorized_api: AuthorizedApi,
    action_type: PassVerificationAction,
) -> impl IntoView {
    let result: RwSignal<Option<ResultHandler<String>>> = create_rw_signal(None);
    let is_verification_mode = create_rw_signal(true);
    let is_loading = create_rw_signal(false);
    let page_title = get_page_title(action_type.clone());
    let current_password_signal = create_rw_signal(String::new());
    let new_password_signal = create_rw_signal(String::new());
    let new_password_repeat_signal = create_rw_signal(String::new());
    let action_name = get_action_to_perform_title(is_verification_mode, action_type.clone());
    let (_, error_fields_signal, pass_verification_map_signal) =
        get_password_verification_form_signals(
            current_password_signal,
            new_password_signal,
            new_password_repeat_signal,
            action_type.clone(),
            is_verification_mode,
        );

    let pass_verification_data = PassVerificationActionData {
        current_password_signal,
        new_password_signal,
        authorized_api,
        is_enter_current_passwrd: is_verification_mode,
        new_password_repeat_signal,
        result,
        is_loading,
        action_type: action_type.clone(),
        pass_verification_map_signal,
    };
    let action_to_perform =
        get_action_to_perform(pass_verification_data.clone(), action_type.clone());

    let any_error = Signal::derive(move || error_fields_signal.get().len() > 0);

    let passords_are_equal =
        Signal::derive(move || new_password_signal.get() == new_password_repeat_signal.get());

    let action_enabled = get_is_action_enabled_signal(
        action_type,
        any_error,
        is_verification_mode,
        passords_are_equal,
    );

    let is_success = Signal::derive(move || match result.get() {
        Some(ResultHandler::OkResult(_)) => true,
        _ => false,
    });

    let additional_form_action = create_action(move |_| async move {
        result.update(|x| *x = None);
    });

    view! {
        <div class="container">
            <div class="row d-flex justify-content-center align-items-center">
                <div class="blurry-card" style="height:100%">
                    <div class="row justify-content-center">
                        <div class="col-md-10 col-lg-6 col-xl-5 order-2 order-lg-1">

                            <Show
                                when= {move || !is_loading.get()}
                                fallback= move || view! { <div></div>}
                            >
                                <p class="text-center h3 fw-bold mb-2 mx-1 mx-md-4 mt-4">{page_title.clone()}</p>
                            </Show>

                            <form class="mx-1 mx-md-4">
                                <Show
                                    when= move  || !is_loading.get()
                                    fallback= move || view! {
                                        <div>{move || format!("Performing {}", action_name.get())}</div>
                                    }
                                >
                                   <PasswordVerificationForm
                                        pass_verification_data = pass_verification_data.clone()
                                        action_enabled
                                        action_name
                                        is_success
                                        action_to_perform
                                        additional_form_action
                                   ></PasswordVerificationForm>
                                </Show>
                                <TextFieldErrors
                                    error_fields_signal
                                >
                                </TextFieldErrors>
                                <Show
                                    when= move  || result.get().is_some()
                                    fallback= move || view! { <div></div>}
                                >
                                <ActionResult
                                    result=result.get()
                                    current_password_signal
                                ></ActionResult>
                                </Show>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ActionResult(
    result: Option<ResultHandler<String>>,
    current_password_signal: RwSignal<String>,
) -> impl IntoView {
    view! {
        {move || {
            let login_result = result.clone();
            console_log("came here");
            if current_password_signal.get().is_empty() {

                match login_result.unwrap_or(ResultHandler::OkResult(String::new())) {
                        ResultHandler::OkResult(res_msg) => {

                            view! {
                                <div style="color:green">{res_msg}</div>
                            }
                        }
                        ResultHandler::ErrResult(err_msg) => {
                            view! {
                                <div style="color:red">{err_msg}</div>
                            }
                        }
                }
            }
            else {
               view! {
                   <div></div>
               }
            }
        }}
    }
}
