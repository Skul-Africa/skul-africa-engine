// This module handles storing and retrieving curriculum data
// It uses the global DB connection defined in db.rs (thread-safe)

use anyhow::Result;
use rusqlite::params;
use crate::curriculum::models::{Curriculum, Topic};
use crate::database::db;
use rusqlite::Connection;

/// Insert a curriculum and all its topics/subtopics/objectives into the DB
/// Returns the ID of the newly created curriculum
pub fn insert_curriculum(curr: &Curriculum) -> Result<i64> {
    // Lock the global DB connection for mutable access
 // Lock the global DB connection for mutable access
let mut conn_guard = db::DB_CONN.lock().unwrap(); // <- mutable binding
let conn: &mut Connection = &mut *conn_guard;    // <- now this works


// Start a transaction
let tx = conn.transaction()?;


    // Insert into curriculums table
    tx.execute(
        "INSERT INTO curriculums (school_id, subject, class_level) VALUES (?1, ?2, ?3)",
        params![curr.school_id, curr.subject, curr.class_level],
    )?;
    let curriculum_id = tx.last_insert_rowid(); // Capture inserted curriculum ID

    // Insert topics, subtopics, objectives
    for (idx, t) in curr.topics.iter().enumerate() {
        tx.execute(
            "INSERT INTO topics (curriculum_id, topic_index, topic_name) VALUES (?1, ?2, ?3)",
            params![curriculum_id, idx as i64, t.name],
        )?;
        let topic_id = tx.last_insert_rowid();

        // Insert subtopics
        for sub in &t.subtopics {
            tx.execute(
                "INSERT INTO subtopics (topic_id, text) VALUES (?1, ?2)",
                params![topic_id, sub],
            )?;
        }

        // Insert objectives
        for obj in &t.objectives {
            tx.execute(
                "INSERT INTO objectives (topic_id, text) VALUES (?1, ?2)",
                params![topic_id, obj],
            )?;
        }
    }

    // Commit transaction
    tx.commit()?;

    Ok(curriculum_id)
}

/// Retrieve a topic (with subtopics and objectives) by name
pub fn get_topic_by_name(
    school_id: &str,
    class_level: &str,
    subject: &str,
    topic_name: &str
) -> Result<Option<Topic>> {
   let  conn = db::DB_CONN.lock().unwrap();
 // Lock connection

    // Query topic
    let mut stmt = conn.prepare(
        "SELECT t.id, t.topic_name FROM topics t
         JOIN curriculums c ON c.id = t.curriculum_id
         WHERE c.school_id = ?1 AND c.class_level = ?2 AND c.subject = ?3 AND t.topic_name = ?4
         LIMIT 1"
    )?;
    let mut rows = stmt.query(params![school_id, class_level, subject, topic_name])?;

    if let Some(row) = rows.next()? {
        let topic_id: i64 = row.get(0)?;
        let topic_name: String = row.get(1)?;

        // Load subtopics
        let mut sub_stmt = conn.prepare("SELECT text FROM subtopics WHERE topic_id = ?1 ORDER BY id")?;
        let sub_iter = sub_stmt.query_map([topic_id], |r| r.get(0))?;
        let mut subtopics = Vec::new();
        for s in sub_iter { subtopics.push(s?); }

        // Load objectives
        let mut obj_stmt = conn.prepare("SELECT text FROM objectives WHERE topic_id = ?1 ORDER BY id")?;
        let obj_iter = obj_stmt.query_map([topic_id], |r| r.get(0))?;
        let mut objectives = Vec::new();
        for o in obj_iter { objectives.push(o?); }

        let topic = Topic {
            name: topic_name,
            subtopics,
            objectives,
        };
        Ok(Some(topic))
    } else {
        Ok(None) // Topic not found
    }
}
