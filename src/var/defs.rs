use std::path::PathBuf;
use crate::scheme::*;
use colored::*;
use dirs;

pub fn concatinate(scheme: &mut SCHEME) {
    let _home_path: PathBuf = dirs::home_dir().expect(
        &format!("{} {}", "error:".red().bold(), "Path of home is impossible to get"));

    let config_path: PathBuf = dirs::config_dir().expect(
        &format!("{} {}", "error:".red().bold(), "Path for configs is impossible to get"));
    let mut lule_configs = config_path.clone();
    lule_configs.push("lule");

    let cache_path: PathBuf = dirs::cache_dir().expect(
        &format!("{} {}", "error:".red().bold(), "Path for configs is impossible to get"));
    let mut lule_cache = cache_path.clone();
    lule_cache.push("lule");



    scheme.set_theme(Some("dark".to_string()));
    scheme.set_config(Some(lule_configs.to_str().unwrap().to_string()));
    scheme.set_cache(Some(lule_cache.to_str().unwrap().to_string()));
    scheme.set_palette(Some("pigment".to_string()));
}
