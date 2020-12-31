use crate::gen::palette;
use crate::gen::generate;
use crate::show::format;
use crate::show::viuwer;
use crate::scheme::*;

pub fn run_colors(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) {
    let sub = app.subcommand_matches("colors").unwrap();

    palette::colors_from_file(scheme);
    generate::get_all_colors(output, scheme);

    if let Some(arg) = sub.value_of("action") {
        // let values = write::get_json(output);
        if arg ==  "image" {
            let (cols, rows) = crossterm::terminal::size().ok().unwrap();
            viuwer::display_image(&output, (cols).into(), (rows).into()).ok();
        } else if arg ==  "ansi" {
            format::show_colors(&output);
        } else {
            format::show_pastel_colors(&output);

        }
    }


}
