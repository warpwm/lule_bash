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
        // .after_help("Does really amazing things to great people...but be careful with -R")
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        // .global_setting(AppSettings::NextLineHelp)
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
        .arg(
            Arg::with_name("scheme")
                .long("scheme")
                .value_name("FILEPATH")
                .help("specify a color scheme from configs to use")
                .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Generate new colors from an image")
                .arg(
                    Arg::with_name("wallpath")
                        .help("specify a folder to pick an image randomly")
                        .long("wallpath")
                        .visible_aliases(&["path"])
                        .takes_value(true)
                        .value_name("DIRPATH")
                        .conflicts_with("image")
                )
                .arg(
                    Arg::with_name("palette")
                        .help("specify a palete generator for colors")
                        .long("palette")
                        .takes_value(true)
                        .possible_values(&["schemer2", "pigment"])
                        .default_value("pigment")
                        .value_name("NAME")
                )
                .arg(
                    Arg::with_name("image")
                        .help("specify the image to extract colors from")
                        .long("image")
                        .visible_aliases(&["source"])
                        .takes_value(true)
                        .value_name("FLEPATH")
                        .conflicts_with("wallpath")
                )
                .arg(
                    Arg::with_name("action")
                        .help("action to take")
                        .possible_values(&["set", "regen", "pipe"])
                        .default_value("pipe")
                        .required(true)
                        .takes_value(true)
                        .last(true)
                )
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
            SubCommand::with_name("picker")
                .about("Pick a color from the display using picker")
        )
        .subcommand(
            SubCommand::with_name("help")
                .setting(AppSettings::Hidden)
        )
}
