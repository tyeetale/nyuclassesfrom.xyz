mod fetch;
mod json;
mod database;
mod types;
mod util;

use fetch::*;
use indicatif::ProgressBar;
use database::*;
use util::*;
// use meilisearch_sdk::{task_info::TaskInfo, errors::Error};

#[tokio::main]
async fn main() {
    // We first fetch school and subject info
    let schools = fetch_schools().await.expect("Fetch school failed");
    let subjects = fetch_subjects().await.expect("Fetch subjects failed");
    // maybe we need to flatten the subjects map
    println!("Metadata collected");

    let window = 50;
    let year: u16 = 2022;
    let semester = String::from("Fall");

    let mut school_ctr = 1;
    let mut class_ctr = 0;

    // Initialize redis connection
    let (url, key) = read_env_variables();
    let mut con = connect_database(&*url, &*key);
    println!("Connected to database");

    for (school_code, subject_codes) in &subjects {
        let mut tmp = Vec::new();
        let school_name = match schools.get(school_code) {
            None => {
                println!("No such school: {}", school_code);
                school_code
            },
            Some(name) => &name.name,
        };

        for (subject_code, _) in subject_codes {
            tmp.push((school_code, subject_code));
        }
        // Collect information for all subjects in the same school
        println!(
            "[{}/{}] Collecting courses for {}",
            school_ctr,
            subjects.len(),
            school_name
        );

        let catalog = fetch_course_catalog(year, &semester, &tmp)
            .await
            .expect("Failed to fetch course catalog");

        // To prevent timeout
        // we set a window of some size to limit requests number per iteration
        let mut step = catalog.len() / window;
        if catalog.len() % window != 0 {
            step += 1;
        }

        let mut course_details = Vec::new();
        let bar = ProgressBar::new(catalog.len() as u64);

        // Collect detailed course info
        for i in 0..step {
            let left = i * window;
            let right = std::cmp::min(left + window, catalog.len());
            course_details.extend(
                fetch_course_details(&catalog, left, right)
                    .await
                    .expect("Failed to fetch course details"),
            );
            bar.inc((right - left) as u64);
        }

        bar.finish();

        // Insert into database
        println!("[{}/{}] Uploading to database", school_ctr, subjects.len());
        let bar = ProgressBar::new(course_details.len() as u64);
        for course_detail in &course_details {
            let subject_name = &subjects
                .get(&course_detail.subjectCode.school)
                .expect("School does not exist")
                .get(&course_detail.subjectCode.code)
                .expect(&*format!("Subject {} does not exist in {}", &course_detail.subjectCode.code, school_code))
                .name;
            // Cleaning data
            let flattened = flatten(&mut class_ctr, school_name, subject_name, year, &semester, course_detail)
                .expect("Failed to flatten nested course");
            // insert all records into database
            insert_course(&flattened, &mut con).expect("Failed to insert course into database");
            bar.inc(1);
        }
        school_ctr += 1;
        bar.finish();
    }
}
