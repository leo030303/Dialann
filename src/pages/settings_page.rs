use leptos::*;
use leptos_router::A;
use serde_wasm_bindgen::to_value;

use crate::{invoke, Dummy};

#[component]
pub fn SettingsPage() -> impl IntoView {
    let import_from_csv = move |_| {
        spawn_local(async move {
            invoke("import_from_csv", to_value(&Dummy::default()).unwrap()).await;
        })
    };
    let export_to_csv = move |_| {
        spawn_local(async move {
            invoke("export_to_csv", to_value(&Dummy::default()).unwrap()).await;
        })
    };
    view! {
        <div>
            <nav class="navbar bg-body-tertiary sticky-top">
                <div class="container-fluid">
                    <A href="/" class="btn btn-primary"><span><img src={"/public/back_icon.svg"} class="icon"/></span></A>
                    <div class="navbar-brand">{"Settings"}</div>
                </div>
            </nav>
            <div class="p-4">
                <h3 class="mx-auto">{"Manage Data"}</h3>
                <button on:click=import_from_csv class="btn btn-primary d-block mx-auto my-3">{"Import from CSV"}</button>
                <button on:click=export_to_csv class="btn btn-primary d-block mx-auto my-3">{"Export to CSV"}</button>
            </div>
        </div>
    }
}
