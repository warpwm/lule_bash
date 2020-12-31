mod cli;
mod var;
mod gen;
mod show;
mod scheme;
mod helper;

extern crate file;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use cli::create::*;
use cli::colors::*;
use scheme::*;


fn main() {
    let mut output = WRITE::init();
    let mut scheme = SCHEME::init();

    let app = cli::build_cli().get_matches();
    var::concatinate(&app, &mut scheme);

    match app.subcommand_name() {
        Some("create") => run_create(&app, &mut output, &mut scheme),
        Some("colors") => run_colors(&app, &mut output, &mut scheme),
        None => println!("No subcommand was used"),
        Some(_) => println!("Some other subcommand was used"),
    }
}

