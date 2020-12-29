// extern crate serde;
// extern crate serde_json;
// extern crate serde_derive;

use std::env;
use std::fs::File;
use std::io::Write;
use colored::*;
// use serde_json::Value as jval;

pub fn write_colors(colors: Vec<pastel::Color>) {
    let mut dir = env::temp_dir();
    dir.push("lule_colors");
    let mut lule_colors = File::create(dir.clone()).
        unwrap_or_else(|err| {
            eprintln!("{} {} {} {} {}",
                "error:".red().bold(),
                "Could not create temp file",
                dir.as_os_str().to_str().unwrap().yellow(),
                "->", err);
            std::process::exit(1);
        });

    let mut record = Vec::new();
    for color in colors.iter() {
        record.push(format!("{}", color.to_rgb_hex_string(true)));
    }

    lule_colors.write(record.join("\n").as_bytes()).
        unwrap_or_else(|err| {
            eprintln!("{} {} {} {} {}",
                "error:".red().bold(),
                "Could not write into",
                dir.as_os_str().to_str().unwrap().yellow(),
                "->", err);
            std::process::exit(1);
        });

}
