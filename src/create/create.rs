extern crate rand;
extern crate image;
extern crate colored;

use std::env;
use std::fs::File;
use std::io::Write;

use clap;
use crate::create::palette;
use colored::*;

use crate::create::generate::*;
use crate::create::write::*;
use crate::create::execute::*;
use crate::display::colors::*;
use crate::concat::concat::*;
use crate::scheme::*;

pub fn run_create(app: &clap::App, output: &mut WRITE, scheme: &mut SCHEME) {

    concatinate(app, scheme);

    let opts = app.clone().get_matches();
    let sub = opts.subcommand_matches("create").unwrap();

    output.set_wallpaper(scheme.image().clone().unwrap());

    let mut colors: Vec<pastel::Color> = Vec::new();

    if let Some(arg) = sub.value_of("palette") {
        match arg.as_ref() {
            "pigment" => palette_pigment(output.wallpaper(), &mut colors),
            _ => { eprintln!("{} {}", 
                "error:".red().bold(), 
                "Not a valid palette command"); 
                std::process::exit(1);
            }
        }
    }

    let colors = get_all_colors(&app, &mut colors.clone());

    output.set_theme("dark".to_string());
    output.set_colors(colors);


    if let Some(arg) = sub.value_of("action") {
        if arg ==  "pipe" {
            // write_colors(&output);
            // external_command(output);
            show_colors(&output);
        }
        if arg ==  "set" {
            write_colors(&output);
            external_command();
        }
    }

}


fn palette_pigment(image: &str, lab: &mut Vec<pastel::Color>) {
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

    let colors = palette::pigments(image, 16, Some(300))
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
