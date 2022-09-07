mod fetch;
use std::fs::File;
use std::io::Write;
use crate::fetch::fetch::{fetch_subjects, fetch_course_catalog};

#[tokio::main]
async fn main() {
    if let Ok(school_subject_catalog) = fetch_subjects().await {
        let catalog = fetch_course_catalog(2022, &String::from("fa"), &school_subject_catalog).await.unwrap();
        let mut file = File::create("./course_catalog.txt").unwrap();
        for c in catalog {
            write!(&mut file, "{}\t{}\t{}\n", c.0, c.1, c.2).unwrap();
        }
    }
}
