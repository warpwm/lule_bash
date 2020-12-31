use clap;
use crate::create::generate;
use crate::create::palette;
use crate::create::write;
use crate::create::execute;
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
