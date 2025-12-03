// This module handles incoming commands to the engine
// It parses JSON received from stdin and forwards it
// to the correct engine function, e.g., curriculum loader.

use anyhow::Result;       // For error handling
use serde_json::Value;    // For working with JSON values
use crate::curriculum::receiver; // Import our curriculum receiver module

/// This function takes a JSON string as input, determines
/// which action is requested, and calls the appropriate handler.
/// It returns a JSON object (serde_json::Value) as response.
pub fn handle_command(raw: &str) -> Result<Value> {
    // Parse the raw string into a serde_json Value
    let v: Value = serde_json::from_str(raw)?;
    
    // Extract the "action" field from JSON
    // This tells the engine what operation to perform
    let action = v.get("action")
        .and_then(|a| a.as_str()) // Convert JSON value to Rust &str
        .ok_or_else(|| anyhow::anyhow!("missing action field"))?; // Error if missing

    // Match the action string to decide what to do
    match action {
        "load_curriculum" => {
            // For curriculum loading, expect a "payload" field
            let payload = v.get("payload")
                .ok_or_else(|| anyhow::anyhow!("missing payload for load_curriculum"))?;
            
            // Call the curriculum receiver to process the payload
            let res = receiver::receive_curriculum(payload.clone())?;
            
            // Return a success JSON response
            Ok(serde_json::json!({"status":"ok","result": res}))
        }

        // Simple ping command for testing if engine is alive
        "ping" => Ok(serde_json::json!({"status":"ok","message":"pong"})),

        // Unknown action
        _ => Err(anyhow::anyhow!("unknown action: {}", action)),
    }
}
