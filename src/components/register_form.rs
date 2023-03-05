use api_boundary::*;
use leptos::*;

#[component]
pub fn RegisterForm(
    cx: Scope,
    action: Action<NewUser, ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (password, set_password) = create_signal(cx, String::new());
    let (password_again, set_password_again) = create_signal(cx, String::new());
    let (username, set_username) = create_signal(cx, String::new());
    let (email, set_email) = create_signal(cx, String::new());
    let (full_name, set_full_name) = create_signal(cx, String::new());

    let dispatch_action =
        move || action.dispatch(
          NewUser {
            username: username.get(),
            password: password.get(),
            email: email.get(),
            full_name: full_name.get()
          });

    let button_is_disabled = Signal::derive(cx, move || {
        let all_one_empty = 
          disabled.get() || password.get().is_empty() || email.get().is_empty() ||
          username.get().is_empty() || full_name.get().is_empty() || password_again.get().is_empty();

        let passwords_are_equal =
          match (password.get().is_empty(),password_again.get().is_empty()) {
            (true,true) => false,
            (_,_) => {
              password.get() == password_again.get()
            }
          };

          all_one_empty || !passwords_are_equal
    });

    let on_change_closure = move |set_signal: WriteSignal<String>| {
      move |ev| {
        let val = event_target_value(&ev);
        set_signal.update(|v|*v = val);
      }
    } ;

    let on_key_up_closure = move |set_signal: WriteSignal<String> | {
      move |ev: ev::KeyboardEvent| {
        match (&*ev.key(),button_is_disabled.get()) {
            ("Enter",false) => {
              dispatch_action();
            }
            (_,_) => {
              let val = event_target_value(&ev);
              set_signal.update(|p|*p = val);
            }
        }
      }
    } ;

    view!{cx,
      <div class="container register-form">
            <div class="form">
                <div class="note">
                    <p>"Register New User"</p>
                </div>

                <div class="form-content register-border" style="background-color:rgba(0, 0, 0, 0.5);">
                    <div class="row">
                        <div class="col-md-6">
                            <div class="form-group">
                              <RegisterInput
                                on_change= on_change_closure(set_username)
                                on_key_up= on_key_up_closure(set_username)
                                disabled= disabled
                                place_holder = "User Name".to_string()
                                input_type = "text".to_string()
                              />
                            </div>
                            <div class="form-group">
                              <RegisterInput
                                on_change= on_change_closure(set_full_name)
                                on_key_up= on_key_up_closure(set_full_name)
                                disabled= disabled
                                place_holder = "Full Name".to_string()
                                input_type = "text".to_string()
                              />
                            </div>
                            <div class="form-group">
                              <RegisterInput
                                on_change= on_change_closure(set_email)
                                on_key_up= on_key_up_closure(set_email)
                                disabled= disabled
                                place_holder = "Email".to_string()
                                input_type = "text".to_string()
                              />
                            </div>
                        </div>
                        <div class="col-md-6">
                            <div class="form-group">
                              <RegisterInput
                                on_change= on_change_closure(set_password)
                                on_key_up= on_key_up_closure(set_password)
                                disabled= disabled
                                place_holder = "Password".to_string()
                                input_type = "password".to_string()
                              />
                            </div>
                            <div class="form-group">
                              <RegisterInput
                                on_change= on_change_closure(set_password_again)
                                on_key_up= on_key_up_closure(set_password_again)
                                disabled= disabled
                                place_holder = "Repeat Password".to_string()
                                input_type = "password".to_string()
                              />
                            </div>
                        </div>
                    </div>
                    {move || error.get().map(|err| view!{ cx,
                      <div class="d-flex justify-content-center mx-4 mb-3 mb-lg-4">
                        <p style ="color:white;" >{ err }</p>
                      </div>
                    })}
                    <button type="button"
                            class="btnSubmit"
                            prop:disabled = move || button_is_disabled.get()
                            prop:hidden = move || button_is_disabled.get()
                            on:click = move |_| dispatch_action()>"Submit"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn RegisterInput<F,G>(cx: Scope,
                    on_change: F,
                    on_key_up: G,
                    disabled: Signal<bool>,
                    place_holder: String,
                    input_type: String) -> impl IntoView
where 
    F: FnMut(web_sys::Event) + 'static + Clone,
    G: FnMut(ev::KeyboardEvent) + 'static + Clone,
{
  view! {cx,
    <input
      class="form-control"
      type = input_type
      required
      placeholder = place_holder
      prop:disabled = move || disabled.get()
      on:keyup = on_key_up
      // The `change` event fires when the browser fills the form automatically,
      on:change = on_change
    />
  }
}
