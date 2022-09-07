mod fetch;
use std::{fs::File};
use std::io::{Write, BufReader, BufRead};

use futures::FutureExt;

use crate::fetch::fetch::{fetch_subjects, fetch_course_catalog, fetch_course_details};

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

async fn fetch_and_save_courses_json(line_number: u32) {
    let catalog = read_and_process_catalog("./course_catalog.txt", line_number);
    let mut file = File::create("./course_info.txt").unwrap();
    
    let info = fetch_course_details(&catalog).await.unwrap();
    
    for i in info.into_iter() {
        let serialized_content = serde_json::to_string(&i).unwrap();
        write!(file, "{}\n", serialized_content).unwrap();
    }
}

fn read_json_from_disk(path: &str) {
    
}

#[tokio::main]
async fn main() {

    fetch_and_save_courses_json(30).await;
    // if let Ok(school_subject_catalog) = fetch_subjects().await {
    //     let catalog = fetch_course_catalog(2022, &String::from("fa"), &school_subject_catalog).await.unwrap();
    //     let mut file = File::create("./course_catalog.txt").unwrap();
    //     for c in catalog {
    //         write!(&mut file, "{}\t{}\t{}\n", c.0, c.1, c.2).unwrap();
    //     }
    // }
    // let file = File::open("./course_catalog.txt").unwrap();
    // let reader = BufReader::new(file);
    // let catalog = vec!((
    //     String::from("Intermediate Korean II"),
    //     String::from("UA"),
    //     String::from("EAST")
    // ));
    // let res = fetch_course_details(&catalog).await.unwrap();
    // println!("{:?}", res);
}
