mod cli;
mod create;
mod display;
mod concat;
mod scheme;
mod helper;

use create::create::run_create;
use crate::scheme::*;


fn main() {
    let _env_lule_c: String = std::env::var("LULE_C").unwrap_or("".to_string());

    let mut output = WRITE::init();
    let mut scheme = SCHEME::init();

    let app = cli::build_cli();

    match app.clone().get_matches().subcommand_name() {
        Some("create") => run_create(&app.clone(), &mut output, &mut scheme),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

}
