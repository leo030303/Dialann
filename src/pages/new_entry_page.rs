use crate::{
    invoke,
    models::{entry::Entry, mood::Mood},
};
use gloo_console::log;
use leptos::*;
use leptos_router::{use_params_map, A};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use wasmtimer::std::SystemTime;
use wasmtimer::std::UNIX_EPOCH;

#[derive(Serialize, Deserialize)]
struct SendDate {
    date_string: String,
}

#[derive(Serialize, Deserialize)]
struct GetEntryByIdArgs {
    id: i64,
}

#[derive(Serialize, Deserialize)]
struct InsertEntryArgs {
    entry: Entry,
}

#[component]
pub fn NewEntryPage() -> impl IntoView {
    let params = use_params_map();
    let eid = move || params.with(|params| params.get("eid").cloned());
    log!(format!("EID value: {:?}", eid()));
    let eid_option = eid().map(|eid_string| eid_string.parse::<i64>().unwrap());
    let (selected_date, set_selected_date) = create_signal(
        if let Ok(time_obj) = SystemTime::now().duration_since(UNIX_EPOCH) {
            time_obj.as_secs() as i64
        } else {
            0
        },
    );
    let (selected_mood, set_selected_mood) = create_signal(Mood::Okay);
    let (selected_content, set_selected_content) = create_signal(String::new());
    spawn_local(async move {
        if let Some(eid_parsed) = eid_option {
            let entry_from_db: Option<Entry> = serde_wasm_bindgen::from_value(
                invoke(
                    "get_entry_by_id",
                    to_value(&GetEntryByIdArgs { id: eid_parsed }).unwrap(),
                )
                .await,
            )
            .expect("malformed json returned");
            if let Some(entry) = entry_from_db {
                log!(format!("{:?}", entry));
                set_selected_content.set(entry.content);
                set_selected_mood.set(Mood::from_int(entry.mood));
                set_selected_date.set(entry.date_created);
            }
        };
    });
    let content_input_ref: NodeRef<html::Textarea> = create_node_ref();
    let date_input_ref: NodeRef<html::Input> = create_node_ref();
    let awesome_input_ref: NodeRef<html::Input> = create_node_ref();
    let good_input_ref: NodeRef<html::Input> = create_node_ref();
    let okay_input_ref: NodeRef<html::Input> = create_node_ref();
    let bad_input_ref: NodeRef<html::Input> = create_node_ref();
    let awful_input_ref: NodeRef<html::Input> = create_node_ref();

    let create_new_entry = move |_| {
        if awesome_input_ref
            .get()
            .map_or(false, |input_ref| input_ref.checked())
        {
            set_selected_mood.set(Mood::Awesome);
        } else if good_input_ref
            .get()
            .map_or(false, |input_ref| input_ref.checked())
        {
            set_selected_mood.set(Mood::Good);
        } else if okay_input_ref
            .get()
            .map_or(false, |input_ref| input_ref.checked())
        {
            set_selected_mood.set(Mood::Okay);
        } else if bad_input_ref
            .get()
            .map_or(false, |input_ref| input_ref.checked())
        {
            set_selected_mood.set(Mood::Bad);
        } else if awful_input_ref
            .get()
            .map_or(false, |input_ref| input_ref.checked())
        {
            set_selected_mood.set(Mood::Awful);
        };
        let new_date = date_input_ref
            .get()
            .map_or(String::new(), |input_ref| input_ref.value());
        let new_content = content_input_ref
            .get()
            .map_or(String::new(), |input_ref| input_ref.value());
        if !new_content.is_empty() {
            set_selected_content.set(new_content);
        };
        let mut current_selected_date = selected_date.get();
        spawn_local(async move {
            if !new_date.is_empty() {
                let timestamp: i64 = serde_wasm_bindgen::from_value(
                    invoke(
                        "convert_date_to_timestamp",
                        to_value(&SendDate {
                            date_string: new_date,
                        })
                        .unwrap(),
                    )
                    .await,
                )
                .unwrap();
                log!(format!("async timestamp: {}", timestamp));
                current_selected_date = timestamp;
            };
            log!(format!("other timestamp: {}", selected_date.get()));
            let entry = Entry {
                eid: eid_option,
                date_created: current_selected_date,
                content: selected_content.get(),
                mood: selected_mood.get().to_int(),
            };
            invoke(
                "insert_entry",
                to_value(&InsertEntryArgs { entry }).unwrap(),
            )
            .await;
        });
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    };

    view! {
        <div>
            <nav class="navbar bg-body-tertiary sticky-top">
                <div class="container-fluid">
                    <A href="/" class="btn btn-primary"><span><img src={"/public/back_icon.svg"} class="icon"/></span></A>
                    <div class="navbar-brand">{"Entry"}</div>
                </div>
            </nav>
            <div class="p-4">
                <form>
                    <div style="margin: auto; width: 175px;">
                        <label class="mood_button_box">
                            <input name="mood_radio" value="awesome" type="radio" node_ref={awesome_input_ref}/>
                            <span class="mood_button awesome"></span>
                        </label>
                        <label class="mood_button_box">
                            <input name="mood_radio" value="good" type="radio" node_ref={good_input_ref}/>
                            <span class="mood_button good"></span>
                        </label>
                        <label class="mood_button_box">
                            <input name="mood_radio" value="okay" type="radio" node_ref={okay_input_ref}/>
                            <span class="mood_button okay"></span>
                        </label>
                        <label class="mood_button_box">
                            <input name="mood_radio" value="bad" type="radio" node_ref={bad_input_ref}/>
                            <span class="mood_button bad"></span>
                        </label>
                        <label class="mood_button_box">
                            <input name="mood_radio" value="awful" type="radio" node_ref={awful_input_ref}/>
                            <span class="mood_button awful"></span>
                        </label>
                    </div>
                    <input
                        type="date"
                        class="form-control m-3"
                        node_ref={date_input_ref}
                    />
                    <textarea class="form-control m-3" value=selected_content placeholder="What's up?" rows="4" cols="50" node_ref={content_input_ref}>
                    </textarea>
                    <button type="button" on:click=create_new_entry class="btn btn-primary" style="display:block; width: 100%;" >{"Save"}</button>
                </form>
            </div>
        </div>
    }
}
