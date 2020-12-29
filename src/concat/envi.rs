use clap;
use crate::scheme::*;
use crate::helper;

use crate::concat::clap::*;


pub fn concatinate_envi(app: &clap::App, output: &mut WRITE, scheme: &mut SCHEME) {
    let env_lule_w = std::env::var("LULE_W");
    if env_lule_w.is_ok(){
        scheme.set_image(Some(helper::random_image(&env_lule_w.unwrap())));
    }
}
