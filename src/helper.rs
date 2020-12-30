#![allow(dead_code)]

use std::{fs, env};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;


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

pub fn write_temp_file(filename: &str, content: &[u8]) {
    let mut dir = env::temp_dir();
    dir.push(filename);
    let mut file_name = File::create(dir.clone()).
        unwrap_or_else(|err| {
            eprintln!("{} {} {} {} {}",
                "error:".red().bold(),
                "Could not create temp file",
                dir.as_os_str().to_str().unwrap().yellow(),
                "->", err);
            std::process::exit(1);
        });

    file_name.write(content).
        unwrap_or_else(|err| {
            eprintln!("{} {} {} {} {}",
                "error:".red().bold(),
                "Could not write into",
                dir.as_os_str().to_str().unwrap().yellow(),
                "->", err);
            std::process::exit(1);
        });
}

pub fn lines_to_vec(filename: &str) -> Vec<String> {
    let mut temp_file = env::temp_dir();
    temp_file.push(filename);
    // File must exist in current path before this produces output
    let mut content = Vec::new();
    if let Ok(lines) = read_lines(temp_file) {
        for line in lines {
            if let Ok(ip) = line {
                content.push(ip)
            }
        }
    }
    content
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}
