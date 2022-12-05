use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FlatCourseInfo {
    // Unique ID used as the primary key
    pub id: u32,
    pub school_name: String,
    pub school_code: String,
    // no such field can be found
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
    pub course_location: String,
    pub session_start: String,
    pub session_end: String,
    // Do we need this field at the moment?
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
    // 24 hrs
    pub start_time: String,
    pub end_time: String,
    pub at: String,
    pub timezone: String,
    pub instructors: Vec<String>,
    pub description: Option<String>,
    pub prerequisits: Option<String>,
    pub fulfillment: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NestedCourseInfoFull {
    pub name: String,
    pub deptCourseId: String,
    pub description: Option<String>,
    pub subjectCode: SubjectCode,
    pub sections: Vec<SectionFull>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NestedCourseInfoSimple {
    pub name: String,
    pub deptCourseId: String,
    pub subjectCode: SubjectCode,
    pub sections: Vec<SectionSimple>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectCode {
    pub code: String,
    pub school: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SectionSimple {
    pub registrationNumber: u32,
    pub code: String,
    pub instructors: Vec<String>,
    pub r#type: String,
    pub status: String,
    pub meetings: Option<Vec<Meeting>>,
    pub instructionMode: Option<String>,
    pub name: String,
    pub minUnits: f32,
    pub maxUnits: f32,
    pub location: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SectionFull {
    pub registrationNumber: u32,
    pub code: String,
    pub instructors: Vec<String>,
    pub r#type: String,
    pub status: String,
    pub meetings: Option<Vec<Meeting>>,
    pub receitations: Option<Vec<SectionFull>>,
    pub waitlistTotal: Option<u32>,
    pub instructionMode: Option<String>,
    pub name: String,
    pub campus: String,
    pub minUnits: f32,
    pub maxUnits: f32,
    pub grading: String,
    pub location: String,
    pub notes: Option<String>,
    pub prerequisites: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Meeting {
    pub beginDate: String,
    pub minutesDuration: u32,
    pub endDate: String,
}
