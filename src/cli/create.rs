use crate::gen::generate;
use crate::gen::palette;
use crate::gen::write;
use crate::gen::execute;
use crate::var;
use crate::scheme::*;

pub fn run(app: &clap::ArgMatches, output: &mut WRITE, scheme: &mut SCHEME) {
    let sub = app.subcommand_matches("create").unwrap();
    var::defs::concatinate(scheme);
    var::envi::concatinate(scheme);
    var::args::concatinate(app, scheme);
    var::pipe::concatinate(scheme);

    if let Some(arg) = sub.value_of("palette") {
        match arg.as_ref() {
            "pigment" => palette::palette_pigment(scheme),
            _ => println!("something went crazy wrong"),
        }
    }

    generate::get_all_colors(output, scheme);
    output.set_theme(scheme.theme().clone().unwrap());
    output.set_wallpaper(scheme.image().clone().unwrap());

    let values = write::get_json(output, false);
    if atty::isnt(atty::Stream::Stdout) {
        write::write_temp_colors(&output);
        println!("{}", &values);
    } else {
        if let Some(arg) = sub.value_of("action") {
            if arg ==  "json" {
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
}
