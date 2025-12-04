use anyhow::Result;
use serde_json::Value;

use crate::file_parser::parse_docx_table;
use crate::student_processor::process_student;

pub fn handle_command(raw: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(raw)?;
    let action = v["action"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("missing action"))?;

    match action {
        "parse_docx" => {
            let path = v["path"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("missing path"))?;

            let rows = parse_docx_table(path)?;

            Ok(serde_json::json!({
                "status": "ok",
                "rows": rows
            }))
        }
        "ping" => {
    Ok(serde_json::json!({
        "status": "ok",
        "message": "pong"
    }))
}


        "parse_student_row" => {
            let row: Vec<String> = v["row"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("row must be array"))?
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect();

            let student = process_student(row)?;

            Ok(serde_json::json!({
                "status": "ok",
                "student": student
            }))
        }

        _ => Err(anyhow::anyhow!("unknown action: {}", action)),
    }
}
