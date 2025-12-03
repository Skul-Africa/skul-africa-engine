use rusqlite::Connection;
use std::sync::Mutex;
use once_cell::sync::Lazy;


// Thread-safe, global DB connection
pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("data/engine.db").expect("failed to open db");
    conn.pragma_update(None, "foreign_keys", &"ON").unwrap();

    // Create tables if not exist
    conn.execute_batch(
        r#"
        BEGIN;
        CREATE TABLE IF NOT EXISTS curriculums (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            school_id TEXT NOT NULL,
            subject TEXT NOT NULL,
            class_level TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS topics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            curriculum_id INTEGER NOT NULL,
            topic_index INTEGER NOT NULL,
            topic_name TEXT NOT NULL,
            FOREIGN KEY(curriculum_id) REFERENCES curriculums(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS subtopics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            topic_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            FOREIGN KEY(topic_id) REFERENCES topics(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS objectives (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            topic_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            FOREIGN KEY(topic_id) REFERENCES topics(id) ON DELETE CASCADE
        );
        COMMIT;
        "#
    ).unwrap();

    Mutex::new(conn)
});
