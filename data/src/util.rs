use crate::json::*;
use crate::types::Error;
use chrono::prelude::*;
use chrono::Duration;
use dotenv;
use reqwest::Url;
use std::cmp;
use std::env;

pub struct UrlBuilder {}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Season {
    January,
    Spring,
    Summer,
    Fall,
}

impl ToString for Season {
    fn to_string(&self) -> String {
        match self {
            Self::January => String::from("January"),
            Self::Spring => String::from("Spring"),
            Self::Summer => String::from("Summer"),
            Self::Fall => String::from("Fall"),
        }
    }
}

impl UrlBuilder {
    // this is the url points to the latest api
    pub fn build_schools_endpoint_url(term: &String) -> Result<Url, Error> {
        let url = &*format!("https://nyu.a1liu.com/api/schools/{}", term);
        match Url::parse(url) {
            Ok(res) => Ok(res),
            _ => Err(Error::BuildUrlFailed(String::from(url))),
        }
    }
    pub fn build_courses_endpoint_url(
        term: &Season,
        year: u16,
        subject: &String,
    ) -> Result<Url, Error> {
        let url = format!(
            "https://nyu.a1liu.com/api/courses/{}/{}",
            get_term_str(term, year),
            subject
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
    let (description, fulfillment, prerequisite) = {
        match &course.description {
            Some(description) => get_description_fulfillment_prerequisite(&description),
            None => (None, None, None),
        }
    };
    let timezone = {
        if course.sections[0].meetings.is_none()
            || course.sections[0].meetings.as_ref().unwrap().len() == 0
        {
            String::from("")
        } else {
            get_time_zone(&course.sections[0].meetings.as_ref().unwrap()[0].beginDateLocal)
        }
    };
    for section in &course.sections {
        let (start_date, end_date) = {
            if let Some(meetings) = &section.meetings {
                get_start_end_date(meetings)
            } else {
                (
                    String::from("Time unavailable"),
                    String::from("Time unavailable"),
                )
            }
        };
        // Could be problematic when different sessions in a week have different hours
        let (start_hour, end_hour) = {
            if section.meetings.is_none() || section.meetings.as_ref().unwrap().len() == 0 {
                (
                    String::from("Time unavailable"),
                    String::from("Time unavailable"),
                )
            } else {
                get_start_end_hour(&section.meetings.as_ref().unwrap()[0])
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
            school_name: school.to_owned(),
            subject_name: subject.to_owned(),
            term: term.to_owned(),
            year: year as u32,
            // determined based on the school
            timezone: timezone.to_owned(),
            subject_code: course.subjectCode.to_owned(),
            subject_number: course.deptCourseId.to_owned(),
            class_name: course.name.to_owned(),
            units: section.maxUnits,
            class_number: section.registrationNumber,
            section: section.code.to_owned(),
            grading: section.grading.to_owned(),
            course_location: section.location.to_owned(),
            class_status: section.status.to_owned(),
            instruction_mode: section.instructionMode.to_owned(),
            component: section.r#type.to_owned(),
            meet_monday: meet_days.0,
            meet_tuesday: meet_days.1,
            meet_wednesday: meet_days.2,
            meet_thursday: meet_days.3,
            meet_friday: meet_days.4,
            meet_saturday: meet_days.5,
            meet_sunday: meet_days.6,
            instructors: section.instructors.to_owned(),
            // Need to fill this field with other values
            prerequisite: prerequisite.to_owned(),
            notes: section.notes.to_owned(),
            at: section.location.to_owned(),
            // we need to calculate the rest of the fields
            start_time: start_hour,
            end_time: end_hour,
            session_start: start_date,
            session_end: end_date,
            description: description.to_owned(),
            fulfillment: fulfillment.to_owned(),
        });
        *id += 1;
    }
    Ok(res)
}

fn get_naive_date_time(date: &String) -> NaiveDateTime {
    match NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S%z") {
        Ok(datetime) => datetime,
        _ => NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%SZ").unwrap(),
    }
}

fn get_start_end_hour(meeting: &Meeting) -> (String, String) {
    let naive_start_time = get_naive_date_time(&meeting.beginDateLocal);
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
        let naive_date_time = get_naive_date_time(&meeting.beginDateLocal);
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
    if meetings.len() == 0 {
        return ("".to_string(), "".to_string());
    }
    let mut earliest_date = get_naive_date_time(&meetings[0].beginDateLocal).date();
    let mut latest_date = get_naive_date_time(&meetings[0].endDateLocal).date();
    for meeting in meetings {
        let begin = get_naive_date_time(&meeting.beginDateLocal).date();
        let end = get_naive_date_time(&meeting.endDateLocal).date();
        earliest_date = cmp::min(earliest_date, begin);
        latest_date = cmp::max(latest_date, end);
    }
    (earliest_date.to_string(), latest_date.to_string())
}

fn get_description_fulfillment_prerequisite(
    string: &String,
) -> (Option<String>, Option<String>, Option<String>) {
    if string.len() == 0 {
        return (None, None, None);
    }
    let mut description = None;
    let mut fulfillment = None;
    let mut prerequisite = None;

    let tmp = string.to_lowercase();

    // This is unable to cover all string patterns (actual patterns include pre-requisite, and may appear in notes field)
    match (tmp.find("fulfillment"), tmp.find("prerequisite")) {
        (None, None) => description = Some(string.clone()),
        (Some(ful_idx), None) => {
            // for some reasons we don't need to convert to UTF-8 string anymore...
            let end = ful_idx + 12;
            if ful_idx == 0 {
                // everything is fulfillment
                fulfillment = Some(clean_up_string(&string[end..]).to_string());
            } else {
                // split the string at the index
                description = Some(clean_up_string(&string[..ful_idx]).to_string());
                fulfillment = Some(clean_up_string(&string[end..]).to_string());
            }
        }
        (None, Some(pre_idx)) => {
            let end = pre_idx + 13;
            if pre_idx == 0 {
                prerequisite = Some(clean_up_string(&string[end..]).to_string());
            } else {
                description = Some(clean_up_string(&string[..pre_idx]).to_string());
                prerequisite = Some(clean_up_string(&string[end..]).to_string());
            }
        }
        (Some(ful_idx), Some(pre_idx)) => {
            let end1 = ful_idx + 12;
            let end2 = pre_idx + 13;
            if ful_idx < pre_idx {
                if ful_idx != 0 {
                    description = Some(clean_up_string(&string[..ful_idx]).to_string());
                }
                fulfillment = Some(clean_up_string(&string[end1..pre_idx]).to_string());
                prerequisite = Some(clean_up_string(&string[end2..]).to_string());
            } else {
                if pre_idx != 0 {
                    description = Some(clean_up_string(&string[..pre_idx]).to_string());
                }
                prerequisite = Some(clean_up_string(&string[end2..ful_idx]).to_string());
                fulfillment = Some(clean_up_string(&string[end1..]).to_string());
            }
        }
    }
    (description, fulfillment, prerequisite)
}

fn get_time_zone(date: &String) -> String {
    // Only supports three locations now
    match DateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S%z") {
        Ok(time) => time.timezone().to_string(),
        _ => String::from("+00:00"),
    }
}

