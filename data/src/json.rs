use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FlatCourseInfo {
    pub id: u32,
    pub school_name: String,
    pub subject_name: String,
    pub subject_code: String,
    pub subject_number: String,
    pub class_name: String,
    pub term: String,
    pub year: u32,
    pub units: f32,
    pub class_number: u32,
    pub section: String,
    pub grading: String,
    pub course_location: Option<String>,
    pub session_start: String,
    pub session_end: String,
    pub class_status: String,
    pub instruction_mode: Option<String>,
    pub component: String,
    pub meet_monday: bool,
    pub meet_tuesday: bool,
    pub meet_wednesday: bool,
    pub meet_thursday: bool,
    pub meet_friday: bool,
    pub meet_saturday: bool,
    pub meet_sunday: bool,
    pub start_time: String,
    pub end_time: String,
    pub at: Option<String>,
    pub timezone: String,
    pub instructors: Vec<String>,
    pub description: Option<String>,
    pub prerequisite: Option<String>,
    pub fulfillment: Option<String>,
    pub notes: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NestedCourseInfoFull {
    pub name: String,
    pub deptCourseId: String,
    pub description: Option<String>,
    pub sections: Vec<SectionFull>,
    pub subjectCode: String,
}

// We only need this struct
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SectionFull {
    pub registrationNumber: u32,
    pub code: String,
    pub instructors: Vec<String>,
    pub r#type: String,
    pub status: String,
    pub meetings: Option<Vec<Meeting>>,
    pub recitations: Option<Vec<SectionFull>>,
    pub waitlistTotal: Option<u32>,
    pub instructionMode: Option<String>,
    pub name: Option<String>,
    pub campus: String,
    pub minUnits: f32,
    pub maxUnits: f32,
    pub grading: String,
    pub location: Option<String>,
    pub notes: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Meeting {
    pub beginDate: String,
    pub minutesDuration: u32,
    pub endDate: String,
    pub beginDateLocal: String,
    pub endDateLocal: String,
}

// We need to add another struct here to store the schools
#[derive(Debug, Serialize, Deserialize)]
pub struct SchoolCatalog {
    pub term: String,
    pub schools: Vec<SchoolInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchoolInfo {
    pub name: String,
    pub code: String,
    pub subjects: Vec<SubjectInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectInfo {
    pub code: String,
    pub name: String,
}