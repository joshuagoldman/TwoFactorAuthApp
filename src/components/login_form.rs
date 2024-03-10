use crate::api::api_boundary;
use leptos::{ev, *};

#[component]
pub fn LoginForm(
    login_action: Action<api_boundary::Credentials, ()>,
    login_error: ReadSignal<Option<String>>,
) -> impl IntoView {
    let (password, set_password) = create_signal(String::new());
    let (username, set_username) = create_signal(String::new());
    let login_allowed = Signal::derive(move || {
        !String::is_empty(&password.get()) && !String::is_empty(&username.get())
    });

    let dispatch_action = move || {
        login_action.dispatch(api_boundary::Credentials {
            password: password.get(),
            username: username.get(),
        })
    };

    view! {
            <div class="row d-flex justify-content-center align-items-center h-10">
                <div class="col-12 col-md-8 col-lg-6 col-xl-5">
                    <div class="blurry-card" style="border-radius: 1rem;">
                        <div class="card-body p-5 text-center">
                            <div class="mb-md-5 mt-md-4 pb-5">

                                <h2 class="fw-bold text-white mb-2 text-uppercase">{"Login"}</h2>
                                <p class="text-white mb-5">{"Please enter your login and password!"}</p>

                                <div class="form-outline text-white form-white mb-4">
                                    <input type="username"
                                            id="typeEmailX"
                                            class="form-control form-control-lg"
                                            on:keyup = move |ev: ev::KeyboardEvent| {
                                                match &*ev.key() {
                                                    "enter" => {
                                                        if login_allowed.get() {
                                                            dispatch_action();
                                                        }
                                                    }
                                                    _=> {
                                                        let val = event_target_value(&ev);
                                                        set_password.update(|p|*p = val);
                                                    }
                                                }
                                            } />
                                    <label class="form-label" for="typeEmailX">{"Username"}</label>
                                </div>

                                <div class="form-outline text-white form-white mb-4">
                                    <input type="password"
                                            id="typePasswordX"
                                            class="form-control form-control-lg"
                                            on:keyup = move |ev: ev::KeyboardEvent| {
                                                match &*ev.key() {
                                                    "enter" => {
                                                        if login_allowed.get() {
                                                            dispatch_action();
                                                        }
                                                    }
                                                    _=> {
                                                        let val = event_target_value(&ev);
                                                        set_username.update(|p|*p = val);
                                                    }
                                                }
                                            } />
                                    <label class="form-label" for="typePasswordX">{"Password"}</label>
                                </div>

                                <p class="small mb-5 pb-lg-2"><a class="text-white" href="/reset">{"Forgot password?"}</a></p>

                                <button class="btn btn-outline-light btn-lg px-5"
                                        type="submit"
                                        disabled= {move || login_allowed.get()}
                                        on:click = move |_| {
                                            dispatch_action();
                                        }>"Login"</button>

                                <div>
                                    <p class="mt-5 pb-lg-2 text-white">"Don't have an account? " <a href="/register" class="text-white-50 fw-bold">{"Sign Up"}</a>
                                    </p>
                                </div>
                                <div>
                                    <p class="mt-5 pb-lg-2 text-white"
                                       style={move || {
                                           if login_error.get().is_some() {
                                              "color:red;display:none"
                                            }
                                            else {
                                              "color:red"
                                            }
                                        }}>{move || login_error.get()}<a href="/register" class="text-white-50 fw-bold">{"Sign Up"}</a>
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
    }
}
