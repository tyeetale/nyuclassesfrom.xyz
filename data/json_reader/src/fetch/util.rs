use crate::fetch::json_structure::*;
use crate::fetch::types::Location;
use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::*;
use reqwest::header::STRICT_TRANSPORT_SECURITY;

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

fn convert_local_time_to_utc(location: &Location, local_datetime: &NaiveDateTime) -> DateTime<Utc> {
    match location {
        Location::Shanghai => Asia::Shanghai
            .from_local_datetime(local_datetime)
            .unwrap()
            .with_timezone(&Utc),
        Location::NewYork => America::New_York
            .from_local_datetime(local_datetime)
            .unwrap()
            .with_timezone(&Utc),
        Location::AbuDhabi => Asia::Dubai
            .from_local_datetime(local_datetime)
            .unwrap()
            .with_timezone(&Utc),
    }
}

