use crate::json::FlatCourseInfo;
use meilisearch_sdk::Client;
use meilisearch_sdk::task_info::TaskInfo;
use meilisearch_sdk::errors::Error;
use futures::executor::block_on;

pub(crate) fn connect_database(url: &str, key: &str) -> Client {
    // No error handling to this function
    let cli = Client::new(url, key);
    cli
}

// This function inserts a flattened course info into the database
pub(crate) fn insert_course(
    course: &Vec<FlatCourseInfo>,
    cli: &mut Client,
) -> Result<TaskInfo, Error> {
    // block on uploading the file
    // would be great if we could do this in parallel to receiving documents
    // so that we can fully utilize the duplex bandwidth
    block_on(cli.index("course").add_documents(course, None))
}

#[cfg(test)]
#[allow(dead_code, unused_variables)]
mod test {
    use crate::database::*;
    use crate::util::{read_env_variables};
    use std::{fs::File, io::{BufRead, BufReader}};
    use futures::executor::block_on;

    #[test]
    fn test_conn_db() {
        // env file
        // let (url, password) = read_env_variables();
        let (url, key) = read_env_variables();
        let cli = connect_database(&*url, &*key);
    }

    #[test]
    fn test_insert_record() {
        // read and deserialize course info stored in json
        let (url, key) = read_env_variables();
        let cli = meilisearch_sdk::Client::new(url, key);
        // create_index(&mut con).expect("Failed to create schema");
        let file = File::open("./cached/course_flat.json").unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();
        // invoke function to insert json
        let mut courses = Vec::new();
        for (_, line) in lines.enumerate() {
            let obj = serde_json::from_str::<FlatCourseInfo>(&line.unwrap()).unwrap();
            // insert_course(ctr, &obj, &mut con).expect("Error inserting course");
            courses.push(obj);
        }
        block_on(cli.index("course").add_documents(&courses, None)).unwrap();
    }

    #[test]
    fn test_delete_all_docs() {
        let (url, key) = read_env_variables();
        let cli = connect_database(&*url, &*key);
        block_on(cli.index("course").delete_all_documents()).unwrap();
    }
}
