use std::fs;

use rand::seq::IteratorRandom;
use colored::*;

pub fn vaid_image(path: &str) -> String {
    match image::open(path) {
        Ok(_) => path.to_owned(),
        Err(_) => {
            eprintln!("{} {} {} {}",
                "error:".red().bold(), "Path",
                path.yellow(),
                "is not a valid image file");
            std::process::exit(1);
        }
    }
}

// TODO: check if folder is empty, is valid, exists or has other files than images
pub fn random_image(path: &str) -> String {
    let mut rng = rand::thread_rng();
    let files = fs::read_dir(path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let filepath = file.path().display().to_string();
    vaid_image(&filepath)
}
