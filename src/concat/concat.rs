use clap;
use crate::scheme::*;
use colored::*;

use crate::concat::clap::*;
use crate::concat::envi::*;

pub fn concatinate(app: &clap::App, output: &mut WRITE, scheme: &mut SCHEME) {
    let opts = app.clone().get_matches();
    let sub = opts.subcommand_matches("create").unwrap();

    concatinate_envi(app, output, scheme);
    concatinate_clap(app, output, scheme);

    if scheme.image().is_none() {
        eprintln!("{} {} {} {}", "error:".red().bold(), "Environment variable", "'$LULE_W'".yellow(), "is empty");
        eprintln!("{} {} {} {}", "error:".red().bold(), "Argument option", "'--wallpath'".yellow(), "is not set");
        eprintln!("{} {} {} {}", "error:".red().bold(), "Image argument", "'--image'".yellow(), "is not given");
        eprintln!("\n{}\n\t{}\n\n{} {}", "USAGE".yellow(), "lule help <subcommands>...", 
            "For more information try", "--help".blue() );
        std::process::exit(1);
    }


}
