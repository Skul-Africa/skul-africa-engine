use anyhow::{Result, anyhow};
use serde::Serialize;
use crate::file_parser::Row;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ParsedStudent {
    pub school_id: String,
    pub firstname: String,
    pub lastname: String,
    pub gender: String,

    #[serde(rename = "othername")]
    pub othername: Option<String>,

    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<String>,

    pub email: Option<String>,
    pub phone: Option<String>,

    #[serde(rename = "admissionNumber")]
    pub admission_number: Option<String>,

    pub address: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,

    #[serde(rename = "bloodGroup")]
    pub blood_group: Option<String>,

    pub genotype: Option<String>,
    pub allergies: Option<String>,
    #[serde(rename = "medicalNotes")]
    pub medical_notes: Option<String>,

    #[serde(rename = "parentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "parentPhone")]
    pub parent_phone: Option<String>,
    #[serde(rename = "parentEmail")]
    pub parent_email: Option<String>,
    #[serde(rename = "parentAddress")]
    pub parent_address: Option<String>,

    pub relationship: Option<String>,

    #[serde(rename = "emergencyName")]
    pub emergency_name: Option<String>,
    #[serde(rename = "emergencyPhone")]
    pub emergency_phone: Option<String>,
    #[serde(rename = "emergencyRelationship")]
    pub emergency_relationship: Option<String>,

    #[serde(rename = "currentSession")]
    pub current_session: Option<String>,
    #[serde(rename = "currentTerm")]
    pub current_term: Option<String>,
}

fn get_header_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("firstname", "firstname"),
        ("first name", "firstname"),
        ("lastname", "lastname"),
        ("last name", "lastname"),
        ("othername", "othername"),
        ("other name", "othername"),
        ("gender", "gender"),
        ("sex", "gender"),
        ("dob", "date_of_birth"),
        ("dateofbirth", "date_of_birth"),
        ("date of birth", "date_of_birth"),
        ("email", "email"),
        ("phone", "phone"),
        ("admission number", "admission_number"),
        ("admissionno", "admission_number"),
        ("address", "address"),
        ("state", "state"),
        ("city", "city"),
        ("bloodgroup", "blood_group"),
        ("genotype", "genotype"),
        ("allergies", "allergies"),
        ("disabilities", "disabilities"),
        ("medicalnotes", "medical_notes"),
        ("parentname", "parent_name"),
        ("parentphone", "parent_phone"),
        ("parentemail", "parent_email"),
        ("parentaddress", "parent_address"),
        ("relationship", "relationship"),
        ("emergencyname", "emergency_name"),
        ("emergencyphone", "emergency_phone"),
        ("emergencyrelationship", "emergency_relationship"),
        ("currentsession", "current_session"),
        ("currentterm", "current_term"),
    ])
}

pub fn process_school_form(school_id: &str, rows: Vec<Row>) -> Result<Vec<ParsedStudent>> {
    if rows.is_empty() { return Ok(vec![]); }

    let header_map = get_header_map();
    let raw_headers: Vec<String> = rows[0]
        .iter()
        .map(|h| {
            let key = h.to_lowercase().trim().to_string();
            header_map.get(key.as_str()).copied().unwrap_or(h.as_str()).to_string()
        })
        .collect();

    let mut students = Vec::new();

    for (row_index, row) in rows.iter().enumerate().skip(1) {
        let mut student = ParsedStudent {
            school_id: school_id.to_string(),
            firstname: "".to_string(),
            lastname: "".to_string(),
            gender: "".to_string(),
            othername: None,
            date_of_birth: None,
            email: None,
            phone: None,
            admission_number: None,
            address: None,
            state: None,
            city: None,
            blood_group: None,
            genotype: None,
            allergies: None,
            medical_notes: None,
            parent_name: None,
            parent_phone: None,
            parent_email: None,
            parent_address: None,
            relationship: None,
            emergency_name: None,
            emergency_phone: None,
            emergency_relationship: None,
            current_session: None,
            current_term: None,
        };

        for (i, value) in row.iter().enumerate() {
            if i >= raw_headers.len() { continue; }
            match raw_headers[i].as_str() {
                "firstname" => student.firstname = value.clone(),
                "lastname" => student.lastname = value.clone(),
                "gender" => student.gender = value.clone(),
                "othername" => student.othername = Some(value.clone()),
                "date_of_birth" => student.date_of_birth = Some(value.clone()),
                "email" => student.email = Some(value.clone()),
                "phone" => student.phone = Some(value.clone()),
                "admission_number" => student.admission_number = Some(value.clone()),
                "address" => student.address = Some(value.clone()),
                "state" => student.state = Some(value.clone()),
                "city" => student.city = Some(value.clone()),
                "blood_group" => student.blood_group = Some(value.clone()),
                "genotype" => student.genotype = Some(value.clone()),
                "allergies" => student.allergies = Some(value.clone()),
                "medical_notes" => student.medical_notes = Some(value.clone()),
                "parent_name" => student.parent_name = Some(value.clone()),
                "parent_phone" => student.parent_phone = Some(value.clone()),
                "parent_email" => student.parent_email = Some(value.clone()),
                "parent_address" => student.parent_address = Some(value.clone()),
                "relationship" => student.relationship = Some(value.clone()),
                "emergency_name" => student.emergency_name = Some(value.clone()),
                "emergency_phone" => student.emergency_phone = Some(value.clone()),
                "emergency_relationship" => student.emergency_relationship = Some(value.clone()),
                "current_session" => student.current_session = Some(value.clone()),
                "current_term" => student.current_term = Some(value.clone()),
                _ => {}
            }
        }

        if student.firstname.is_empty() || student.lastname.is_empty() || student.gender.is_empty() {
            return Err(anyhow!("Missing required fields at row {}", row_index + 1));
        }

        students.push(student);
    }

    Ok(students)
}
