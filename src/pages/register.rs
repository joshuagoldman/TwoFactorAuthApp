use leptos::*;
use leptos_router::*;

use api_boundary::*;

use crate::{
    api::{self, UnauthorizedApi},
    components::register_form::*,
    Page,
};

#[component]
pub fn Register(api: UnauthorizedApi) -> impl IntoView {
    let (register_response, set_register_response) = create_signal(None::<NewUserResponse>);
    let (register_error, set_register_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let register_action = create_action(move |new_user_ref: &NewUser| {
        let new_user = new_user_ref.clone();
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = api.register(&new_user).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_register_response.update(|v| *v = Some(res));
                    set_register_error.update(|e| *e = None);
                }
                Err(err) => {
                    let msg = match err {
                        api::Error::Fetch(js_err) => {
                            format!("{js_err:?}")
                        }
                        api::Error::Api(err) => err.message,
                    };
                    log::warn!(
                        "Unable to register new account for {}: {msg}",
                        &new_user.full_name
                    );
                    set_register_error.update(|e| *e = Some(msg));
                }
            }
        }
    });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
      <Show
        when = move || register_response.get().is_some()
        fallback = move || view!{
          <RegisterForm
            action = register_action
            error = register_error.into()
            disabled
          />
        }
      >
        <p style="color:white;">"You have successfully registered."</p>
        <p style="color:white;">"First, open your authentication app up and scan the bar code below:"</p>
        <BarCode
            new_user_info=register_response.get().unwrap()
        />
        <p style="color:white;">
         "When this has been done, then you may"
         <A href=Page::Login.path()>"login"</A>
         " with your new account."
        </p>
      </Show>
    }
}

#[component]
fn BarCode(new_user_info: NewUserResponse) -> impl IntoView {
    view! {
        <div>
            <img id="barcode"
                src={new_user_info.qr_code}
                alt=""
                title="Scan this with an authentication app"
                width="200"
                height="200"
            />
        </div>
    }
}
