use crate::gen::generate;
use crate::gen::palette;
use crate::gen::write;
use crate::gen::execute;
use crate::scheme::*;

pub fn run_create(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) {
    let sub = app.subcommand_matches("create").unwrap();

    if let Some(arg) = sub.value_of("palette") {
        match arg.as_ref() {
            "pigment" => palette::palette_pigment(scheme),
            _ => palette::colors_from_file(scheme),
        }
    }

    generate::get_all_colors(output, scheme);
    output.set_theme(scheme.theme().clone().unwrap());
    output.set_wallpaper(scheme.image().clone().unwrap());

    if let Some(arg) = sub.value_of("action") {
        let values = write::get_json(output);
        if arg ==  "pipe" {
            write::write_temp_colors(&output);
            println!("{}", &values);

        }
        if arg ==  "set" {
            write::write_temp_colors(&output);
            write::write_cache_colors(scheme, values);
            write::copy_to_cache(scheme);
            execute::external_command();
        }
    }
}
