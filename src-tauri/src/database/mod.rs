use once_cell::sync::Lazy;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use std::fs;
use std::path::Path;

mod migrations;

static DB_FILE_PATH: Lazy<String> = Lazy::new(|| get_db_path());

static CONNECTION_POOL: Lazy<Pool<SqliteConnectionManager>> = Lazy::new(|| {
    let manager = SqliteConnectionManager::file(DB_FILE_PATH.as_str());
    Pool::new(manager).expect("Failed to create pool.")
});

pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
    run_migrations().expect("Failed to run migrations.");
}

fn establish_connection() -> PooledConnection<SqliteConnectionManager> {
    CONNECTION_POOL.get().expect("Failed to get a connection.")
}

fn run_migrations() -> Result<()> {
    let connection = establish_connection();
    // Manually execute migration SQL statements
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

fn create_db_file() {
    let db_path = DB_FILE_PATH.clone();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    Path::new(&DB_FILE_PATH.as_str()).exists()
}

fn get_db_path() -> String {
    let config_dir = dirs::home_dir().unwrap();
    config_dir
        .join("draft-storage/db.sqlite3")
        .to_str()
        .unwrap()
        .to_owned()
}
