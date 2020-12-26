use clap::{App};
use crate::create::palette;

pub fn run_create(app: App) {

    let opts = app.clone().get_matches();
    let sub = opts.subcommand_matches("create").unwrap();

    if let Some(ref in_file) = sub.value_of("image") {
        println!("An input file: {}", in_file);
    }

    if let Some(arg) = sub.value_of("palette") {
        match arg.as_ref() {
            "pigment" => palette_pigment(),
            // "schemer2" => println!("schemer2"),
            _ => println!("{}", arg),
        }
    }
}

fn palette_pigment(){
    let colors = palette::colors("/home/bresilla/sets/.wallpaper/africa_feeling_feel_4k.jpg", 16);
    println!("{}", colors.join("\n"));
}
