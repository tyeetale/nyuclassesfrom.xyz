use crate::fetch::types::{Error, Location};
use crate::fetch::util::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FlatCourseInfo {
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
pub struct SubjectName {
    name: String,
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

impl NestedCourseInfoFull {
    pub fn flatten(
        self,
        year: u32,
        term: &String,
        school_name: &String,
        subject_name: &String,
    ) -> Result<Vec<FlatCourseInfo>, Error> {
        let mut res = Vec::new();
        for section in self.sections.iter() {
            let timezone = match &**school_name {
                "NYU Shanghai" => String::from("\"+8\""),
                "NYU Abu Dhabi" => String::from("\"+4\""),
                _ => String::from("\"-4\""),
            };
            let (mut description, mut fulfillment) = (None, None);
            if let Some(info) = &self.description {
                for (i, string) in info.split("\n").enumerate() {
                    if string.len() >= 11 && string[..11] == *"Fulfillment" {
                        fulfillment = Some(string.into());
                    } else if i == 0 {
                        // this is the first 
                        description = Some(string.into());
                    }
                }
            }
            let (start_date, end_date) = get_start_end_date(section.meetings.as_ref());
            let schedule = get_naive_schedule(section.meetings.as_ref());
            let meeting_days = get_meeting_days(&schedule);
            let (start_time, end_time) = {
                if schedule.len() > 0 {
                    (schedule[0].0.time().to_string(), schedule[0].1.time().to_string());
                }
                (String::from("Time unavailable"), String::from("Time unavailable"))
            };
            let info = FlatCourseInfo {
                school_name: school_name.clone(),
                school_code: self.subjectCode.school.clone(),
                subject_name: subject_name.clone(),
                subject_code: self.subjectCode.code.clone(),
                subject_number: self.deptCourseId.clone(),
                class_name: self.name.clone(),
                term: term.clone(),
                year: year,
                units: section.maxUnits,
                class_number: section.registrationNumber,
                section: section.code.clone(),
                grading: section.grading.clone(),
                course_location: section.location.clone(),
                session_start: start_date,
                session_end: end_date,
                class_status: section.status.clone(),
                instruction_mode: section.instructionMode.clone(),
                component: section.r#type.clone(),
                meet_monday: meeting_days.0,
                meet_tuesday: meeting_days.1,
                meet_wednesday: meeting_days.2,
                meet_thursday: meeting_days.3,
                meet_friday: meeting_days.4,
                meet_saturday: meeting_days.5,
                meet_sunday: meeting_days.6,
                start_time: start_time,
                end_time: end_time,
                at: section.location.clone(),
                timezone: timezone,
                instructors: section.instructors.clone(),
                description: description,
                prerequisits: section.prerequisites.clone(),
                fulfillment: fulfillment,
                notes: section.notes.clone(),
            };
            res.push(info);
        }
        Ok(res)
    }
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
