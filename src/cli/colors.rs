use clap;
use crate::scheme::*;
use crate::create::palette;
use crate::display::colors;
use crate::create::generate;

pub fn run_colors(_app: &clap::App, output: &mut WRITE, scheme: &mut SCHEME) {

    // concat::concatinate(app, scheme);

    // let opts = app.clone().get_matches();
    // let sub = opts.subcommand_matches("colors").unwrap();



    palette::colors_from_file(scheme);
    generate::get_all_colors(output, scheme);
    colors::show_pastel_colors(&output);
}
