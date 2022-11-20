use crate::json::FlatCourseInfo;
use redis::*;
use serde_json;

pub(crate) fn connect_redis(addr: &str) -> Result<Connection, RedisError> {
    let cli = redis::Client::open(format!("redis://{}", addr))?;
    let con = cli.get_connection()?;
    Ok(con)
}

/*
Creates an index for searching based on the schema created
We could add or delete fields from below
We can also specify weight for each field
*/
pub(crate) fn create_index(con: &mut Connection) -> Result<(), RedisError> {
    // replace this schema with json
    redis::cmd("FT.CREATE")
        .arg("idx:courses")
        .arg("ON")
        .arg("JSON")
        .arg("PREFIX")
        .arg("1")
        .arg("course:")
        .arg("SCHEMA")
        .arg("$.class_name") // field 1
        .arg("AS")
        .arg("class_name")
        .arg("TEXT")
        .arg("$.school_name") // field 2
        .arg("AS")
        .arg("school_name")
        .arg("TEXT")
        .arg("$.subject_number") // field 3
        .arg("AS")
        .arg("subject_number")
        .arg("TEXT")
        .arg("$.description") // field 4
        .arg("AS")
        .arg("description")
        .arg("TEXT")
        .arg("$.instructors[0]") // field 5
        .arg("AS")
        .arg("instructor")
        .arg("TEXT")
        .arg("$.instructors[1]")
        .arg("AS")
        .arg("instructor2")
        .arg("TEXT")
        .arg("$.instructors[2]")
        .arg("AS")
        .arg("instructor3")
        .arg("TEXT")
        // we could store all instructors as a string, separated by backslash
        .query(con)
}

pub(crate) fn drop_index(con: &mut Connection) -> Result<(), RedisError> {
    redis::cmd("FT.DROPINDEX")
        .arg("idx:courses")
        .query(con)
}

// This function inserts a flattened course info into redisDB
pub(crate) fn insert_course(
    class_counter: u32,
    course: &FlatCourseInfo,
    con: &mut Connection,
) -> Result<(), RedisError> {
    redis::cmd("JSON.SET")
        .arg(format!("course:{}", class_counter))
        .arg("$")
        .arg(serde_json::to_string(course).unwrap())
        .query(con)
}

#[cfg(test)]
mod test {
    use crate::redis_db::*;
    use std::{fs::File, io::{BufRead, BufReader}};
    #[test]
    fn test_create_schema() {
        let mut con = connect_redis("127.0.0.1").expect("Failed to connect to redis server");
        create_index(&mut con).expect("Failed to create schema");
    }

    #[test]
    fn test_insert_record() {
        // read and deserialize course info stored in json
        let mut con = connect_redis("127.0.0.1").expect("Failed creating connection");
        drop_index(&mut con);
        create_index(&mut con).expect("Failed to create schema");
        let file = File::open("./json/course_flat.json").unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();
        // invoke function to insert json
        let mut ctr: u32 = 0;
        for (_, line) in lines.enumerate() {
            let obj = serde_json::from_str::<FlatCourseInfo>(&line.unwrap()).unwrap();
            insert_course(ctr, &obj, &mut con).expect("Error inserting course");
            ctr += 1;
        }
    }
}
