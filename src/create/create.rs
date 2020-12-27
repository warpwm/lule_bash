extern crate rand;
extern crate image;
extern crate colored;

use std::{fs, env};
use std::fs::File;

use clap;
use rand::seq::IteratorRandom;
use crate::create::palette;
use colored::*;

pub fn run_create(app: clap::App) {

    let image: String;

    let env_lule_w: String = std::env::var("LULE_W").unwrap_or("".to_string());
    let _env_lule_s: String = std::env::var("LULE_S").unwrap_or("".to_string());

    let opts = app.clone().get_matches();
    let sub = opts.subcommand_matches("create").unwrap();

    // setting "image" source and checking is any of the flags and env variables are present for
    // to set properly the source
    if env_lule_w.is_empty() && !sub.is_present("wallpath") && !sub.is_present("image") {
        eprintln!("{} {} {} {}", "error:".red().bold(), "Environment variable", "'$LULE_W'".yellow(), "is empty");
        eprintln!("{} {} {} {}", "error:".red().bold(), "Argument option", "'--wallpath'".yellow(), "is not set");
        eprintln!("{} {} {} {}", "error:".red().bold(), "Image argument", "'--image'".yellow(), "is not given");
        eprintln!("\n{}\n\t{}\n\n{} {}", "USAGE".yellow(), "lule help <subcommands>...", 
            "For more information try", "--help".blue() );
        std::process::exit(1);
    } else if let Some(ref arg) = sub.value_of("image") {
        image = get_image(arg);
    } else if let Some(ref arg) = sub.value_of("wallpath") {
        image = random_image(arg);
    } else {
        image = random_image(&env_lule_w);
    }

    if let Some(arg) = sub.value_of("palette") {
        match arg.as_ref() {
            "pigment" => palette_pigment(&image),
            _ => println!("{}", arg),
        }
    }
}

fn palette_pigment(image: &str) {
// fn palette_pigment(image: &str) std::io::Result<()> {

    let mut dir = env::temp_dir();
    dir.push("lule_palette");
    let _f = File::create(dir);

    let colors = palette::colors(image, 16, "LAB");
    let palette = colors.join("\n");


    println!("{}", palette);
}

fn get_image(path: &str) -> String {
    let img = match image::open(path) {
        Ok(_) => path.to_owned(),
        Err(_) => {
            eprintln!("{} {} {} {}", "error:".red().bold(), "Path", path.yellow(), "is not a valid image file");
            std::process::exit(1);
        }
    };
    img
}

// TODO: check if folder is empty, is valid, exists or has other files than images
fn random_image(path: &str) -> String {
    let mut rng = rand::thread_rng();
    let files = fs::read_dir(path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let filepath = file.path().display().to_string();
    get_image(&filepath)
}
