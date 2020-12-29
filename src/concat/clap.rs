use clap;
use crate::scheme::*;
use crate::helper;

pub fn concatinate_clap(app: &clap::App, scheme: &mut SCHEME) {
    let opts = app.clone().get_matches();
    let sub = opts.subcommand_matches("create").unwrap();

    if let Some(ref arg) = sub.value_of("image") {
        scheme.set_image(Some(helper::vaid_image(arg)));
    } else if let Some(ref arg) = sub.value_of("wallpath") {
        scheme.set_image(Some(helper::random_image(arg)));
    }
}
