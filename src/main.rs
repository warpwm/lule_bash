mod create;
mod cli;

use create::create::*;


fn main() {
    // let LULE_W = std::env::var("LULE_W").ok();

    let app = cli::build_cli();
    let opts = app.clone().get_matches();

    match opts.subcommand_name() {
        Some("create") => run_create(app.clone()),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

}
