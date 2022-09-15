use crate::fetch::json_structure::*;
use chrono::prelude::*;
use chrono::Duration;
use reqwest::Url;
use crate::fetch::types::Error;

pub struct UrlBuilder {}

impl UrlBuilder {
    pub fn build_schools_endpoint_url() -> Result<Url, Error> {
        let url = "https://schedge.a1liu.com/schools";
        match Url::parse(url) {
            Ok(res) => Ok(res),
            _ => Err(Error::BuildUrlFailed(String::from(url))),
        }
    }
    pub fn build_subjects_endpoint_url() -> Result<Url, Error> {
        let url = "https://schedge.a1liu.com/subjects";
        match Url::parse(
            url
        ) {
            Ok(res) => Ok(res),
            _ => Err(Error::BuildUrlFailed(String::from(url))),
        }
    }
    pub fn build_courses_endpoint_url(year: u32, semester: &String, school: &String, subject: &String) -> Result<Url, Error> {
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
    pub fn build_search_endpoint_url(course: &String, school: &String, subject: &String) -> Result<Url, Error> {
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

pub fn get_naive_schedule(
    meetings: Option<&Vec<Meeting>>,
) -> Vec<(NaiveDateTime, NaiveDateTime)> {
    let mut res = Vec::new();
    match meetings {
        None => {}
        Some(dates) => {
            // for each of the date
            // convert to days in a week
            for date in dates {
                let naive_start_time =
                    NaiveDateTime::parse_from_str(&date.beginDate, "%Y-%m-%d %H:%M:%S").unwrap();
                // produce meeting schedule by adding its duration to the time
                let duration = Duration::minutes(date.minutesDuration.into());
                let naive_schedule = (naive_start_time, naive_start_time + duration);
                res.push(naive_schedule);
            }
        }
    }
    res
}

pub fn get_meeting_days(schedule: &Vec<(NaiveDateTime, NaiveDateTime)>) -> (bool, bool, bool, bool, bool, bool, bool) {
    let mut res = (false, false, false, false, false, false, false);
    for (start, end) in schedule {
        match start.weekday() {
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

pub fn get_start_end_date(schedule: Option<&Vec<Meeting>>) -> (String, String) {
    match schedule {
        None => {}
        Some(meetings) => {
            if meetings.len() > 0 {
                return (
                    NaiveDateTime::parse_from_str(&meetings[0].beginDate, "%Y-%m-%d %H:%M:%S").unwrap().date().to_string(),
                    NaiveDateTime::parse_from_str(&meetings[0].endDate, "%Y-%m-%d %H:%M:%S").unwrap().date().to_string(),
                );
            }
        }
    }
    (String::from("Date unavailabe"), String::from("Date unavailable"))
}