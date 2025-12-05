use axum::{
    Router,
    routing::{get, post},
    extract::Json,
    response::IntoResponse,
};
use serde_json::Value;
use crate::file_parser::parse_docx_table;
use crate::student_processor::process_student;
use crate::school_form_parser;
use crate::auth::get_authenticated_school_id;
use anyhow::{Result, anyhow};

pub fn create_router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/command", post(run_command))
}

async fn ping() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "message": "pong"
    }))
}

async fn run_command(Json(body): Json<Value>) -> impl IntoResponse {
    match serde_json::to_string(&body)
        .ok()
        .and_then(|s| handle_command(&s).ok())
    {
        Some(v) => Json(v),
        None => Json(serde_json::json!({
            "status": "error",
            "error": "Invalid command"
        })),
    }
}

pub fn handle_command(raw: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(raw)?;
    let action = v["action"]
        .as_str()
        .ok_or_else(|| anyhow!("missing action"))?;

    match action {
        "parse_school_form" => {
            let token = v["token"].as_str().ok_or_else(|| anyhow!("missing token"))?;
            let auth_school_id = get_authenticated_school_id(token)?;

            let rows = parse_docx_table(v["file_path"].as_str().unwrap())?;
            let students = school_form_parser::process_school_form(&auth_school_id, rows)?;

            Ok(serde_json::json!({
                "status": "ok",
                "school_id": auth_school_id,
                "students": students
            }))
        }

        "parse_docx" => {
            let path = v["path"].as_str().ok_or_else(|| anyhow!("missing path"))?;
            let rows = parse_docx_table(path)?;

            Ok(serde_json::json!({
                "status": "ok",
                "rows": rows
            }))
        }

        "parse_student_row" => {
            let row: Vec<String> = v["row"]
                .as_array()
                .ok_or_else(|| anyhow!("row must be array"))?
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect();

            let student = process_student(row)?;
            Ok(serde_json::json!({ "status": "ok", "student": student }))
        }

        "ping" => Ok(serde_json::json!({ "status": "ok", "message": "pong" })),

        _ => Err(anyhow!("unknown action: {}", action)),
    }
}
