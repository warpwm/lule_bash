use crate::scheme::*;
use crate::helper;

pub fn concatinate_envi(scheme: &mut SCHEME) {
    let env_lule_w = std::env::var("LULE_W");
    if env_lule_w.is_ok(){
        scheme.set_image(Some(helper::random_image(&env_lule_w.unwrap())));
    }
}
