// This module receives curriculum data in JSON format,
// validates it, and stores it in the database using curriculum_store.rs.

use anyhow::Result; // For error handling
use serde_json::Value; // To work with JSON payloads
use crate::curriculum::models::Curriculum; // Our Rust struct representing a curriculum
use crate::database::curriculum_store; // Functions to insert/retrieve curriculum

/// Receives a JSON payload (serde_json::Value), validates it,
/// converts it to a Curriculum struct, and inserts it into the DB.
/// Returns a JSON response indicating success and curriculum ID.
pub fn receive_curriculum(payload: Value) -> Result<serde_json::Value> {
    // Deserialize the JSON payload into our Rust Curriculum struct
    let curr: Curriculum = serde_json::from_value(payload)?;

    // Minimal validation: ensure the curriculum has at least one topic
    if curr.topics.is_empty() {
        return Err(anyhow::anyhow!("curriculum must contain at least one topic"));
    }

    // Optional: You could add more validations here, e.g.,
    // - Ensure each topic has a name
    // - Ensure each topic has at least one objective
    // - Check for duplicate topic names

    // Insert the curriculum into the database
    let id = curriculum_store::insert_curriculum(&curr)?;

    // Return a JSON response with the new curriculum ID
    Ok(serde_json::json!({
        "message": "curriculum_loaded",
        "curriculum_id": id
    }))
}
