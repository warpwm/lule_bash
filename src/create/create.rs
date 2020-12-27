extern crate rand;
extern crate image;
extern crate colored;

use std::{fs, env};
use std::fs::File;
use std::io::Write;

use clap;
use rand::seq::IteratorRandom;
use crate::create::palette;
use colored::*;

use crate::create::generate::*;

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
        image = vaid_image(arg);
    } else if let Some(ref arg) = sub.value_of("wallpath") {
        image = random_image(arg);
    } else {
        image = random_image(&env_lule_w);
    }

    let mut colors: Vec<pastel::Color> = Vec::new();

    if let Some(arg) = sub.value_of("palette") {
        match arg.as_ref() {
            "pigment" => palette_pigment(&image, &mut colors),
            _ => { eprintln!("{} {}", 
                "error:".red().bold(), 
                "Not a valid palette command"); 
                std::process::exit(1);
            }
        }
    }

    let colors = get_all_colors(colors.clone());

    if let Some(arg) = sub.value_of("action") {
        if arg ==  "pipe" {
            for col in colors.iter() {
                println!("{}", col.to_lab_string(pastel::Format::Spaces));
                // println!("{}", col.to_rgb_hex_string(true))
            }
        }
    }
}


fn palette_pigment(image: &str, lab: &mut Vec<pastel::Color>) {
// fn palette_pigment(image: &str) std::io::Result<()> {
    let mut dir = env::temp_dir();
    dir.push("lule_palette");
    let mut lule_palette = File::create(dir.clone()).
        unwrap_or_else(|err| {
            eprintln!("{} {} {} {} {}",
                "error:".red().bold(),
                "Could not create temp file",
                dir.as_os_str().to_str().unwrap().yellow(),
                "->", err);
            std::process::exit(1);
        });

    let colors = palette::pigments(image, 10, palette::Mood::Dominant)
        .unwrap_or_else(|err| {
            eprintln!("{} {} {}",
                "error:".red().bold(), 
                "Problem creating palette ->", 
                err);
            std::process::exit(1);
        });

    let mut record = Vec::new();

    for (color, _) in colors.iter() {
        let lab_color = pastel::Color::from_lab(
                color.l.into(),
                color.a.into(),
                color.b.into(),
                1.into());
        lab.push(lab_color.clone());
        record.push(format!("{}", lab_color.to_rgb_hex_string(true)));
    }

    lule_palette.write(record.join("\n").as_bytes()).
        unwrap_or_else(|err| {
            eprintln!("{} {} {} {} {}",
                "error:".red().bold(),
                "Could not write into",
                dir.as_os_str().to_str().unwrap().yellow(),
                "->", err);
            std::process::exit(1);
        });

}

fn vaid_image(path: &str) -> String {
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
fn random_image(path: &str) -> String {
    let mut rng = rand::thread_rng();
    let files = fs::read_dir(path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let filepath = file.path().display().to_string();
    vaid_image(&filepath)
}
