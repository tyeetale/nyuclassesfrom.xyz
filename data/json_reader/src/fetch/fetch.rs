use std::collections::HashMap;
use tokio;

use crate::fetch::json_structure::{CourseInfo, SubjectName};
use crate::fetch::url_builder::UrlBuilder;
use crate::fetch::types::Error;

pub async fn fetch_subjects() -> Result<HashMap<String, HashMap<String, SubjectName>>, Error> {
    // first step, fetch the course and school info 
    let url = UrlBuilder::build_subjects_endpoint_url()?;

    match reqwest::get(url).await {
        Ok(res) => {
            match res.json::<HashMap<String, HashMap<String, SubjectName>>>().await {
                Ok(map) => return Ok(map),
                Err(_) => return Err(Error::DecodeContentFailed),
            }
        },
        Err(_) => return Err(Error::FetchContentFailed),
    };
}

pub async fn fetch_courses(year: u32, semester: &String, school_subject_catalog: &HashMap<String, HashMap<String, SubjectName>>) -> Result<(), Error> {
    // second step, iterate through all schools and their corresponding subjects
    // send get requests to the api
    let mut url_handles = Vec::new();
    for (school, subject_list) in school_subject_catalog {
        for (subject, _) in subject_list {
            let url = UrlBuilder::build_courses_endpoint_url(year, &semester, school, subject)?;
            url_handles.push(tokio::spawn(reqwest::get(url)));
        }
    }

    // wait for downloading of jsons
    let url_len = url_handles.len();
    let mut parse_handles = Vec::new();
    for (i, jh) in url_handles.into_iter().enumerate() {
        let res = jh.await.unwrap().unwrap();
        println!("Response retrieved: {}/{}", i + 1, url_len);
        parse_handles.push(tokio::spawn(res.json::<Vec<CourseInfo>>()));
    }

    // wait for completion of parsing
    let parse_len = parse_handles.len();
    for (i, jh) in parse_handles.into_iter().enumerate() {
        let _res = jh.await.unwrap().unwrap();
        println!("JSON parsed: {}/{}", i + 1, parse_len);
    }

    Ok(())
}