// Currently only support 5 campuses
#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    BuildUrlFailed(String),
    FetchContentFailed,
    ParseContentFailed,
    JoinTaskFailed,
    CannotFlattenJson,
    RedisError,
}