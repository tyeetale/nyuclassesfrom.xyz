use crate::json::*;
use crate::types::Error;
use chrono::prelude::*;
use chrono::Duration;
use std::env;
use dotenv;
use reqwest::Url;
use std::cmp;

pub struct UrlBuilder {}

pub enum Season {
    January,
    Spring,
    Summer,
    Fall,
}

impl UrlBuilder {
    // this is the url points to the latest api
    pub fn build_schools_endpoint_url(term: &String) -> Result<Url, Error> {
        let url = &*format!("https://nyu.a1liu.com/api/schools/{}", term);
        match Url::parse(url) {
            Ok(res) => Ok(res),
            _ => Err(Error::BuildUrlFailed(String::from(url)))
        }
    }
    pub fn build_subjects_endpoint_url() -> Result<Url, Error> {
        let url = "https://schedge.a1liu.com/subjects";
        match Url::parse(url) {
            Ok(res) => Ok(res),
            _ => Err(Error::BuildUrlFailed(String::from(url))),
        }
    }
    pub fn build_courses_endpoint_url(
        year: u16,
        semester: &String,
        school: &String,
        subject: &String,
    ) -> Result<Url, Error> {
        let url = format!(
            "https://schedge.a1liu.com/{year}/{semester}/{school}/{subject}",
            year = year,
            semester = *semester,
            school = school,
            subject = subject
        );
        match Url::parse(&*url) {
            Ok(res) => Ok(res),
            _ => Err(Error::BuildUrlFailed(url)),
        }
    }
    pub fn build_search_endpoint_url(
        course: &String,
        school: &String,
        subject: &String,
    ) -> Result<Url, Error> {
        let url = format!(
            "https://schedge.a1liu.com/2022/fa/search?full=true&query={course}&school={school}&subject={subject}",
            course=course,
            school=school,
            subject=subject
        );
        match Url::parse(&*url) {
            Ok(res) => Ok(res),
            _ => Err(Error::BuildUrlFailed(url)),
        }
    }
}

impl Season {
    fn get_short_name(&self) -> String {
        let res = match self {
            Season::January => "ja",
            Season::Spring => "sp",
            Season::Summer => "su",
            Season::Fall => "fa",
        };
        res.to_string()
    }
}

pub fn flatten(
    id: &mut u32,
    school: &String,
    subject: &String,
    year: u16,
    term: &String,
    course: &NestedCourseInfoFull,
) -> Result<Vec<FlatCourseInfo>, Error> {
    let mut res = Vec::new();
    // we create an instance for each session
    let (description, fulfillment) = {
        match &course.description {
            Some(description) => get_description_fulfillment(&description),
            None => (None, None),
        }
    };
    let timezone = get_time_zone(school);
    for section in &course.sections {
        let (start_date, end_date) = {
            if let Some(meetings) = &section.meetings {
                get_start_end_date(meetings)
            } else {
                (String::from("Time unavailable"), String::from("Time unavailable"))
            }
        };
        // Could be problematic when different sessions in a week have different hours
        let (start_hour, end_hour) = {
            if let Some(meetings) = &section.meetings {
                get_start_end_hour(&meetings[0])
            } else {
                (String::from("Time unavailable"), String::from("Time unavailable"))
            }
        };
        let meet_days = {
            if let Some(meetings) = &section.meetings {
                get_meeting_days(meetings)
            } else {
                (false, false, false, false, false, false, false)
            }
        };
        res.push(FlatCourseInfo {
            // These fields will be determined by inputs
            id: *id,
            school_name: school.clone(),
            subject_name: subject.clone(),
            term: term.clone(),
            year: year as u32,
            // determined based on the school
            timezone: timezone.clone(),
            school_code: course.subjectCode.school.clone(),
            subject_code: course.subjectCode.code.clone(),
            subject_number: course.deptCourseId.clone(),
            class_name: course.name.clone(),
            units: section.maxUnits,
            class_number: section.registrationNumber,
            section: section.code.clone(),
            grading: section.grading.clone(),
            course_location: section.location.clone(),
            class_status: section.status.clone(),
            instruction_mode: section.instructionMode.clone(),
            component: section.r#type.clone(),
            meet_monday: meet_days.0,
            meet_tuesday: meet_days.1,
            meet_wednesday: meet_days.2,
            meet_thursday: meet_days.3,
            meet_friday: meet_days.4,
            meet_saturday: meet_days.5,
            meet_sunday: meet_days.6,
            instructors: section.instructors.clone(),
            prerequisits: section.prerequisites.clone(),
            notes: section.notes.clone(),
            at: section.location.clone(),
            // we need to calculate the rest of the fields
            start_time: start_hour,
            end_time: end_hour,
            session_start: start_date,
            session_end: end_date,
            description: description.clone(),
            fulfillment: fulfillment.clone(),
        });
        *id += 1;
    }
    Ok(res)
}

