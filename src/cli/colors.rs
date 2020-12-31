use crate::gen::palette;
use crate::gen::generate;
use crate::show::format;
use crate::show::viuwer;
use crate::scheme::*;
use std::env;

pub fn run_colors(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) {
    let sub = app.subcommand_matches("colors").unwrap();

    let mut color_temp = env::temp_dir();
    color_temp.push("lule_palette");
    palette::colors_from_file(color_temp, scheme);
    generate::get_all_colors(output, scheme);


    let mut wall_temp = env::temp_dir();
    wall_temp.push("lule_wallpaper");


    output.set_wallpaper(scheme.image().clone().unwrap());
    output.set_theme(scheme.theme().clone().unwrap());

    let (cols, rows) = crossterm::terminal::size().ok().unwrap();
    if let Some(arg) = sub.value_of("action") {
        // let values = write::get_json(output);
        if arg ==  "image" {
            viuwer::display_image(&output, (cols).into(), (rows -1).into()).ok();
        } else if arg ==  "ansii" {
            format::show_colors(&output, 0..output.colors().len(), 1);
        } else if arg ==  "list" {
            format::show_pastel_colors(&output, 0..output.colors().len());
        } else if arg ==  "mix" {
            viuwer::display_image(&output, (cols).into(), (rows -3).into()).ok();
            println!("Main colors:");
            format::show_colors(&output, 0..16, ((cols - 56) / 16).into());

        }
    }
}
