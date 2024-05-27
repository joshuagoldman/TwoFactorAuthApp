use leptos::{component, view, IntoView};

#[component]
pub fn PageNotFound() -> impl IntoView {
    view! {
        <div class="container">
            <div class="row d-flex justify-content-center align-items-center">
                <div class="blurry-card" style="height:100%">
                    <div class="row justify-content-center">
                        <div class="col-md-10 col-lg-6 col-xl-5 order-2 order-lg-1">
                            <p style="color:red;" class="text-center h3 fw-bold mb-2 mx-1 mx-md-4 mt-4">Page Not Found</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
