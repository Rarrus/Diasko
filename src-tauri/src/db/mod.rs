pub mod draft;
pub mod project;
pub mod task;

use std::path::PathBuf;
use std::sync::Mutex;
pub use task::Task;

use rusqlite::{Connection, Result};

pub type DbState = Mutex<DB>;

pub struct DB {
    pub name_project: String,
    pub name_link: String,
    pub task: Task,
    pub conn: Connection,
}

const INIT_SQL: &str = "
    CREATE TABLE IF NOT EXISTS projects (
        id   INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL Unique
    );
    CREATE TABLE IF NOT EXISTS help (
        id  INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        text TEXT NOT NULL
    );
    INSERT OR IGNORE INTO help (name, text) VALUES ('Draft', '');
";

impl DB {
    pub fn new(path: PathBuf) -> DB {
        let conn = Connection::open(path.join("projects.db")).expect("Failed to open a database");
        DB::from_conn(conn)
    }

    #[cfg(test)]
    pub fn new_test() -> DB {
        let conn = Connection::open_in_memory().expect("Failed to open an in-memory database");
        DB::from_conn(conn)
    }

    fn from_conn(conn: Connection) -> DB {
        let name_project = "".to_owned();
        let name_link = format!("{}_link", name_project);
        let task: Task = Task::new();
        let db = DB {
            name_project,
            task,
            name_link,
            conn,
        };
        db.initialize().expect("Failed to initialize a database");
        db
    }

    fn initialize(&self) -> Result<()> {
        self.conn.execute_batch(INIT_SQL)?;
        Ok(())
    }
}
