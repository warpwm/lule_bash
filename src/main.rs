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

use cli::create;
use cli::colors;
use cli::test;
use scheme::*;


fn main() {
    let mut output = WRITE::init();
    let mut scheme = SCHEME::init();

    let app = cli::build_cli().get_matches();
    var::concatinate(&app, &mut scheme);

    if let Some(subcommand) = app.subcommand_name() {
        match subcommand {
            "colors" => colors::run(&app, &mut output, &mut scheme),
            "create" => create::run(&app, &mut output, &mut scheme),
            "test" => test::run(&app, &mut output, &mut scheme),
            _ => unreachable!()
        }
    }
}
