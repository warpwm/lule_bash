use clap;
use crate::create::generate;
use crate::create::palette;
use crate::create::write;
use crate::create::execute;
use crate::display::colors;
use crate::scheme::*;
use crate::concat;

pub fn run_create(app: &clap::App, output: &mut WRITE, scheme: &mut SCHEME) {

    concat::concatinate(app, scheme);

    let opts = app.clone().get_matches();
    let sub = opts.subcommand_matches("create").unwrap();


    if let Some(arg) = sub.value_of("palette") {
        match arg.as_ref() {
            "pigment" => palette::palette_pigment(scheme),
            _ => palette::colors_from_file(scheme),
        }
    }

    generate::get_all_colors(output, scheme);

    output.set_wallpaper(scheme.image().clone().unwrap());

    if let Some(arg) = sub.value_of("action") {
        if arg ==  "pipe" {
            colors::show_colors(&output);
        }
        if arg ==  "set" {
            write::write_colors(&output);
            execute::external_command();
        }
    }
}
