// // integration tests
// // camera integration

use colored::Colorize;
use debug_print::debug_println;
use spinners::{Spinner, Spinners};
use std::time::Instant;

use std::env;
use video_sentry::db;

use video_sentry::image_processor;
use video_sentry::trainer;
use video_sentry::ui;
use video_sentry::video_processor;

fn main() {
    let args: Vec<String> = env::args().collect();
    debug_println!("{:?}", args);

    db::init();

    if args.len() > 1 {
        match args[1].as_str() {
            "ui" => ui::ui().unwrap(),
            "cli_trainer" => trainer::cli::cli(),
            "train" => stdout_wrapper(|| {
                trainer::directory_trainer::DirectoryTrainer::new(args[2].clone()).call()
            }),
            "processor" => stdout_wrapper(|| {
                image_processor::call(&args[2]);
            }),
            "video" => stdout_wrapper(|| {
                video_processor::call(&args[2]).unwrap();
            }),
            &_ => todo!(),
        }
    }
}

fn stdout_wrapper<F: Fn()>(f: F) {
    let mut sp = Spinner::new(Spinners::Monkey, "Processing...".into());
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    sp.stop();
    println!("\r\n{} {:?}\r\n", "Done!".green().bold(), duration);
}
