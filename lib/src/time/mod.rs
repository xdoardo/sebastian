#![allow(dead_code)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimeTableConfig {
    pub academic_year: String,
    pub faculty: String,
    pub course: String,
    pub years: Vec<String>,
}