fn clean_up_string(string: &str) -> &str {
    if string.len() == 0 {
        return string;
    }
    // println!("{}", string);
    let mut start = 0;
    let mut end = string.len() - 1;
    // Sanity check before stripping
    while string.chars().nth(start).is_some()
        && (string.chars().nth(start).unwrap() == ':' || string.chars().nth(start).unwrap() == ' ')
    {
        start += 1;
    }
    while string.chars().nth(end).is_some() && string.chars().nth(end).unwrap() == ' ' && end > 0 {
        end -= 1;
    }
    &string[start..end + 1]
}

pub fn read_env_variables() -> (String, String) {
    dotenv::dotenv().expect("Failed to read .env file");
    (
        env::var("DB_URL").expect("URL not found"),
        env::var("DB_KEY").expect("Key not found"),
    )
}

pub fn get_term_str(season: &Season, year: u16) -> String {
    format!("{}{}", season.get_short_name(), year)
}

#[cfg(test)]
mod tests {
    use crate::json::Meeting;
    use crate::util::{
        get_description_fulfillment_prerequisite, get_meeting_days, get_start_end_date,
        get_start_end_hour, get_time_zone, read_env_variables,
    };

    #[test]
    fn test_get_naive_date_time() {
        let m1 = Meeting {
            beginDate: String::from("2023-01-31T01:45:00Z"),
            minutesDuration: 75,
            endDate: String::from("2023-05-12T15:59:00Z"),
            beginDateLocal: String::from("2023-01-31T09:45:00+08:00"),
            endDateLocal: String::from("2023-05-12T23:59:00+08:00"),
        };
        let m2 = Meeting {
            beginDate: String::from("2022-09-01T10:00:00Z"),
            minutesDuration: 0,
            endDate: String::from("2022-12-14T23:59:00Z"),
            beginDateLocal: String::from("2022-09-01T10:00:00+08:00"),
            endDateLocal: String::from("2022-12-14T23:59:00+08:00"),
        };
        let sched1 = get_start_end_hour(&m1);
        assert_eq!(sched1, (String::from("09:45:00"), String::from("11:00:00")));

        let sched2 = get_start_end_hour(&m2);
        assert_eq!(sched2, (String::from("10:00:00"), String::from("10:00:00")));
    }

