mod fetch;
mod json;
mod types;
mod util;
mod redis_db;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use fetch::fetch_schools;

use fetch::{fetch_course_catalog, fetch_course_details, fetch_subjects};
use json::*;
use redis_db::create_index;
use util::*;

fn read_and_process_catalog(path: &str, line_number: u32) -> Vec<(String, String, String)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut res = Vec::new();
    for (i, line) in lines.enumerate() {
        if i >= line_number as usize {
            break;
        }
        let string = line.unwrap();
        let content = string.split("\t");
        let tmp: Vec<_> = content.into_iter().map(|x| String::from(x)).collect();
        res.push((tmp[0].clone(), tmp[1].clone(), tmp[2].clone()));
    }
    res
}

async fn fetch_courses_save_as_json(line_number: u32) {
    let catalog = read_and_process_catalog("./course_catalog.txt", line_number);
    let mut file = File::create("./course_info.txt").unwrap();

    let info = fetch_course_details(&catalog).await.unwrap();

    for i in info.into_iter() {
        let serialized_content = serde_json::to_string(&i).unwrap();
        write!(file, "{}\n", serialized_content).unwrap();
    }
}

fn main() {
    
}
