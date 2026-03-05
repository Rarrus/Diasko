use crate::db::DbState;
use rusqlite::{params, Result};
use tauri::State;


#[tauri::command]
pub fn get_draft(db: State<DbState>) -> Result<String, String> {
	db.lock()
		.unwrap()
		.conn
		.query_row(
			"SELECT text FROM help WHERE name = ?1 LIMIT 1",
			params!["Draft"],
			|row| row.get(0),
		)
		.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_draft(db: State<DbState>, text: &str) -> Result<usize, String> {
	db.lock().unwrap().conn
		.prepare_cached(
			"UPDATE help SET text = ?1 WHERE name = ?2"
		)
		.unwrap()
		.execute(params![text, "Draft"])
		.map_err(|e| e.to_string())
}
