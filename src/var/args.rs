use clap;
use crate::scheme::*;
use crate::helper;

pub fn concatinate(app: &clap::ArgMatches, scheme: &mut SCHEME) {
    if let Some(ref sub) = app.subcommand_matches("create"){
        if let Some(ref arg) = sub.value_of("image") {
            scheme.set_image(Some(helper::vaid_image(arg)));
        } else if let Some(ref arg) = sub.value_of("wallpath") {
            scheme.set_image(Some(helper::random_image(arg)));
        }
        if let Some(ref arg) = sub.value_of("theme") {
            scheme.set_theme(Some(arg.to_string()));
        }
    };
    if let Some(ref sub) = app.subcommand_matches("config"){
        if let Some(ref arg) = sub.value_of("theme") {
            scheme.set_theme(Some(arg.to_string()));
        }
    };
}
