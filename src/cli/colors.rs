use clap;
use crate::gen::palette;
use crate::gen::generate;
use crate::show::format;
use crate::scheme::*;

pub fn run_colors(_app: &clap::App, output: &mut WRITE, scheme: &mut SCHEME) {

    // concat::concatinate(app, scheme);

    // let opts = app.clone().get_matches();
    // let sub = opts.subcommand_matches("colors").unwrap();



    palette::colors_from_file(scheme);
    generate::get_all_colors(output, scheme);
    format::show_pastel_colors(&output);
}
