mod database;
mod fetch;
mod json;
mod types;
mod util;

use std::collections::HashMap;

use database::*;
use fetch::*;
use indicatif::ProgressBar;
use util::*;
// use meilisearch_sdk::{task_info::TaskInfo, errors::Error};

#[tokio::main]
async fn main() {
    // We first fetch school and subject info
    let term = Season::Spring;
    let year = 2023;

    let term_code = get_term_str(&term, year);
    let mut index = String::from("course-");
    index.push_str(&term_code);

    let schools = fetch_schools(&term_code)
        .await
        .expect("Fetch school failed");
    println!("Metadata collected");

    let mut school_ctr = 1;
    let mut class_ctr = 0;

    // Initialize db connection
    let (url, key) = read_env_variables();
    let mut con = connect_database(&*url, &*key);
    println!("Connected to database");

    for school_info in &schools.schools {
        println!(
            "[{}/{}] Collecting courses for {}",
            school_ctr,
            schools.schools.len(),
            &school_info.name
        );
        // translate the code name map
        let code_name_map: HashMap<_, _> = school_info
            .subjects
            .iter()
            .map(|x| (&x.code, &x.name))
            .collect();

        let courses = fetch_courses(&term, year, &code_name_map)
            .await
            .expect("Failed to fetch courses");

        // We then flatten and clean the data
        let bar = ProgressBar::new(courses.len() as u64);
        println!("[{}/{}] Uploading courses to the database", school_ctr, schools.schools.len());

        for course in &courses {
            let flattened = flatten(
                &mut class_ctr,
                &school_info.name,
                code_name_map.get(&course.subjectCode).unwrap(),
                year,
                &term.to_string(),
                course,
            )
            .expect("Failed to flatten string");
            insert_course(&flattened, &index, &mut con).expect("Failed to insert course");
            bar.inc(1);
        }
        bar.finish();

        school_ctr += 1;
    }
    println!("Completed");
    println!("Uploaded {} courses in total", class_ctr);
}
