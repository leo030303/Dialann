use db::{Entry, EntryDao};
use handle_dates::{convert_text_to_timestamp, convert_timestamp_to_text};
use tauri::Manager;

mod db;
pub mod handle_dates;
pub mod import_export_handlers;

#[tauri::command(rename_all = "snake_case")]
fn convert_timestamp_to_date(timestamp: i64) -> String {
    convert_timestamp_to_text(timestamp)
}

#[tauri::command(rename_all = "snake_case")]
fn convert_date_to_timestamp(date_string: String) -> i64 {
    convert_text_to_timestamp(date_string)
}

#[tauri::command(rename_all = "snake_case")]
fn get_all(state: tauri::State<EntryDao>) -> Vec<Entry> {
    state.get_all()
}

#[tauri::command(rename_all = "snake_case")]
fn get_entry_by_id(state: tauri::State<EntryDao>, id: i64) -> Option<Entry> {
    state.get_entry_by_id(id)
}

#[tauri::command(rename_all = "snake_case")]
fn delete_entry(state: tauri::State<EntryDao>, entry: Entry) {
    state.delete_entry(entry);
}

#[tauri::command(rename_all = "snake_case")]
fn insert_entry(state: tauri::State<EntryDao>, entry: Entry) {
    state.insert_entry(entry);
}

#[tauri::command(rename_all = "snake_case")]
fn load_by_month_and_year(
    state: tauri::State<EntryDao>,
    start_month_in_secs: i64,
    end_month_in_secs: i64,
) -> Vec<Entry> {
    state.load_by_month_and_year(start_month_in_secs, end_month_in_secs)
}

#[tauri::command(rename_all = "snake_case")]
fn search_ascending(state: tauri::State<EntryDao>, search_string: String) -> Vec<Entry> {
    state.search_ascending(search_string)
}

#[tauri::command(rename_all = "snake_case")]
fn search_descending(state: tauri::State<EntryDao>, search_string: String) -> Vec<Entry> {
    state.search_descending(search_string)
}

#[tauri::command(rename_all = "snake_case")]
fn export_to_csv(app_handle: tauri::AppHandle) {
    import_export_handlers::export_to_csv(app_handle);
}

#[tauri::command(rename_all = "snake_case")]
fn import_from_csv(app_handle: tauri::AppHandle) {
    import_export_handlers::import_from_csv(app_handle);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let db_path = app
                .path()
                .app_data_dir()
                .unwrap()
                .join(String::from("dialann_entries.db"));
            app.manage(EntryDao::new(db_path));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_all,
            get_entry_by_id,
            delete_entry,
            insert_entry,
            load_by_month_and_year,
            search_ascending,
            search_descending,
            export_to_csv,
            import_from_csv,
            convert_date_to_timestamp,
            convert_timestamp_to_date
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
