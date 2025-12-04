use serde::Serialize;
use crate::file_parser::Row;

#[derive(Serialize)]
pub struct Student {
    pub roll_no: String,
    pub full_name: String,
    pub class_level: String,
    pub gender: String,
}

pub fn process_student(row: Row) -> anyhow::Result<String> {
    if row.len() < 4 {
        return Err(anyhow::anyhow!("Row does not have enough columns"));
    }

    let student = Student {
        roll_no: row[0].clone(),
        full_name: row[1].clone(),
        class_level: row[2].clone(),
        gender: row[3].clone(),
    };

    Ok(serde_json::to_string(&student)?)
}
