use chrono::{Datelike, Months, TimeZone, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;

use crate::{components::diary_list_item::DiaryListItem, invoke, models::entry::Entry};

#[derive(Serialize, Deserialize)]
struct LoadByMonthAndYearArgs {
    start_month_in_secs: i64,
    end_month_in_secs: i64,
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
    let (current_month, _set_current_month) = create_signal(current_month_date);
    let (entries_list, set_entries_list): (ReadSignal<Vec<Entry>>, WriteSignal<Vec<Entry>>) =
        create_signal(Vec::new());
    spawn_local(async move {
        let entries_from_db: Vec<Entry> = serde_wasm_bindgen::from_value(
            invoke(
                "load_by_month_and_year",
                to_value(&LoadByMonthAndYearArgs {
                    start_month_in_secs: current_month.get().timestamp(),
                    end_month_in_secs: current_month
                        .get()
                        .to_owned()
                        .checked_add_months(Months::new(1))
                        .unwrap()
                        .timestamp(),
                })
                .unwrap(),
            )
            .await,
        )
        .expect("malformed json returned");
        set_entries_list.set(entries_from_db);
    });

    view! {
        <div>
            <div class="bg-body-secondary sticky-top">
                <h1 class="d-inline">{"Dialann"}</h1>
                <div class="float-end bg-body-secondary ">
                    <a href="/settings" class="btn btn-primary mx-1"><span><img src={"/public/settings_icon.svg"} class="icon"/></span></a>
                    <a href="/reports" class="btn btn-primary mx-1"><span><img src={"/public/reports_icon.svg"} class="icon"/></span></a>
                    <a href="/about" class="btn btn-primary mx-1"><span><img src={"/public/about_icon.svg"} class="icon"/></span></a>
                </div>
            </div>
            <div style="padding-top: 50px; padding-bottom: 50px; width: 80%; display:block; margin:auto;">
                {move || entries_list.get().iter().map(|entry| DiaryListItem::from_entry(entry).to_html()).collect::<Vec<_>>()}
            </div>
            <a href="/new_entry" style="position:fixed; bottom:0; width: 100%; display: block; text-align: center;" class="btn btn-primary">{"New Entry"}</a>
        </div>
    }
}
