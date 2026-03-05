use crate::db::task::{get_children_internal, root_task_internal};
use crate::db::{DbState, Task};
use rusqlite::{params, Result as SqliteResult};
use tauri::State;

#[tauri::command]
pub fn get_list_projects(db_state: State<DbState>) -> Result<Vec<String>, String> {
    let name = db_state
        .lock()
        .unwrap()
        .conn
        .prepare("SELECT name FROM projects")
        .map_err(|e| e.to_string())?
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<SqliteResult<Vec<String>>>()
        .map_err(|e| e.to_string())?;
    Ok(name)
}

#[tauri::command]
pub fn create_project(db_state: State<DbState>, name: &str) -> Result<bool, String> {
    let mut db_lock = db_state.lock().unwrap();
    let conn = &mut db_lock.conn;

    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Ok(false);
    }
    let name_link = format!("{}_link", trimmed);

    conn.execute("INSERT INTO projects (name) VALUES (?1)", params![trimmed])
        .map_err(|e| e.to_string())?;

    conn.execute_batch(&format!(
        "BEGIN;
		CREATE TABLE IF NOT EXISTS \"{trimmed}\" (id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                text TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS \"{name_link}\"(parent_id INTEGER NOT NULL,
                child_id INTEGER NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES \"{trimmed}\"(id),
                FOREIGN KEY (child_id) REFERENCES \"{trimmed}\"(id)
            );
            CREATE INDEX IF NOT EXISTS \"idx_{trimmed}\" ON \"{trimmed}\" (id);
            CREATE INDEX IF NOT EXISTS \"idx_{name_link}_parent\" ON \"{name_link}\" (parent_id);
            CREATE INDEX IF NOT EXISTS \"idx_{name_link}_child\" ON \"{name_link}\" (child_id);
		COMMIT;"
    ))
    .map_err(|e| e.to_string())?;

    conn.execute(
        &format!("INSERT INTO \"{trimmed}\" (name, text) VALUES (?1,?2)"),
        params![trimmed, ""],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

#[tauri::command]
pub fn delete_project(db_state: State<DbState>, name: &str) -> Result<bool, String> {
    let mut db_lock = db_state.lock().unwrap();
    let conn = &mut db_lock.conn;

    conn.execute("DELETE FROM projects WHERE name = ?1", params![name])
        .map_err(|e| e.to_string())?;

    let table_exists: bool = conn
        .query_row(
            "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)",
            params![name],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    if !table_exists {
        return Ok(false);
    }

    conn.execute(&format!("DROP TABLE \"{}_link\"", name), [])
        .map_err(|e| e.to_string())?;
    conn.execute(&format!("DROP TABLE \"{}\"", name), [])
        .map_err(|e| e.to_string())?;

    Ok(true)
}

#[tauri::command]
pub fn go_to_project(db_state: State<DbState>, name: &str) -> Result<(Task, Vec<Task>), String> {
    let mut db = db_state.lock().unwrap();
    db.name_project = name.to_owned();
    db.name_link = format!("{}_link", name);

    let task = root_task_internal(&db.conn, name)?;
    db.task = task.clone();

    let children = get_children_internal(&db.conn, name, task.id)?;

    Ok((task, children))
}
