mod fetch;
use std::{fs::File};
use std::io::{Write, BufReader, BufRead};

use crate::fetch::fetch::{fetch_subjects, fetch_course_catalog, fetch_course_details};
use crate::fetch::util::*;
use crate::fetch::json_structure::*;

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

#[tokio::main]
async fn main() {
    let file = File::open("./course_info.txt").unwrap();
    let mut output = File::create("./course_flat.json").unwrap();
    let reader = BufReader::new(file);
    let year = 2022;
    let term = String::from("Fall");
    let school_name = String::from("NYU Shanghai");
    let subject_name = String::from("Bussiness and Finance");
    for line in reader.lines() {
        if let Ok(content) = line {
            let json: NestedCourseInfoFull = serde_json::from_str(&*content).unwrap();
            let res = json.flatten(year, &term, &school_name, &subject_name).unwrap();
            for info in res.iter() {
                write!(output, "{}\n", serde_json::to_string(info).unwrap()).unwrap();
            }
        }
    }
}
