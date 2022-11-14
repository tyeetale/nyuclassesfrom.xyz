use std::collections::HashMap;
use tokio;

use crate::json::{NestedCourseInfoFull, NestedCourseInfoSimple, Name};
use crate::types::Error;
use crate::util::UrlBuilder;

pub(crate) async fn fetch_subjects() -> Result<HashMap<String, HashMap<String, Name>>, Error> {
    // first step, fetch the course and school info
    let url = UrlBuilder::build_subjects_endpoint_url()?;

    match reqwest::get(url).await {
        Ok(res) => {
            match res
                .json::<HashMap<String, HashMap<String, Name>>>()
                .await
            {
                Ok(map) => return Ok(map),
                Err(_) => return Err(Error::ParseContentFailed),
            }
        }
        Err(_) => return Err(Error::FetchContentFailed),
    };
}

pub(crate) async fn fetch_schools() -> Result<HashMap<String, Name>, Error> {
    let url = UrlBuilder::build_schools_endpoint_url()?;

    match reqwest::get(url).await {
        Ok(res) => {
            match res
                .json::<HashMap<String, Name>>()
                .await
            {
                Ok(map) => return Ok(map),
                Err(_) => return Err(Error::ParseContentFailed),
            }
        }
        Err(_) => return Err(Error::FetchContentFailed),
    };
}

pub(crate) async fn fetch_course_catalog(
    year: u32,
    semester: &String,
    school_subject_catalog: &HashMap<String, HashMap<String, Name>>,
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
                parse_handles.push(tokio::spawn(res.json::<Vec<NestedCourseInfoSimple>>()));
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

// we can reduce the block rate of requests by writing a dynamic window for
// sending requests
pub(crate) async fn fetch_course_details(catalog: &Vec<(String, String, String)>) -> Result<Vec<NestedCourseInfoFull>, Error> {
    let mut course_info_list = Vec::new();
    let mut fetch_handles = Vec::new();
    let mut parse_handles = Vec::new();
    for (course, school, subject) in catalog {
        let url = UrlBuilder::build_search_endpoint_url(course, school, subject)?;
        fetch_handles.push(tokio::spawn(reqwest::get(url)));
    }

    let fetch_len = fetch_handles.len();
    for (i, jh) in fetch_handles.into_iter().enumerate() {
        match jh.await {
            Ok(Ok(res)) => {
                println!("Response retrieved: {}/{}", i, fetch_len);
                parse_handles.push(tokio::spawn(res.json::<Vec<NestedCourseInfoFull>>()));
            }
            Ok(Err(_)) => return Err(Error::FetchContentFailed),
            Err(_) => return Err(Error::JoinTaskFailed),
        }
    }

    let parse_len = parse_handles.len();
    for (i, jh) in parse_handles.into_iter().enumerate() {
        match jh.await {
            Ok(Ok(res)) => {
                println!("JSON parsed: {}/{}", i, parse_len);
                course_info_list.extend(res);
            },
            Ok(Err(error)) => {
                println!("{:?}", error);
                return Err(Error::ParseContentFailed);
            },
            _ => return Err(Error::JoinTaskFailed),
        }
    }

    Ok(course_info_list)
}