fn get_naive_date_time(date: &String) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap()
}

fn get_start_end_hour(meeting: &Meeting) -> (String, String) {
    let naive_start_time = get_naive_date_time(&meeting.beginDate);
    let duration = Duration::minutes(meeting.minutesDuration.into());
    let naive_end_time = naive_start_time + duration;
    let naive_schedule = (
        naive_start_time.format("%H:%M:%S").to_string(),
        naive_end_time.format("%H:%M:%S").to_string(),
    );
    naive_schedule
}

fn get_meeting_days(meetings: &Vec<Meeting>) -> (bool, bool, bool, bool, bool, bool, bool) {
    let mut res = (false, false, false, false, false, false, false);
    for meeting in meetings {
        let naive_date_time = get_naive_date_time(&meeting.beginDate);
        match naive_date_time.weekday() {
            Weekday::Mon => res.0 = true,
            Weekday::Tue => res.1 = true,
            Weekday::Wed => res.2 = true,
            Weekday::Thu => res.3 = true,
            Weekday::Fri => res.4 = true,
            Weekday::Sat => res.5 = true,
            Weekday::Sun => res.6 = true,
        }
    }
    res
}

fn get_start_end_date(meetings: &Vec<Meeting>) -> (String, String) {
    // get the earliest date as the start date
    // get the latest date as the end date
    let mut earliest_date = get_naive_date_time(&meetings[0].beginDate).date();
    let mut latest_date = get_naive_date_time(&meetings[0].endDate).date();
    for meeting in meetings {
        let begin = get_naive_date_time(&meeting.beginDate).date();
        let end = get_naive_date_time(&meeting.endDate).date();
        earliest_date = cmp::min(earliest_date, begin);
        latest_date = cmp::max(latest_date, end);
    }
    (earliest_date.to_string(), latest_date.to_string())
}

fn get_description_fulfillment(string: &String) -> (Option<String>, Option<String>) {
    // Split the string by \n
    let mut description = None;
    let mut fulfillment = None;
    for (i, s) in string.split("\n").enumerate() {
        // To accomodate utf-8 strings, we need to find the actual end index
        
        let end = s.char_indices().map(|(i, _)|i).nth(11);
        if end.is_some() && s[..end.unwrap()] == *"Fulfillment" {
            fulfillment = Some(clean_up_string(&s[11..]).to_string());
        } else if s.len() > 0 && i == 0 {
            // assuming that all descriptions starts with description text
            description = Some(clean_up_string(&s).to_string());
        }
    }
    (description, fulfillment)
}

fn get_time_zone(school: &String) -> String {
    // Only supports three locations now
    match &school[..] {
        "NYU Abu Dhabi" => String::from("\"+4\""),
        "NYU Shanghai" => String::from("\"+8\""),
        _ => String::from("\"-5\""),
    }
}

fn clean_up_string(string: &str) -> &str {
    if string.len() == 0 {
        return string;
    }
    let mut start = 0;
    let mut end = string.len() - 1;
    // Sanity check before stripping
    while string.chars().nth(start).is_some()
        && (string.chars().nth(start).unwrap() == ':' || string.chars().nth(start).unwrap() == ' ')
    {
        start += 1;
    }
    while string.chars().nth(end).is_some() && string.chars().nth(end).unwrap() == ' ' {
        end -= 1;
    }
    &string[start..end + 1]
}

pub fn read_env_variables() -> (String, String) {
    dotenv::dotenv().expect("Failed to read .env file");
    (env::var("DB_URL").expect("URL not found"), env::var("DB_KEY").expect("Key not found"))
}

pub fn get_term_str(season: &Season, year: u16) -> String {
    format!("{}{}", season.get_short_name(), year)
}

#[cfg(test)]
mod tests {
    use chrono;
    use std::fs::File;
    use std::io::{Write, BufReader, BufRead};
    use crate::json::{Meeting, NestedCourseInfoFull};
    use crate::util::{
        get_description_fulfillment, get_meeting_days, get_start_end_date, get_start_end_hour, get_time_zone, flatten, read_env_variables
    };

