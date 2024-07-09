use leptos::*;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::{
        about_page::AboutPage, main_page::MainPage, new_entry_page::NewEntryPage,
        reports_page::ReportsPage, settings_page::SettingsPage,
    },
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Dialann"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
        }>
            <Routes>
                <Route path="/" view=MainPage/>
                <Route path="/settings" view=SettingsPage/>
                <Route path="/about" view=AboutPage/>
                <Route path="/reports" view=ReportsPage/>
                <Route path="/new_entry" view=|| view! { <NewEntryPage/> }/>
                <Route path="/new_entry/:eid" view=|| view! { <NewEntryPage/> }/>
            </Routes>
        </Router>
    }
}
