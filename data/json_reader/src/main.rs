mod fetch;

use crate::fetch::fetch::fetch_from_api;

#[tokio::main]
async fn main() {
    fetch_from_api(2022, String::from("fa")).await.unwrap();
}
