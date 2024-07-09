use chrono::{DateTime, TimeZone, Utc};
use leptos::*;
use leptos_router::A;
use serde::{Deserialize, Serialize};

use crate::models::{entry::Entry, mood::Mood};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiaryListItem {
    pub eid: Option<i64>,
    pub date: DateTime<Utc>,
    pub mood: Mood,
    pub content: String,
}

impl DiaryListItem {
    pub fn from_entry(entry: &Entry) -> Self {
        Self {
            eid: entry.eid,
            date: Utc.timestamp_opt(entry.date_created, 0).unwrap(),
            mood: Mood::from_int(entry.mood),
            content: entry.content.clone(),
        }
    }

    pub fn to_entry(&self) -> Entry {
        Entry {
            eid: self.eid,
            date_created: self.date.timestamp(),
            content: self.content.clone(),
            mood: self.mood.to_int(),
        }
    }

    pub fn date_string(&self) -> String {
        self.date.format("%a, %d %b %Y").to_string()
    }

    pub fn to_html(&self) -> impl IntoView {
        view! {
            <DiaryListItemHtml diary_list_item={(*self).clone()}/>
        }
    }
}

#[component]
fn DiaryListItemHtml(diary_list_item: DiaryListItem) -> impl IntoView {
    view! {
        <A class="card mx-auto my-2 w-100 d-block p-2 bg-body-secondary text-decoration-none" href={format!("/new_entry/{}", diary_list_item.eid.unwrap())} >
            <div class="card-title">
                <header style="text-align:left;">
                    <h5>{diary_list_item.date_string()}</h5>
                    <span style={format!("float:right;color:{}", diary_list_item.mood.colour_code())}>
                        <img src={diary_list_item.mood.icon_url()} style="width: 30px; height: 30px"/>
                        <div class="fs-4 d-inline">{diary_list_item.mood.name()}</div>
                    </span>
                </header>
                <div class="card-body">
                    <p>
                        {diary_list_item.content.clone()}
                    </p>
                </div>
            </div>
        </A>
    }
}
