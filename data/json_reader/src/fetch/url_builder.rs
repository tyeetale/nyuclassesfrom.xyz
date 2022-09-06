use reqwest::Url;
use crate::fetch::types::Error;

pub struct UrlBuilder {}

impl UrlBuilder {
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
    pub fn build_search_endpoint_url(course: String, school: String, subject: String) -> Result<Url, Error> {
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