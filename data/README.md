# Backend data processing

This README.md file provides a simple walkthrough for setting up the application correctly.

## Dependencies
- [Rust](https://www.rust-lang.org/learn/get-started)
- [Cargo](https://www.rust-lang.org/learn/get-started)
- [Meilisearch](https://www.meilisearch.com/)

## Important
You must create a separate file named .env under the current folder, with two variables named `DB_URL` and `DB_KEY`, where the url to the Meilisearch instance is assigned to the first variable, and the corresponding private key assigned to the second variable.

## Run
Under the current directory, type the following command to run fetch course data and upload it to Meilisearch database.

```sh
cargo run
```
