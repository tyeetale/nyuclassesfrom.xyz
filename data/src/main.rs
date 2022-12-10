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
            // insert_course(&flattened, &mut con).expect("Failed to insert course");
            bar.inc(1);
        }

        school_ctr += 1;
    }

    // for (school_code, subject_codes) in &subjects {
    //     let mut tmp = Vec::new();
    //     let school_name = match schools.get(school_code) {
    //         None => {
    //             println!("No such school: {}", school_code);
    //             school_code
    //         },
    //         Some(name) => &name.name,
    //     };

    //     for (subject_code, _) in subject_codes {
    //         tmp.push((school_code, subject_code));
    //     }
    //     // Collect information for all subjects in the same school
    //     println!(
    //         "[{}/{}] Collecting courses for {}",
    //         school_ctr,
    //         subjects.len(),
    //         school_name
    //     );

    //     let catalog = fetch_course_catalog(year, &semester, &tmp)
    //         .await
    //         .expect("Failed to fetch course catalog");

    //     // To prevent timeout
    //     // we set a window of some size to limit requests number per iteration
    //     let mut step = catalog.len() / window;
    //     if catalog.len() % window != 0 {
    //         step += 1;
    //     }

    //     let mut course_details = Vec::new();
    //     let bar = ProgressBar::new(catalog.len() as u64);

    //     // Collect detailed course info
    //     for i in 0..step {
    //         let left = i * window;
    //         let right = std::cmp::min(left + window, catalog.len());
    //         course_details.extend(
    //             fetch_course_details(&catalog, left, right)
    //                 .await
    //                 .expect("Failed to fetch course details"),
    //         );
    //         bar.inc((right - left) as u64);
    //     }

    //     bar.finish();

    //     // Insert into database
    //     println!("[{}/{}] Uploading to database", school_ctr, subjects.len());
    //     let bar = ProgressBar::new(course_details.len() as u64);
    //     for course_detail in &course_details {
    //         let subject_name = &subjects
    //             .get(&course_detail.subjectCode.school)
    //             .expect("School does not exist")
    //             .get(&course_detail.subjectCode.code)
    //             .expect(&*format!("Subject {} does not exist in {}", &course_detail.subjectCode.code, school_code))
    //             .name;
    //         // Cleaning data
    //         let flattened = flatten(&mut class_ctr, school_name, subject_name, year, &semester, course_detail)
    //             .expect("Failed to flatten nested course");
    //         // insert all records into database
    //         insert_course(&flattened, &mut con).expect("Failed to insert course into database");
    //         bar.inc(1);
    //     }
    //     school_ctr += 1;
    //     bar.finish();
    // }
}
