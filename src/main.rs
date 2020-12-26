use clap::{App, Arg, SubCommand, AppSettings};
// use colored::*;

const LOGO: &str = "
                     ▐▓
                     ▐▓▓▓▄
                     ▓▓▓▓▓▓▓▓▄▄▄▄                      ▄▄▓▓▓
                     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▄▄               ▄▓▓▓▓▓▓
                     ▓▓▓▓▓▓▓▀ ▓▓▓▓▓▓▓▓          ▄▓▓▓▓▓▓▓▓▓▓
                     ▓▓▓▓▓▓▓▓▄▄ ▀▓▓▓▓▓▓       ▄▓▓▓▓▓▓▓▓▓▓▓▓▓
                     ▐▓▓▓▓▓▓▓▓▓▓  ▀▓▓▓▓▓    ▓▓▓▓▓▓▓▓▓▓▀▀▓▓▓▓
                      ▓▓▓▓▓▓▓▓▓▓▓▄  ▓▓▓▓   ▓▓▓▓▓▓▓▓▓▓▓ ▄▓▓▓▓▓
                       ▀▓▓▓▓▓▓▓▓▓▓▓  ▐▓▌ ▐▓▓▓▓▓▓▓▓▓▓▀ ▐▓▓▓▓▓▓
             ▄▓▓▓▓▓▄▄    ▀▓▓▓▓▓▓▓▓▓▌  ▓   ▓▓▓▓▓▓▓▓▓   ▓▓▓▓▓▓
         ▄▓▒▒▒▒▒▒▒▒▒▒▒▒▓▄   ▀▓▓▓▓▓▓▓  ▓  ▐▓▓▓▓▓▓▓   ▄▓▓▓▓▓
       ▄▓▒▒▒▒▒▒▒▓▓▀▀▀▀▀▓▓▓▓▄   ▀▓▓▓▓     ▓▓▓▓▓▀   ▄▓▓▓▓▀
    ▄▓▒▒▒▒▒  ▓ ▄▄▄▄▄▄       ▀    ▀▓▓    ▓▓▓▀   ▄▓▓▀
▓▒▒▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓▓▄       ▌               ▄▓▒▒▒▒▒▒▒▒▒▓▓▄
   ▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▄          ▄▄▄▄▄▄▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▄
      ▀▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▀▀▀▀           ▀▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▄
          ▀▓▓▒▒▒▒▒▒▒▒▒▒▓▀               ▒       ▀▓▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▓▓
                         ▄▄░    ▄▓▓▓    ▒▓     ▄       ▀▀▀▀▀▀▀ ▄▒▒▒▒▒▓▀
                    ▄▓▒▒▓    ▄▓▒▒▒▓     ▒▒▒▓    ▀▓▓▓▄▄▄▄▄▓▓▓▒▒▒▒▒▒▓▀
                  ▓▒▒▒▒    ▓▒▒▒▒▒▒   ▒  ▒▒▒▒▒▓▓▄   ▀▓▒▒▒▒▒▒▒▒▒▒▒▓▀
                ▓▒▒▒▒▓   ▓▒▒▒▒▒▒▒▒   ▒  ▓▒▒▒▒▒▒▒▒▓▄    ▀▀▀▀▀▀▀
               ▓▒▒▒▒▒  ▓▒▒▒▒▒▒▒▒▒▒  ▓▒   ▒▒▒▒▒▒▒▒▒▒▓
               ▒▒▒▒▒▓ ▓▒▒▒▒▒▒▒▒▒▒▓  ▒▒▓  ▓▒▒▒▒▒▒▒▒▒▒▒
               ▓▒▒▒▓ ▄▒▒▒▒▒▒▒▒▒▓    ▒▒▒▒▓  ▀▒▒▒▒▒▒▒▒▒▒
                ▒▒▒▒▒▒▒▒▒▒▒▒▒▀      ▓▒▒▒▒▒▓▄ ▓▓▒▒▒▒▒▒▒
                ▓▒▒▒▒▒▒▒▒▒▓          ▀▒▒▒▒▒▒▒▓  ▒▒▒▒▒▒▌
                ▓▒▒▒▒▒▓▀               ▀▓▒▒▒▒▒▒▒▒▒▒▒▒▒▌
                ▒▒▓▀                       ▀▀▓▓▒▒▒▒▒▒▒
                                                  ▓▒▒▒
                                                    ▀▒
";

fn main() {
    let matches = App::new("lule")
        .version("0.1")
        .before_help(LOGO)
        .about("a command line to set 255 colors on tty's and other places that use ANSI colors")
        .after_help("Does really amazing things to great people...but be careful with -R")
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::AllowNegativeNumbers)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("configs")
                .long("configs")
                .value_name("FILEPATH")
                .help("specify a config file where to load color preferences")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("template")
                .long("templates")
                .value_name("FILEPATH")
                .help("specify a config file where to templates are set")
                .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Generate new colors from an image")
                .arg(
                    Arg::with_name("from")
                        .help("Add document from a specific importer")
                        .short("f")
                        .long("from")
                        .takes_value(true)
                        .value_name("VALUE")
                        .possible_values(&["doi", "arxiv"])
                        .requires("source")
                )
                .arg(
                    Arg::with_name("source")
                        .help("Specify the source of the interpreter")
                        .short("s")
                        .long("source")
                        .takes_value(true)
                        .requires("from")
                        .value_name("SOURCE")
                )
                .arg(
                    Arg::with_name("input")
                        .help("the file to add")
                        .last(true)
                ),
        )
        .subcommand(
            SubCommand::with_name("theme")
                .about("Switch theme form dark to light or vice-versa")
        )
        .subcommand(
            SubCommand::with_name("colors")
                .about("Display current colors in terminal")
        )
        .get_matches();

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }

    // Continued program logic goes here...
}
