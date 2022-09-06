use std::collections::HashMap;

use futures::future::join_all;

use crate::fetch::json_structure::{CourseInfo, SubjectName};
use crate::fetch::url_builder::UrlBuilder;
use crate::fetch::types::Error;

pub async fn fetch_from_api(year: u32, semester: String) -> Result<(), Error> {
    // first step, fetch the course and school info 
    let url = UrlBuilder::build_subjects_endpoint_url()?;

    let res = match reqwest::get(url).await {
        Ok(res) => {
            match res.json::<HashMap<String, HashMap<String, SubjectName>>>().await {
                Ok(map) => map,
                Err(_) => return Err(Error::DecodeContentFailed),
            }
        },
        Err(_) => return Err(Error::FetchContentFailed),
    };

    // second step, iterate through all schools and their corresponding subjects
    // send get requests to the api
    let mut futures = Vec::new();
    for (school, subject_list) in &res {
        for (subject, _) in subject_list {
            let url = UrlBuilder::build_courses_endpoint_url(year, &semester, school, subject)?;
            futures.push(reqwest::get(url));
        }
    }

    println!("{:?}", join_all(futures.into_iter()).await);

    Ok(())
}