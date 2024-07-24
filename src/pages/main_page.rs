use chrono::{Datelike, Months, TimeZone, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::{
    components::diary_list_item::DiaryListItem,
    invoke,
    models::{entry::Entry, mood::Mood},
};

#[derive(Serialize, Deserialize)]
struct LoadByMonthAndYearArgs {
    start_month_in_secs: i64,
    end_month_in_secs: i64,
}

#[component]
pub fn MoodBar(mood_list: Vec<Mood>) -> impl IntoView {
    view! {
        <div>
        </div>
    }
}

#[component]
pub fn MoodChart(entry_list: Vec<Entry>) -> impl IntoView {
    view! {
        <div>
        </div>
    }
}

#[component]
pub fn MainPage() -> impl IntoView {
    let now = Utc::now();

    // Get the year, month, and set day to 1
    let year = now.year();
    let month = now.month();
    let day = 1;
    let hour = 0;
    let minute = 0;

    // Create a DateTime object for the first minute of the current month
    let current_month_date = Utc
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .unwrap();
    let (current_month, set_current_month) = create_signal(current_month_date);
    let entries_list = create_resource_with_initial_value(
        move || current_month.get(),
        |current_month_value| async move {
            serde_wasm_bindgen::from_value::<Vec<Entry>>(
                invoke(
                    "load_by_month_and_year",
                    to_value(&LoadByMonthAndYearArgs {
                        start_month_in_secs: current_month_value.timestamp(),
                        end_month_in_secs: current_month_value
                            .to_owned()
                            .checked_add_months(Months::new(1))
                            .unwrap()
                            .timestamp(),
                    })
                    .unwrap(),
                )
                .await,
            )
            .expect("malformed json returned")
        },
        Some(vec![]),
    );
    view! {
        <div>
            <div class="bg-body-secondary sticky-top">
                <div class="d-block">
                    <h1 class="d-inline">{"Dialann"}</h1>
                    <div class="float-end bg-body-secondary d-inline">
                        <a href="/settings" class="btn btn-primary mx-1"><span><img src={"/public/settings_icon.svg"} class="icon"/></span></a>
                        <a href="/reports" class="btn btn-primary mx-1"><span><img src={"/public/reports_icon.svg"} class="icon"/></span></a>
                        <a href="/about" class="btn btn-primary mx-1"><span><img src={"/public/about_icon.svg"} class="icon"/></span></a>
                    </div>
                </div>
                <div class="d-block mx-auto text-center">
                    <button
                        on:click=move |_| {
                            set_current_month.set(current_month.get().checked_sub_months(Months::new(1)).unwrap())
                        }
                      type="button" class="btn btn-primary m-1 d-inline align-middle">{"<"}</button>
                    <p class="form-control w-auto m-1 d-inline-block align-middle">{move || current_month.get().format("%b-%Y").to_string()}</p>
                    <button
                        on:click=move |_| {
                            set_current_month.set(current_month.get().checked_add_months(Months::new(1)).unwrap())
                        }
                      type="button" class="btn btn-primary m-1 d-inline align-middle">{">"}</button>
                </div>
            </div>
            <div style="padding-top: 50px; padding-bottom: 50px; width: 80%; display:block; margin:auto;">
                {move || entries_list.get().unwrap().iter().map(|entry| DiaryListItem::from_entry(entry).to_html()).collect::<Vec<_>>()}
            </div>
            <a href="/new_entry" style="position:fixed; bottom:0; width: 100%; display: block; text-align: center;" class="btn btn-primary">{"New Entry"}</a>
        </div>
    }
}
