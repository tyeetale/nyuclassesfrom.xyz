mod database;
mod fetch;
mod json;
mod types;
mod util;

use std::{collections::HashMap,};

use database::*;
use fetch::*;
use indicatif::ProgressBar;
use util::*;
// use meilisearch_sdk::{task_info::TaskInfo, errors::Error};

#[tokio::main]
async fn main() {
    println!("Enter year");
    let mut year_str = String::new();
    std::io::stdin().read_line(&mut year_str).unwrap();
    println!("Enter term (january, spring, summer, fall)");
    let mut term_str = String::new();
    std::io::stdin().read_line(&mut term_str).unwrap();

    let year = year_str.trim_end().parse::<u16>().expect("Entered invalid year");
    let term = term_str.trim_end().parse::<Season>().expect("Entered invalid season");
    let term_code = format!("{}{}", term.get_short_name(), year);
    let schools = fetch_schools(&term_code)
        .await
        .expect("Fetch school failed");
    println!("Metadata collected");

    // Define database index
    let index = String::from("course");

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
