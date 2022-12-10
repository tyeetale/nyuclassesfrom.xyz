use std::collections::HashMap;

use tokio;
use indicatif::ProgressBar;

use crate::json::{NestedCourseInfoFull, SchoolCatalog,};
use crate::types::Error;
use crate::util::{UrlBuilder, Season};

pub(crate) async fn fetch_schools(term: &String) -> Result<SchoolCatalog, Error> {
    let url = UrlBuilder::build_schools_endpoint_url(term)?;

    match reqwest::get(url).await {
        Ok(res) => {
            match res
                .json::<SchoolCatalog>()
                .await
            {
                Ok(map) => return Ok(map),
                Err(_) => return Err(Error::ParseContentFailed),
            }
        }
        Err(_) => return Err(Error::FetchContentFailed),
    };
}

pub(crate) async fn fetch_courses(
    term: &Season,
    year: u16,
    map: &HashMap<&String, &String>,
) -> Result<Vec<NestedCourseInfoFull>, Error> {
    // second step, iterate through all schools and their corresponding subjects
    // send get requests to the api
    let mut courses = Vec::new();
    let mut fetch_handles = Vec::new();
    let mut parse_handles = Vec::new();

    for (code, _) in map {
        let url = UrlBuilder::build_courses_endpoint_url(term, year, *code)?;
        fetch_handles.push(tokio::spawn(reqwest::get(url)));
    }
    
    let bar = ProgressBar::new(map.len() as u64);
    
    // wait for downloading of jsons
    for jh in fetch_handles.into_iter() {
        match jh.await {
            Ok(Ok(res)) => {
                // println!("Response retrieved: {}/{}", i + 1, fetch_len);
                bar.inc(1);
                parse_handles.push(tokio::spawn(res.json::<Vec<NestedCourseInfoFull>>()));
            }
            Ok(Err(_)) => return Err(Error::FetchContentFailed),
            _ => return Err(Error::JoinTaskFailed),
        }
    }
    bar.finish();

    // wait for parsing of jsons
    for jh in parse_handles.into_iter() {
        match jh.await {
            Ok(Ok(course)) => {
                courses.extend(course);
            }
            Ok(Err(_)) => return Err(Error::ParseContentFailed),
            _ => return Err(Error::JoinTaskFailed),
        }
    }
    Ok(courses)
}