use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand, AppSettings};

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

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .before_help(LOGO)
        .about(crate_description!())
        .after_help("Does really amazing things to great people...but be careful with -R")
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::AllowNegativeNumbers)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
        // .global_setting(AppSettings::DisableHelpSubcommand)
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
        .subcommand(
            SubCommand::with_name("help")
                .setting(AppSettings::Hidden)
        )
}
