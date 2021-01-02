pub mod create;
pub mod colors;
pub mod config;
pub mod daemon;
use colored::*;

use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand, AppSettings};

/////UNSAFE
fn string_to_unsafe_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn build_cli(show_logo: bool) -> App<'static, 'static> {
    let logo: String = if show_logo { "
                     ▐▓".truecolor(255, 50, 0).to_string()+"
                     ▐▓▓▓▄".truecolor(255, 50, 0).to_string().as_str()+"
                     ▓▓▓▓▓▓▓▓▄▄▄▄".truecolor(255, 50, 0).to_string().as_str()+"                      ▄▄▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
                     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▄▄".truecolor(255, 50, 0).to_string().as_str()+"               ▄▓▓▓▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
                     ▓▓▓▓▓▓▓▀ ▓▓▓▓▓▓▓▓".truecolor(255, 50, 0).to_string().as_str()+"          ▄▓▓▓▓▓▓▓▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
                     ▓▓▓▓▓▓▓▓▄▄ ▀▓▓▓▓▓▓".truecolor(255, 50, 0).to_string().as_str()+"       ▄▓▓▓▓▓▓▓▓▓▓▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
                     ▐▓▓▓▓▓▓▓▓▓▓  ▀▓▓▓▓▓".truecolor(255, 50, 0).to_string().as_str()+"    ▓▓▓▓▓▓▓▓▓▓▀▀▓▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
                      ▓▓▓▓▓▓▓▓▓▓▓▄  ▓▓▓▓".truecolor(255, 50, 0).to_string().as_str()+"   ▓▓▓▓▓▓▓▓▓▓▓ ▄▓▓▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
                       ▀▓▓▓▓▓▓▓▓▓▓▓  ▐▓▌".truecolor(255, 50, 0).to_string().as_str()+" ▐▓▓▓▓▓▓▓▓▓▓▀ ▐▓▓▓▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
             ▄▓▓▓▓▓▄▄".truecolor(160, 0, 200).to_string().as_str()+"    ▀▓▓▓▓▓▓▓▓▓▌  ▓".truecolor(255, 50, 0).to_string().as_str()+"   ▓▓▓▓▓▓▓▓▓   ▓▓▓▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
         ▄▓▒▒▒▒▒▒▒▒▒▒▒▒▓▄".truecolor(160, 0, 200).to_string().as_str()+"   ▀▓▓▓▓▓▓▓  ▓".truecolor(255, 50, 0).to_string().as_str()+"  ▐▓▓▓▓▓▓▓   ▄▓▓▓▓▓".truecolor(75, 200, 0).to_string().as_str()+"
       ▄▓▒▒▒▒▒▒▒▓▓▀▀▀▀▀▓▓▓▓▄".truecolor(160, 0, 200).to_string().as_str()+"   ▀▓▓▓▓".truecolor(255, 50, 0).to_string().as_str()+"     ▓▓▓▓▓▀   ▄▓▓▓▓▀".truecolor(75, 200, 0).to_string().as_str()+"
    ▄▓▒▒▒▒▒  ▓ ▄▄▄▄▄▄       ▀".truecolor(160, 0, 200).to_string().as_str()+"    ▀▓▓".truecolor(255, 50, 0).to_string().as_str()+"    ▓▓▓▀   ▄▓▓▀".truecolor(75, 200, 0).to_string().as_str()+"
▓▒▒▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓▓▄".truecolor(160, 0, 200).to_string().as_str()+"       ▌".truecolor(255, 50, 0).to_string().as_str()+"               ▄▓▒▒▒▒▒▒▒▒▒▓▓▄".truecolor(0, 120, 200).to_string().as_str()+"
   ▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▄".truecolor(160, 0, 200).to_string().as_str()+"          ▄▄▄▄▄▄▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▄".truecolor(0, 120, 200).to_string().as_str()+"
      ▀▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▀▀▀▀".truecolor(160, 0, 200).to_string().as_str()+"           ▀▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▄".truecolor(0, 120, 200).to_string().as_str()+"
          ▀▓▓▒▒▒▒▒▒▒▒▒▒▓▀".truecolor(160, 0, 200).to_string().as_str()+"               ▒".truecolor(200, 160, 0).to_string().as_str()+"       ▀▓▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▓▓".truecolor(0, 120, 200).to_string().as_str()+"
                         ▄▄░    ▄▓▓▓".truecolor(0, 200, 160).to_string().as_str()+"    ▒▓".truecolor(200, 160, 0).to_string().as_str()+"     ▄       ▀▀▀▀▀▀▀ ▄▒▒▒▒▒▓▀".truecolor(0, 120, 200).to_string().as_str()+"
                    ▄▓▒▒▓    ▄▓▒▒▒▓".truecolor(0, 200, 160).to_string().as_str()+"     ▒▒▒▓".truecolor(200, 160, 0).to_string().as_str()+"    ▀▓▓▓▄▄▄▄▄▓▓▓▒▒▒▒▒▒▓▀".truecolor(0, 120, 200).to_string().as_str()+"
                  ▓▒▒▒▒    ▓▒▒▒▒▒▒".truecolor(0, 200, 160).to_string().as_str()+"   ▒  ▒▒▒▒▒▓▓▄".truecolor(200, 160, 0).to_string().as_str()+"   ▀▓▒▒▒▒▒▒▒▒▒▒▒▓▀".truecolor(0, 120, 200).to_string().as_str()+"
                ▓▒▒▒▒▓   ▓▒▒▒▒▒▒▒▒".truecolor(0, 200, 160).to_string().as_str()+"   ▒  ▓▒▒▒▒▒▒▒▒▓▄".truecolor(200, 160, 0).to_string().as_str()+"    ▀▀▀▀▀▀▀".truecolor(0, 120, 200).to_string().as_str()+"
               ▓▒▒▒▒▒  ▓▒▒▒▒▒▒▒▒▒▒".truecolor(0, 200, 160).to_string().as_str()+"  ▓▒   ▒▒▒▒▒▒▒▒▒▒▓".truecolor(200, 160, 0).to_string().as_str()+"
               ▒▒▒▒▒▓ ▓▒▒▒▒▒▒▒▒▒▒▓".truecolor(0, 200, 160).to_string().as_str()+"  ▒▒▓  ▓▒▒▒▒▒▒▒▒▒▒▒".truecolor(200, 160, 0).to_string().as_str()+"
               ▓▒▒▒▓ ▄▒▒▒▒▒▒▒▒▒▓".truecolor(0, 200, 160).to_string().as_str()+"    ▒▒▒▒▓  ▀▒▒▒▒▒▒▒▒▒▒".truecolor(200, 160, 0).to_string().as_str()+"
                ▒▒▒▒▒▒▒▒▒▒▒▒▒▀".truecolor(0, 200, 160).to_string().as_str()+"      ▓▒▒▒▒▒▓▄ ▓▓▒▒▒▒▒▒▒".truecolor(200, 160, 0).to_string().as_str()+"
                ▓▒▒▒▒▒▒▒▒▒▓".truecolor(0, 200, 160).to_string().as_str()+"          ▀▒▒▒▒▒▒▒▓  ▒▒▒▒▒▒▌".truecolor(200, 160, 0).to_string().as_str()+"
                ▓▒▒▒▒▒▓▀".truecolor(0, 200, 160).to_string().as_str()+"               ▀▓▒▒▒▒▒▒▒▒▒▒▒▒▒▌".truecolor(200, 160, 0).to_string().as_str()+"
                ▒▒▓▀".truecolor(0, 200, 160).to_string().as_str()+"                       ▀▀▓▓▒▒▒▒▒▒▒".truecolor(200, 160, 0).to_string().as_str()+"
                                                  ▓▒▒▒".truecolor(200, 160, 0).to_string().as_str()+"
                                                    ▀▒".truecolor(200, 160, 0).to_string().as_str() } else { String::new() };
    App::new(crate_name!())
        .version(crate_version!())
        .before_help(string_to_unsafe_static_str(logo))
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
                .value_name("DIRPATH")
                .help("specify a dir to load color configs from")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("cache")
                .long("cache")
                .value_name("DIRPATH")
                .help("specify a dir where to dump color caches")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("temp")
                .long("temp")
                .value_name("FILEPATH")
                .help("specify a template to substitute colors")
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
                    Arg::with_name("scheme")
                        .long("scheme")
                        .value_name("NAME")
                        .help("specify a color scheme from configs to use")
                        .takes_value(true)
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
                    Arg::with_name("theme")
                        .help("specify the theme to extract from colors")
                        .long("theme")
                        .takes_value(true)
                        .value_name("THEME")
                        .possible_values(&["dark", "light"])
                        .default_value("dark")
                )
                .arg(
                    Arg::with_name("action")
                        .help("action to take")
                        .possible_values(&["set", "regen"])
                        .takes_value(true)
                        .last(true)
                )
        )
        .subcommand(
            SubCommand::with_name("daemon")
                .about("Run as deamon process with looping wallpapers")
        )
        .subcommand(
            SubCommand::with_name("colors")
                .about("Display current colors in terminal")
                .arg(
                    Arg::with_name("action")
                        .help("action to take")
                        .possible_values(&["image", "ansii", "list", "mix"])
                        .default_value("ansii")
                        .required(true)
                        .takes_value(true)
                        .last(true)
                )
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Send specific configs to pipe or daemon")
                .arg(
                    Arg::with_name("theme")
                        .help("specify the theme to extract from colors")
                        .long("theme")
                        .takes_value(true)
                        .value_name("THEME")
                        .possible_values(&["dark", "light"])
                        .default_value("dark")
                )
        )
        // .subcommand(
        //     SubCommand::with_name("picker")
        //         .about("Pick a color from the display using picker")
        // )
        .subcommand(
            SubCommand::with_name("test")
                .setting(AppSettings::Hidden)
        )
        // .subcommand(
        //     SubCommand::with_name("help")
        //         .setting(AppSettings::Hidden)
        // )
}
