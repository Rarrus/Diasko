pub mod db;

use db::draft::{get_draft, save_draft};
use db::project::{create_project, delete_project, get_list_projects, go_to_project};
use db::task::{
    create_task, delete_task, get_all_task, get_children, get_parent, get_task, go_to_task,
    root_task, save_text_task,
};
use db::DB;
use std::sync::Mutex;
use tauri::{Builder, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let data_dir = app_handle.path().app_config_dir().expect("Failed to get app data dir");
            app.manage(Mutex::new(DB::new(data_dir)));
            let window = app.get_webview_window("main").unwrap();
            window.maximize().unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_project,
            create_task,
            get_list_projects,
            get_parent,
            get_children,
            get_draft,
            get_all_task,
            get_task,
            go_to_task,
            go_to_project,
            delete_project,
            delete_task,
            save_text_task,
            save_draft,
			root_task
        ])
         
        .run(tauri::generate_context!())
        
        .expect("error while running tauri application");
}
