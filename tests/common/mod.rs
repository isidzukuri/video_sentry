use std::env;
use std::fs;
use video_sentry;

// TODO: create db for each thread
pub fn setup() {
    env::set_var("DATABASE_URL", "./tmp/test/storage/db/database.sql");
    env::set_var("DATABASE_FOLDER", "./tmp/test/storage/db/");
    video_sentry::db::init();
}

// TODO: remove tmp folder after all tests
pub fn cleanup() {
    fs::remove_dir_all("./tmp/").unwrap();
}

pub fn pretrain(){
    video_sentry::trainer::directory_trainer::DirectoryTrainer::new("tests/fixtures/trainer".to_string()).call();
}