    #[test]
    fn test_get_naive_date_time() {
        let m1 = Meeting {
            beginDate: String::from("2022-09-01 14:00:00"),
            minutesDuration: 75,
            endDate: String::from("2022-12-14 23:59:00"),
            beginDateLocal: String::from("2022-09-01 14:00:00"),
            endDateLocal: String::from("2022-12-14 23:59:00"),
        };
        let m2 = Meeting {
            beginDate: String::from("2022-09-01 10:00:00"),
            minutesDuration: 0,
            endDate: String::from("2022-12-14 23:59:00"),
            beginDateLocal: String::from("2022-09-01 10:00:00"),
            endDateLocal: String::from("2022-12-14 23:59:00"),
        };
        let sched1 = get_start_end_hour(&m1);
        assert_eq!(sched1, (String::from("14:00:00"), String::from("15:15:00")));

        let sched2 = get_start_end_hour(&m2);
        assert_eq!(sched2, (String::from("10:00:00"), String::from("10:00:00")));
    }

    #[test]
    fn test_get_meeting_days() {
        let sched1 = vec![
            Meeting {
                beginDate: String::from("2022-08-31 09:00:00"),
                minutesDuration: 180,
                endDate: String::from("2022-08-31 23:59:00"),
                beginDateLocal: String::from("2022-08-31 09:00:00"),
                endDateLocal: String::from("2022-08-31 23:59:00"),
            },
            Meeting {
                beginDate: String::from("2022-08-30 09:00:00"),
                minutesDuration: 420,
                endDate: String::from("2022-08-30 23:59:00"),
                beginDateLocal: String::from("2022-08-30 09:00:00"),
                endDateLocal: String::from("2022-08-30 23:59:00"),
            },
            Meeting {
                beginDate: String::from("2022-08-29 09:00:00"),
                minutesDuration: 180,
                endDate: String::from("2022-08-29 23:59:00"),
                beginDateLocal: String::from("2022-08-29 09:00:00"),
                endDateLocal: String::from("2022-08-29 23:59:00"),
            },
        ];

        let meeting_days = get_meeting_days(&sched1);
        assert_eq!(meeting_days, (true, true, true, false, false, false, false));
    }

    #[test]
    fn test_get_start_end_date() {
        let sched1 = vec![
            Meeting {
                beginDate: String::from("2022-08-31 09:00:00"),
                minutesDuration: 180,
                endDate: String::from("2022-08-31 23:59:00"),
                beginDateLocal: String::from("2022-08-31 09:00:00"),
                endDateLocal: String::from("2022-08-31 23:59:00"),
            },
            Meeting {
                beginDate: String::from("2022-08-30 09:00:00"),
                minutesDuration: 420,
                endDate: String::from("2022-08-30 23:59:00"),
                beginDateLocal: String::from("2022-08-30 09:00:00"),
                endDateLocal: String::from("2022-08-30 23:59:00"),
            },
            Meeting {
                beginDate: String::from("2022-08-29 09:00:00"),
                minutesDuration: 180,
                endDate: String::from("2022-08-29 23:59:00"),
                beginDateLocal: String::from("2022-08-29 09:00:00"),
                endDateLocal: String::from("2022-08-29 23:59:00"),
            },
        ];

        let sched2 = vec![
            Meeting {
                beginDate: String::from("2022-06-30 09:00:00"),
                minutesDuration: 180,
                endDate: String::from("2022-08-31 23:59:00"),
                beginDateLocal: String::from("2022-06-30 09:00:00"),
                endDateLocal: String::from("2022-08-31 23:59:00"),
            },
            Meeting {
                beginDate: String::from("2022-07-01 09:00:00"),
                minutesDuration: 420,
                endDate: String::from("2022-08-30 23:59:00"),
                beginDateLocal: String::from("2022-07-01 09:00:00"),
                endDateLocal: String::from("2022-08-30 23:59:00"),
            },
        ];

        let (start_date, end_date) = get_start_end_date(&sched1);
        assert_eq!(start_date, String::from("2022-08-29"));
        assert_eq!(end_date, String::from("2022-08-31"));

        let (start_date, end_date) = get_start_end_date(&sched2);
        assert_eq!(start_date, String::from("2022-06-30"));
        assert_eq!(end_date, String::from("2022-08-31"));
    }

