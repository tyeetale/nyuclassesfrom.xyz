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

// we can reduce the block rate of requests by writing a dynamic window for
// sending requests
// pub(crate) async fn fetch_course_details(catalog: &Vec<(String, String, String)>, start: usize, end: usize) -> Result<Vec<NestedCourseInfoFull>, Error> {
//     let mut course_info_list = Vec::new();
//     let mut fetch_handles = Vec::new();
//     let mut parse_handles = Vec::new();

//     for i in start..end {
//         let url = UrlBuilder::build_search_endpoint_url(&catalog[i].0, &catalog[i].1, &catalog[i].2)?;
//         fetch_handles.push(tokio::spawn(reqwest::get(url)));
//     }

//     for jh in fetch_handles {
//         match jh.await {
//             Ok(Ok(res)) => {
//                 // println!("Response retrieved: {}/{}", i, fetch_len);
//                 parse_handles.push(tokio::spawn(res.json::<Vec<NestedCourseInfoFull>>()));
//             }
//             Ok(Err(_)) => return Err(Error::FetchContentFailed),
//             Err(_) => return Err(Error::JoinTaskFailed),
//         }
//     }

//     for jh in parse_handles.into_iter() {
//         match jh.await {
//             Ok(Ok(res)) => {
//                 // println!("JSON parsed: {}/{}", i, parse_len);
//                 course_info_list.extend(res);
//             },
//             Ok(Err(error)) => {
//                 println!("{:?}", error);
//                 return Err(Error::ParseContentFailed);
//             },
//             _ => return Err(Error::JoinTaskFailed),
//         }
//     }

//     Ok(course_info_list)
// }
