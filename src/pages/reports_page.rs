use leptos::*;
use leptos_router::A;

#[component]
pub fn ReportsPage() -> impl IntoView {
    view! {
        <div>
            <nav class="navbar bg-body-tertiary sticky-top">
                <div class="container-fluid">
                    <A href="/" class="btn btn-primary"><span><img src={"/public/back_icon.svg"} class="icon"/></span></A>
                    <div class="navbar-brand">{"Reports"}</div>
                </div>
            </nav>
            <h3>{"Reports"}</h3>
        </div>
    }
}
