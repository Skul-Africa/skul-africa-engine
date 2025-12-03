// Defines the data structures for a curriculum
// These are used to deserialize JSON from NestJS
// and serialize data if needed later.

use serde::{Deserialize, Serialize};

/// Represents a full curriculum for a subject and class
#[derive(Debug, Serialize, Deserialize)]
pub struct Curriculum {
    pub school_id: String,       // ID of the school
    pub subject: String,         // Subject name (e.g., "Math")
    pub class_level: String,     // Class level (e.g., "JSS1")
    pub topics: Vec<Topic>,      // List of topics in this curriculum
}

/// Represents a single topic within a curriculum
#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub name: String,            // Topic name (e.g., "Numbers")
    #[serde(default)]
    pub subtopics: Vec<String>,  // List of subtopics
    #[serde(default)]
    pub objectives: Vec<String>, // Learning objectives
}
