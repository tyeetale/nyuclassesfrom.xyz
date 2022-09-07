#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    BuildUrlFailed(String),
    FetchContentFailed,
    ParseContentFailed,
    JoinTaskFailed,
}