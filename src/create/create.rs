extern crate rand;
extern crate image;
extern crate colored;

use rand::seq::IteratorRandom;
use std::fs;
use clap::{App};
use crate::create::palette;
use colored::*;

pub fn run_create(app: App) {

    let mut image: String;

    let mut env_lule_w: String = std::env::var("LULE_W").unwrap_or("".to_string());
    let _env_lule_s: String = std::env::var("LULE_S").unwrap_or("".to_string());

    let opts = app.clone().get_matches();
    let sub = opts.subcommand_matches("create").unwrap();


    if let Some(ref arg) = sub.value_of("wallpath") {
        env_lule_w = arg.to_string()
    }

    image = random_image(&env_lule_w);

    if let Some(ref arg) = sub.value_of("image") {
        image = get_image(arg);
    }

    if let Some(arg) = sub.value_of("palette") {
        match arg.as_ref() {
            "pigment" => palette_pigment(&image),
            // "schemer2" => println!("schemer2"),
            _ => println!("{}", arg),
        }
    }
}

fn palette_pigment(image: &str){
    let colors = palette::colors(image, 16);
    println!("{}", colors.join("\n"));
}

fn get_image(path: &str) -> String {
    let img = match image::open(path) {
        Ok(_) => path.to_owned(),
        Err(_) => {
            println!("{}{}{}", "path: ".red().bold(), path.blue(), " is not a directory".red().bold());
            std::process::exit(1);
        }
    };
    img
}

// TODO: check if folder is empty or has other files than images
fn random_image(path: &str) -> String {
    let mut rng = rand::thread_rng();
    let files = fs::read_dir(path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let filepath = file.path().display().to_string();
    get_image(&filepath)
}