    #[test]
    fn test_get_description_requirement() {
        let des1 = String::from("Covers the principles and design of operating systems. Topics include process scheduling and synchronization, deadlocks, memory management (including virtual memory), input-output, and file systems. Programming assignments. \nPrerequisite: CSCI-SHU 210 Data Structures AND (CENG-SHU 202 Computer Architecture or CSCI-UA 201 Computer Systems Organization).\nFulfillment: Computer Science Major Required Courses; Computer Systems Engineering Major Elective; Data Science Major Courses for Concentration in Computer Science.");
        let (description, fulfillment) = get_description_fulfillment(&des1);
        assert_eq!(description, Some(String::from("Covers the principles and design of operating systems. Topics include process scheduling and synchronization, deadlocks, memory management (including virtual memory), input-output, and file systems. Programming assignments.")));
        assert_eq!(fulfillment, Some(String::from("Computer Science Major Required Courses; Computer Systems Engineering Major Elective; Data Science Major Courses for Concentration in Computer Science.")));

        let des2 = String::from("An intense hands-on study of practical techniques and methods of software engineering. Topics include: advanced object-oriented design, design patterns, refactoring, code optimization, universal modeling language, threading, user interface design, enterprise application development and development tools. All topics are integrated and applied during the semester-long group project. The aim of the project is to prepare students for dynamics in a real workplace. Members of the group will meet on a regular basis to discuss the project and to assign individual tasks. Students will be judged primarily on the final project presentations. \nPrerequisites: Intro to Computer Science. \nFulfillment: CS Electives.");
        let (description, fulfillment) = get_description_fulfillment(&des2);
        assert_eq!(description, Some(String::from("An intense hands-on study of practical techniques and methods of software engineering. Topics include: advanced object-oriented design, design patterns, refactoring, code optimization, universal modeling language, threading, user interface design, enterprise application development and development tools. All topics are integrated and applied during the semester-long group project. The aim of the project is to prepare students for dynamics in a real workplace. Members of the group will meet on a regular basis to discuss the project and to assign individual tasks. Students will be judged primarily on the final project presentations.")));
        assert_eq!(fulfillment, Some(String::from("CS Electives.")));

        let des3 = String::from("The course covers modeling an application and logical database design, the relational model and relational data definition and data manipulation languages, design of relational databases and normalization theory, physical database design, query processing and optimization, transaction processing focusing on concurrency and recovery. The labs emphasize experiential learning of database systems and applications and an insight into various database management systems and query languages.\nPrerequisite: CSCI-SHU 210 Data Structures.");
        let (description, fulfillment) = get_description_fulfillment(&des3);
        assert_eq!(description, Some(String::from("The course covers modeling an application and logical database design, the relational model and relational data definition and data manipulation languages, design of relational databases and normalization theory, physical database design, query processing and optimization, transaction processing focusing on concurrency and recovery. The labs emphasize experiential learning of database systems and applications and an insight into various database management systems and query languages.")));
        assert_eq!(fulfillment, None);

        let des4 = String::from("");
        let (description, fulfillment) = get_description_fulfillment(&des4);
        assert_eq!(description, None);
        assert_eq!(fulfillment, None);

        let des5 = String::from("Fulfillment: hello.");
        let (description, fulfillment) = get_description_fulfillment(&des5);
        assert_eq!(description, None);
        assert_eq!(fulfillment, Some(String::from("hello.")));
    }

    #[test]
    fn test_get_time_zone() {
        let loc1 = String::from("NYU Shanghai");
        let loc2 = String::from("NYU Abu Dhabi");
        let tz1 = get_time_zone(&loc1);
        let tz2 = get_time_zone(&loc2);
        assert_eq!(tz1, "\"+8\"");
        assert_eq!(tz2, "\"+4\"");
    }

    // #[test]
    // // This function flattens nested jsons and appends them to the json folder
    // fn test_output_json() {
    //     let file = File::open("./cached/course_non_flat.txt").unwrap();
    //     let file_name = format!("./cached/course_flat_{}.json", chrono::Local::now().naive_utc().to_string());
    //     let mut output = File::create(file_name).unwrap();
    //     let reader = BufReader::new(file);

    //     let year = 2022;
    //     let term = String::from("Fall");
    //     let school_name = String::from("NYU Shanghai");
    //     let subject_name = String::from("Bussiness and Finance");
    //     let mut id = 0;
    //     for line in reader.lines() {
    //         if let Ok(content) = line {
    //             let course: NestedCourseInfoFull = serde_json::from_str(&*content).unwrap();
    //             let res = flatten(&mut id, &school_name, &subject_name, year, &term, &course).unwrap();
    //             for info in res.iter() {
    //                 write!(output, "{}\n", serde_json::to_string(info).unwrap()).unwrap();
    //             }
    //         }
    //     }
    // }

    #[test]
    fn test_read_env_var() {
        let (url, pw) = read_env_variables();
        println!("URL: {}, Password: {}", url, pw);
    }
}
