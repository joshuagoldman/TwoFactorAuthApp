use leptos::{ev, *};

use crate::pages::Page;

#[component]
pub fn CredentialsForm(
    cx: Scope,
    action: Action<(String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (password, set_password) = create_signal(cx, String::new());
    let (username, set_username) = create_signal(cx, String::new());

    let dispatch_action =
        move || action.dispatch((username.get(), password.get()));

    let button_is_disabled = Signal::derive(cx, move || {
        disabled.get() || password.get().is_empty() || username.get().is_empty()
    });

    view! { cx,
      <div class="container">
        <div class="d-flex justify-content-center h-100">
          <div class="card">
            <div class="card-header">
              <h3>"Sign In"</h3>
              <div class="d-flex justify-content-end social_icon">
                <span><i class="fab fa-facebook-square"></i></span>
                <span><i class="fab fa-google-plus-square"></i></span>
                <span><i class="fab fa-twitter-square"></i></span>
              </div>
            </div>
            <div class="card-body">
              <form>
                <div class="input-group form-group">
                  <div class="input-group-prepend">
                    <span class="input-group-text"><i class="fas fa-user"></i></span>
                  </div>
                  <input type="text" 
                         class="form-control" 
                         placeholder="username"
                         prop:disabled = move || disabled.get()
                         on:keyup = move |ev: ev::KeyboardEvent| {
                           let val = event_target_value(&ev);
                           set_username.update(|v|*v = val);
                         }
                         // The `change` event fires when the browser fills the form automatically,
                         on:change = move |ev| {
                           let val = event_target_value(&ev);
                           set_username.update(|v|*v = val);
                         }/>
                  
                </div>
                <div class="input-group form-group">
                  <div class="input-group-prepend">
                    <span class="input-group-text"><i class="fas fa-key"></i></span>
                  </div>
                  <input type="password"
                         class="form-control"
                         placeholder="password"
                         prop:disabled = move || disabled.get()
                         on:keyup = move |ev: ev::KeyboardEvent| {
                           match &*ev.key() {
                               "Enter" => {
                                 dispatch_action();
                               }
                               _=> {
                                 let val = event_target_value(&ev);
                                 set_password.update(|p|*p = val);
                               }
                           }
                         }
                         // The `change` event fires when the browser fills the form automatically,
                         on:change = move |ev| {
                           let val = event_target_value(&ev);
                           set_password.update(|p|*p = val);
                    }/>
                </div>
                <div class="form-group">
                  <input type="submit" 
                         value="Login" 
                         class="btn float-right login_btn"
                         prop:disabled = move || button_is_disabled.get()
                         
                         on:click = move |_| dispatch_action()/>
                </div>
              </form>
            </div>
            <div class="card-footer">
              <div>
                {move || error.get().map(|err| view!{ cx,
                  <p style ="color:red;" >{ err }</p>
                })}
              </div>
              <div class="d-flex justify-content-center links">
                "Don't have an account?"<a href=Page::Register.path()>"Sign Up"</a>
              </div>
            </div>
          </div>
        </div>
      </div>
    }
}
