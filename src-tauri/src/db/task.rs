use crate::db::DbState;
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub text: String,
}

impl Task {
    pub fn new() -> Task {
        Task {
            id: 0,
            name: String::new(),
            text: String::new(),
        }
    }

    pub fn from_row(row: &rusqlite::Row) -> SqliteResult<Self> {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            text: row.get(2)?,
        })
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new()
    }
}

// Internal functions for reuse and to avoid deadlocks
pub(crate) fn get_children_internal(
    conn: &Connection,
    project_name: &str,
    id: i32,
) -> Result<Vec<Task>, String> {
    let link_table = format!("{}_link", project_name);
    let mut stmt = conn
        .prepare_cached(&format!(
            "SELECT * FROM \"{}\" project JOIN \"{}\" link ON project.id = link.child_id \
		WHERE link.parent_id = ?1",
            project_name, link_table
        ))
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map(params![id], Task::from_row)
        .map_err(|e| e.to_string())?
        .collect::<SqliteResult<Vec<Task>>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}
fn get_task_internal(conn: &Connection, project_name: &str, id: i32) -> Result<Task, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT id, name, text FROM \"{project_name}\" WHERE ID = ?1"
        ))
        .map_err(|e| e.to_string())?;
    let task = stmt
        .query_row(params![id], Task::from_row)
        .map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub fn get_children(db: State<DbState>, id: i32) -> Result<Vec<Task>, String> {
    let db_lock = &db.lock().unwrap();
    let conn = &db_lock.conn;
    let project_name = &db_lock.name_project;
    get_children_internal(conn, project_name, id)
}

#[tauri::command]
pub fn get_parent(db: State<DbState>, project_name: &str, id: i32) -> Result<Task, String> {
    let db = &db.lock().unwrap();
    let task = db
        .conn
        .query_row(
            &format!(
                "SELECT * FROM \"{project_name}\" t
		JOIN ?1 l ON t.id = l.parent_id WHERE l.child_id = ?2"
            ),
            params![db.name_link, id],
            Task::from_row,
        )
        .map_err(|e| e.to_string())?;

    Ok(task)
}

pub fn root_task_internal(conn: &Connection, name_project: &str) -> Result<Task, String> {
    let task = conn
        .query_row(
            &format!("SELECT id, name, text FROM \"{name_project}\" WHERE id = 1"),
            [],
            Task::from_row,
        )
        .map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub fn root_task(db: State<DbState>, name: &str) -> Result<Task, String> {
    let conn = &db.lock().unwrap().conn;
    root_task_internal(conn, name)
}

#[tauri::command]
pub fn save_text_task(db: State<DbState>, text: &str) -> Result<usize, String> {
    let db = db.lock().unwrap();
    db.conn
        .execute(
            &format!("UPDATE \"{}\" SET text = ?1 WHERE id = ?2", db.name_project),
            params![text, db.task.id],
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_task(db: State<DbState>, name: &str) -> Result<i64, String> {
    let db = db.lock().unwrap();
    let conn = &db.conn;
    let parent_id = db.task.id;

    conn.execute(
        &format!(
            "INSERT INTO \"{}\" (name, text)VALUES (?1, ?2)",
            db.name_project
        ),
        params![name, ""],
    )
    .map_err(|e| e.to_string())?;

    let last_id = conn.last_insert_rowid();

    conn.execute(
        &format!(
            "INSERT INTO \"{}\" (parent_id, child_id) VALUES (?1, ?2)",
            db.name_link
        ),
        params![parent_id, last_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(last_id)
}

#[tauri::command]
pub fn delete_task(db: State<DbState>, id: i32) -> Result<(), String> {
    let db = db.lock().unwrap();
    let conn = &db.conn;

    conn.execute(
        &format!("DELETE FROM \"{}\" WHERE id = ?1", db.name_project),
        params![id],
    )
    .map_err(|e| e.to_string())?;
    conn.execute(
        &format!(
            "DELETE FROM \"{}\" WHERE parent_id = ?2 OR child_id = ?2",
            db.name_project
        ),
        params![id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_all_task(db: State<DbState>, project_name: &str) -> Result<Vec<Task>, String> {
    let tasks = db
        .lock()
        .unwrap()
        .conn
        .prepare_cached(&format!("SELECT id, name, text FROM \"{project_name}\""))
        .map_err(|e| e.to_string())?
        .query_map([], Task::from_row)
        .map_err(|e| e.to_string())?
        .collect::<SqliteResult<Vec<Task>>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}

#[tauri::command]
pub fn get_task(db: State<DbState>, project_name: &str, id: i32) -> Result<Task, String> {
    let db_lock = db.lock().unwrap();
    get_task_internal(&db_lock.conn, project_name, id)
}

#[tauri::command]
pub fn go_to_task(db_state: State<DbState>, id: i32) -> Result<(Task, Vec<Task>), String> {
    let mut db = db_state.lock().unwrap();
    let project_name = db.name_project.clone();

    let task = get_task_internal(&db.conn, &project_name, id)?;
    let children = get_children_internal(&db.conn, &project_name, id)?;

    db.task = task.clone();

    Ok((task, children))
}
