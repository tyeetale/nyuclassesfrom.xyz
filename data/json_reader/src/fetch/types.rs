// Currently only support 5 campuses
pub enum Location {
    Shanghai,
    NewYork,
    AbuDhabi,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    BuildUrlFailed(String),
    FetchContentFailed,
    ParseContentFailed,
    JoinTaskFailed,
    CannotFlattenJson,
}