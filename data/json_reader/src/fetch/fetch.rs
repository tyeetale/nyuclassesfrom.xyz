use std::collections::HashMap;
use tokio;

use crate::fetch::json_structure::{CourseInfoSimple, SubjectName};
use crate::fetch::types::Error;
use crate::fetch::url_builder::UrlBuilder;

use super::json_structure::CourseInfoFull;

pub async fn fetch_subjects() -> Result<HashMap<String, HashMap<String, SubjectName>>, Error> {
    // first step, fetch the course and school info
    let url = UrlBuilder::build_subjects_endpoint_url()?;

    match reqwest::get(url).await {
        Ok(res) => {
            match res
                .json::<HashMap<String, HashMap<String, SubjectName>>>()
                .await
            {
                Ok(map) => return Ok(map),
                Err(_) => return Err(Error::ParseContentFailed),
            }
        }
        Err(_) => return Err(Error::FetchContentFailed),
    };
}

pub async fn fetch_course_catalog(
    year: u32,
    semester: &String,
    school_subject_catalog: &HashMap<String, HashMap<String, SubjectName>>,
) -> Result<Vec<(String, String, String)>, Error> {
    // second step, iterate through all schools and their corresponding subjects
    // send get requests to the api
    let mut catalog = Vec::<(String, String, String)>::new();
    let mut fetch_handles = Vec::new();
    let mut parse_handles = Vec::new();
    for (school, subject_list) in school_subject_catalog {
        for (subject, _) in subject_list {
            let url = UrlBuilder::build_courses_endpoint_url(year, &semester, school, subject)?;
            fetch_handles.push(tokio::spawn(reqwest::get(url)));
        }
    }

    // wait for downloading of jsons
    let fetch_len = fetch_handles.len();
    for (i, jh) in fetch_handles.into_iter().enumerate() {
        match jh.await {
            Ok(Ok(res)) => {
                println!("Response retrieved: {}/{}", i + 1, fetch_len);
                parse_handles.push(tokio::spawn(res.json::<Vec<CourseInfoSimple>>()));
            }
            Ok(Err(_)) => return Err(Error::FetchContentFailed),
            _ => return Err(Error::JoinTaskFailed),
        }
    }

    // wait for parsing of jsons
    let parse_len = parse_handles.len();
    for (i, jh) in parse_handles.into_iter().enumerate() {
        match jh.await {
            Ok(Ok(courses)) => {
                println!("JSON parsed: {}/{}", i + 1, parse_len);
                catalog.extend(courses.into_iter().map(|x| (x.name, x.subjectCode.school, x.subjectCode.code)));
            }
            Ok(Err(_)) => return Err(Error::ParseContentFailed),
            _ => return Err(Error::JoinTaskFailed),
        }
    }
    Ok(catalog)
}

#[allow(dead_code)]
pub async fn fetch_course_details(catalog: &Vec<(String, String, String)>) -> Result<Vec<CourseInfoFull>, Error> {
    let course_info_list = Vec::new();
    let mut fetch_handles = Vec::new();
    for (course, school, subject) in catalog {
        let url = UrlBuilder::build_search_endpoint_url(course, school, subject)?;
        fetch_handles.push(reqwest::get(url));
    }

    Ok(course_info_list)
}