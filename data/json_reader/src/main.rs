mod fetch;

use crate::fetch::fetch::{fetch_subjects, fetch_courses};

#[tokio::main]
async fn main() {
    if let Ok(school_subject_catalog) = fetch_subjects().await {
        fetch_courses(2022, &String::from("fa"), &school_subject_catalog).await.unwrap();
    }
}
