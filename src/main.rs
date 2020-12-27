mod create;
mod cli;

use create::create::run_create;


fn main() {
    // let LULE_W = std::env::var("LULE_W").ok();
    //
    let _env_lule_c: String = std::env::var("LULE_C").unwrap_or("".to_string());

    let app = cli::build_cli();
    match app.clone().get_matches().subcommand_name() {
        Some("create") => run_create(app.clone()),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

}