    #[test]
    fn test_get_meeting_days() {
        let sched1 = vec![
            Meeting {
                beginDate: String::from("2022-08-31T09:00:00Z"),
                minutesDuration: 180,
                endDate: String::from("2022-08-31T23:59:00Z"),
                beginDateLocal: String::from("2022-08-31T09:00:00+08:00"),
                endDateLocal: String::from("2022-08-31T23:59:00+08:00"),
            },
            Meeting {
                beginDate: String::from("2022-08-30T09:00:00Z"),
                minutesDuration: 420,
                endDate: String::from("2022-08-30T23:59:00Z"),
                beginDateLocal: String::from("2022-08-30T09:00:00+08:00"),
                endDateLocal: String::from("2022-08-30T23:59:00+08:00"),
            },
            Meeting {
                beginDate: String::from("2022-08-29T09:00:00Z"),
                minutesDuration: 180,
                endDate: String::from("2022-08-29T23:59:00Z"),
                beginDateLocal: String::from("2022-08-29T09:00:00+08:00"),
                endDateLocal: String::from("2022-08-29T23:59:00+08:00"),
            },
        ];

        let meeting_days = get_meeting_days(&sched1);
        assert_eq!(meeting_days, (true, true, true, false, false, false, false));
    }

    #[test]
    fn test_get_start_end_date() {
        let sched1 = vec![
            Meeting {
                beginDate: String::from("2022-08-31T09:00:00Z"),
                minutesDuration: 180,
                endDate: String::from("2022-08-31T23:59:00Z"),
                beginDateLocal: String::from("2022-08-31T09:00:00-05:00"),
                endDateLocal: String::from("2022-08-31T23:59:00-05:00"),
            },
            Meeting {
                beginDate: String::from("2022-08-30T09:00:00Z"),
                minutesDuration: 420,
                endDate: String::from("2022-08-30T23:59:00Z"),
                beginDateLocal: String::from("2022-08-30T09:00:00-05:00"),
                endDateLocal: String::from("2022-08-30T23:59:00-05:00"),
            },
            Meeting {
                beginDate: String::from("2022-08-29T09:00:00Z"),
                minutesDuration: 180,
                endDate: String::from("2022-08-29T23:59:00Z"),
                beginDateLocal: String::from("2022-08-29T09:00:00-05:00"),
                endDateLocal: String::from("2022-08-29T23:59:00-05:00"),
            },
        ];

        let sched2 = vec![
            Meeting {
                beginDate: String::from("2022-06-30T09:00:00Z"),
                minutesDuration: 180,
                endDate: String::from("2022-08-31T23:59:00Z"),
                beginDateLocal: String::from("2022-06-30T09:00:00-05:00"),
                endDateLocal: String::from("2022-08-31T23:59:00-05:00"),
            },
            Meeting {
                beginDate: String::from("2022-07-01T09:00:00Z"),
                minutesDuration: 420,
                endDate: String::from("2022-08-30T23:59:00Z"),
                beginDateLocal: String::from("2022-07-01T09:00:00-05:00"),
                endDateLocal: String::from("2022-08-30T23:59:00-05:00"),
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
        let des1 = String::from("An embedded system is a computer system with a dedicated function within a larger mechanical or electrical system, often with real-time computing constraints. It is embedded as part of a complete device often including hardware and mechanical parts. Embedded systems control many devices in common use today. Topics covered include microcontroller architecture, assembler programming, interrupts, peripheral interfacing, embedded system design, higher-level languages on embedded Systems, as well as a brief introduction to real-time operating systems. Practical Lab Exercises complement the lectures. The students will further specialize and consolidate their knowledge through semester-long hands-on projects. Prerequisite: ( CSCI-SHU 11 or CSCI-SHU 101 ) AND (CENG-SHU 202 or CENG-SHU 201). Fulfillment: CS elective; CE Required, EE Additional Electives.");
        let (description, fulfillment, prerequisite) =
            get_description_fulfillment_prerequisite(&des1);
        assert_eq!(description, Some(String::from("An embedded system is a computer system with a dedicated function within a larger mechanical or electrical system, often with real-time computing constraints. It is embedded as part of a complete device often including hardware and mechanical parts. Embedded systems control many devices in common use today. Topics covered include microcontroller architecture, assembler programming, interrupts, peripheral interfacing, embedded system design, higher-level languages on embedded Systems, as well as a brief introduction to real-time operating systems. Practical Lab Exercises complement the lectures. The students will further specialize and consolidate their knowledge through semester-long hands-on projects.")));
        assert_eq!(
            fulfillment,
            Some(String::from(
                "CS elective; CE Required, EE Additional Electives."
            ))
        );
        assert_eq!(
            prerequisite,
            Some(String::from(
                "( CSCI-SHU 11 or CSCI-SHU 101 ) AND (CENG-SHU 202 or CENG-SHU 201)."
            ))
        );

        let des2 = String::from("In this class, students will learn about the theoretical foundations of machine learning and how to apply these to solve real-world data-driven problems. We will apply machine learning to numerical, textual, and image data. Topics will be drawn from perceptron algorithm, regression, gradient descent and stochastic gradient descent, support vector machines, kernels for support vector machines, recommendation systems, decision trees and random forests, maximum likelihood, estimation, logistic regression, neural networks and the back propagation algorithm, convolutional neural networks, recurrent neural networks, Bayesian analysis and naive Bayes, clustering, latent Dirichlet allocation (LDA), sentiment analysis, dimensionality reduction and principle component analysis, reinforcement learning. Prerequisites: Introduction to Computer Programming, Calculus, and (Probability and Statistics OR Theory of Probability OR Statistics for Business & Economics). Fulfillment: Business Analytics Track; Computer Science Electives; Data Science Major Data Analysis Courses.");
        let (description, fulfillment, prerequisite) =
            get_description_fulfillment_prerequisite(&des2);
        assert_eq!(description, Some(String::from("In this class, students will learn about the theoretical foundations of machine learning and how to apply these to solve real-world data-driven problems. We will apply machine learning to numerical, textual, and image data. Topics will be drawn from perceptron algorithm, regression, gradient descent and stochastic gradient descent, support vector machines, kernels for support vector machines, recommendation systems, decision trees and random forests, maximum likelihood, estimation, logistic regression, neural networks and the back propagation algorithm, convolutional neural networks, recurrent neural networks, Bayesian analysis and naive Bayes, clustering, latent Dirichlet allocation (LDA), sentiment analysis, dimensionality reduction and principle component analysis, reinforcement learning.")));
        assert_eq!(fulfillment, Some(String::from("Business Analytics Track; Computer Science Electives; Data Science Major Data Analysis Courses.")));
        assert_eq!(prerequisite, Some(String::from("Introduction to Computer Programming, Calculus, and (Probability and Statistics OR Theory of Probability OR Statistics for Business & Economics).")));

        let des3 = String::from("In this class, students will learn about the theoretical foundations of machine learning and how to apply these to solve real-world data-driven problems. We will apply machine learning to numerical, textual, and image data. Topics will be drawn from perceptron algorithm, regression, gradient descent and stochastic gradient descent, support vector machines, kernels for support vector machines, recommendation systems, decision trees and random forests, maximum likelihood, estimation, logistic regression, neural networks and the back propagation algorithm, convolutional neural networks, recurrent neural networks, Bayesian analysis and naive Bayes, clustering, latent Dirichlet allocation (LDA), sentiment analysis, dimensionality reduction and principle component analysis, reinforcement learning.");
        let (description, fulfillment, prerequisite) =
            get_description_fulfillment_prerequisite(&des3);
        assert_eq!(description, Some(String::from("In this class, students will learn about the theoretical foundations of machine learning and how to apply these to solve real-world data-driven problems. We will apply machine learning to numerical, textual, and image data. Topics will be drawn from perceptron algorithm, regression, gradient descent and stochastic gradient descent, support vector machines, kernels for support vector machines, recommendation systems, decision trees and random forests, maximum likelihood, estimation, logistic regression, neural networks and the back propagation algorithm, convolutional neural networks, recurrent neural networks, Bayesian analysis and naive Bayes, clustering, latent Dirichlet allocation (LDA), sentiment analysis, dimensionality reduction and principle component analysis, reinforcement learning.")));
        assert_eq!(fulfillment, None);
        assert_eq!(prerequisite, None);

        let des4 = String::from("");
        let (description, fulfillment, prerequisite) =
            get_description_fulfillment_prerequisite(&des4);
        assert_eq!(description, None);
        assert_eq!(fulfillment, None);
        assert_eq!(prerequisite, None);

        let des5 = String::from("Fulfillment: hello.");
        let (description, fulfillment, prerequisite) =
            get_description_fulfillment_prerequisite(&des5);
        assert_eq!(description, None);
        assert_eq!(fulfillment, Some(String::from("hello.")));
        assert_eq!(prerequisite, None);

        let des6 = String::from("Prerequisite: permission of the department. Does not satisfy the major elective requirement. 2-4 credits Students majoring in computer science are permitted to work on an individual basis under the supervision of a full-time faculty member in the department if they have maintained an overall GPA of 3.0 and a GPA of 3.5 in computer science and have a study proposal that is approved by a computer science professor. Students are expected to spend about two to three hours a week per credit (a 4-credit IS would involve about ten to twelve hours a week) on their project. Fulfillment: Computer Science Major Electives. ");
        let (d, f, p) = get_description_fulfillment_prerequisite(&des6);
        assert_eq!(d, None);
        assert_eq!(p, Some(String::from("permission of the department. Does not satisfy the major elective requirement. 2-4 credits Students majoring in computer science are permitted to work on an individual basis under the supervision of a full-time faculty member in the department if they have maintained an overall GPA of 3.0 and a GPA of 3.5 in computer science and have a study proposal that is approved by a computer science professor. Students are expected to spend about two to three hours a week per credit (a 4-credit IS would involve about ten to twelve hours a week) on their project.")));
        assert_eq!(f, Some(String::from("Computer Science Major Electives.")))
    }

    #[test]
    fn test_get_time_zone() {
        let loc1 = String::from("2022-07-01T09:00:00-05:00");
        let loc2 = String::from("2023-01-30T08:45:00+08:00");
        let tz1 = get_time_zone(&loc1);
        let tz2 = get_time_zone(&loc2);
        assert_eq!(tz1, "-05:00");
        assert_eq!(tz2, "+08:00");
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
