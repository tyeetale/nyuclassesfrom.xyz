use serde::{Serialize, Deserialize}; 

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectName {
    name: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CourseInfoFull {
    pub name: String,
    pub deptCourseId: String,
    pub description: String,
    pub subjectCode: SubjectCode,
    pub sections: Vec<SectionFull>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CourseInfoSimple {
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
    pub waitlistTotal: u32,
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