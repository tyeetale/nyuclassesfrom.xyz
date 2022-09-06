use serde::{Serialize, Deserialize}; 

#[derive(Serialize, Deserialize, Debug)]
pub struct SubjectName {
    name: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CourseInfo {
    name: String,
    deptCourseId: String,
    subjectCode: SubjectCode,
    sections: Vec<Section>, 
}

#[derive(Serialize, Deserialize, Debug)]
struct SubjectCode {
    code: String,
    school: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Section {
    registrationNumber: u32,
    code: String,
    instructors: Vec<String>,
    r#type: String,
    status: String,
    meetings: Option<Vec<Meeting>>,
    instructionMode: Option<String>,
    name: String,
    minUnits: f32,
    maxUnits: f32,
    location: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Meeting {
    beginDate: String,
    minutesDuration: u32,
    endDate: String,
}