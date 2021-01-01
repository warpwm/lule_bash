use crate::scheme::*;
use crate::helper::*;
use serde_json::Value;
use std::collections::HashMap as Map;
use std::path::PathBuf;
use std::env;

pub fn write_temp_colors(output: &WRITE) {
    let mut record = Vec::new();
    for color in output.colors().iter() {
        record.push(format!("{}", color.to_rgb_hex_string(true)));
    }
    write_temp_file("lule_colors", record.join("\n").as_bytes());
    write_temp_file("lule_wallpaper", output.wallpaper().as_bytes());
    write_temp_file("lule_theme", output.theme().as_bytes());
}

pub fn get_json(output: &WRITE, map: bool) -> Value {
    let mut color_map = Map::new();
    let mut color_vec = Vec::new();
    for (key, color) in output.colors().iter().enumerate() {
        let name = "color".to_string() + &key.to_string();
        color_map.insert(name, pastel::HEX::from(color).to_string());
        color_vec.push(color.to_rgb_hex_string(true));
    }
    let map_profile = ProfileMap {
        wallpaper: output.wallpaper().to_string(),
        theme: output.theme().to_string(),
        special: Special {
            background: output.colors()[0].to_rgb_hex_string(true),
            foreground: output.colors()[15].to_rgb_hex_string(true),
            cursor: output.colors()[1].to_rgb_hex_string(true),
        },
        colors: color_map
    };
    let vec_profile = ProfileVec {
        wallpaper: output.wallpaper().to_string(),
        theme: output.theme().to_string(),
        special: Special {
            background: output.colors()[0].to_rgb_hex_string(true),
            foreground: output.colors()[15].to_rgb_hex_string(true),
            cursor: output.colors()[1].to_rgb_hex_string(true),
        },
        colors: color_vec
    };
    if map {
        serde_json::to_value(&map_profile).unwrap()
    } else {
        serde_json::to_value(&vec_profile).unwrap()
    }
}

pub fn write_cache_colors(scheme: &mut SCHEME, values: Value) {
    // println!("{}", &values);
    let cache_path = match scheme.cache() {
        Some(value) => value,
        None => ""
    };
    let cache_json = pather(vec!["colors.json"], PathBuf::from(cache_path));
    let json_out = serde_json::to_string_pretty(&values).unwrap();
    write_to_file(cache_json, json_out.as_bytes());

    let mut cache_yaml = PathBuf::from(cache_path); cache_yaml.push("colors.yaml");
    let yaml = serde_yaml::from_str::<serde_yaml::Value>(&json_out).unwrap();
    let yaml_out = serde_yaml::to_string(&yaml).unwrap();
    write_to_file(cache_yaml, yaml_out.as_bytes());

    // let mut cache_toml = PathBuf::from(cache_path); cache_toml.push("colors.toml");
    // let toml = toml::from_str::<serde_yaml::Value>(&json_out).unwrap();
    // let toml_out = serde_yaml::to_string(&toml).unwrap();
    // write_to_file(cache_toml, toml_out.as_bytes());

}

pub fn copy_to_cache(scheme: &mut SCHEME) {
    let cache_path = match scheme.cache() {
        Some(value) => value,
        None => ""
    };

    let lule_colors = pather(vec!["lule_colors"], env::temp_dir());
    let colors = pather(vec!["colors"], PathBuf::from(cache_path));
    copy_to(lule_colors, colors);

    let lule_wallpaper = pather(vec!["lule_wallpaper"], env::temp_dir());
    let wallpaper = pather(vec!["wallpaper"], PathBuf::from(cache_path));
    copy_to(lule_wallpaper, wallpaper);

    let lule_theme = pather(vec!["lule_theme"], env::temp_dir());
    let theme = pather(vec!["theme"], PathBuf::from(cache_path));
    copy_to(lule_theme, theme);
}
