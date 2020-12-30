mod cli;
mod create;
mod display;
mod concat;
mod scheme;
mod helper;

use cli::create::*;
use cli::colors::*;
use scheme::*;


fn main() {
    let mut output = WRITE::init();
    let mut scheme = SCHEME::init();

    let app = cli::build_cli();

    match app.clone().get_matches().subcommand_name() {
        Some("create") => run_create(&app.clone(), &mut output, &mut scheme),
        Some("colors") => run_colors(&app.clone(), &mut output, &mut scheme),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

}

