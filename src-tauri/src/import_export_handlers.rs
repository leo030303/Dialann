use crate::db::EntryDao;
use crate::Entry;
use chrono::Local;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

pub fn export_to_csv(app: tauri::AppHandle) {
    let list_of_entries = app.state::<EntryDao>().get_all();
    let current_date = Local::now().date_naive();
    let initial_file_name = format!("{}_diary_entries.csv", current_date.format("%d-%m-%Y"));
    println!("You pressed the export button");
    // app.dialog()
    //     .file()
    //     .set_file_name(initial_file_name)
    //     .add_filter("My Filter", &["csv"])
    //     .save_file(|file_path| {
    //         if let Some(file_path) = file_path {
    //             if let Ok(mut wtr) = csv::Writer::from_path(file_path) {
    //                 list_of_entries
    //                     .into_iter()
    //                     .for_each(|entry| wtr.serialize(entry).expect("error writing entry"));
    //                 wtr.flush().expect("error writing csv");
    //             }
    //         };
    //     });
}

pub fn import_from_csv(app: tauri::AppHandle) {
    app.dialog().file().pick_file(move |file_path| {
        if let Some(file_path) = file_path {
            println!("This is the path: {0}", file_path.path.display());
            if let Ok(mut rdr) = csv::Reader::from_path(file_path.path) {
                for row in rdr.deserialize() {
                    if let Ok(entry) = row {
                        let mut entry: Entry = entry;
                        entry.eid = None;
                        app.state::<EntryDao>().insert_entry_no_id(entry);
                    } else {
                        println!("{:?}", row);
                    };
                }
            };
        };
    });
}
