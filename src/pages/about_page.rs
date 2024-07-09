use leptos::*;
use leptos_router::A;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <nav class="navbar bg-body-tertiary sticky-top">
                <div class="container-fluid">
                    <A href="/" class="btn btn-primary"><span><img src={"/public/back_icon.svg"} class="icon"/></span></A>
                    <div class="navbar-brand">{"About"}</div>
                </div>
            </nav>
            <div class="p-4">
                <h3>{"Version:"}</h3>
                <p>{"1.0.0"}</p>
                <h3>{"Author:"}</h3>
                <p>{"Leo Ring"}</p>
                <h3>{"Github:"}</h3>
                <p>{"http://github.com/leo030303/Dialann"}</p>
                <h3>{"License:"}</h3>
                <p>{"GNU General Public License"}</p>
            </div>
        </div>
    }
}
