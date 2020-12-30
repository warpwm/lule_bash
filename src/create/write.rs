use crate::scheme::*;
use crate::helper::*;
use serde_json::{json, Map, Value};

pub fn write_temp_colors(output: &WRITE) {
    let mut record = Vec::new();
    for color in output.colors().iter() {
        record.push(format!("{}", color.to_rgb_hex_string(true)));
    }
    write_temp_file("lule_colors", record.join("\n").as_bytes());
    write_temp_file("lule_wallpaper", output.wallpaper().as_bytes());
    write_temp_file("lule_theme", output.theme().as_bytes());
}

pub fn write_json(output: &WRITE) {
    let mut colors = Vec::new();
    for color in output.colors() {
        colors.push(color.to_rgb_hex_string(true));
    }
    // let new_profile = Profile {
    //     wallpaper: output.wallpaper().to_string(),
    //     theme: output.theme().to_string(),
    //     special: Special {
    //         background: colors[0].clone(),
    //         foreground: colors[15].clone(),
    //         cursor: colors[1].clone(),
    //     },
    //     colors: colors
    // };
    //
    let mut map = Map::new();

    for (key, val) in colors.iter().enumerate() {
        let name = "color".to_string() + &key.to_string();
        map.insert(name, Value::String(val.to_string()));
    }

    let obj = Value::Object(map);
    let profile = json!({
        "wallpaper": output.wallpaper().to_string(),
        "theme": output.theme().to_string(),
        "special": {
            "background": colors[0].clone(),
            "foreground": colors[15].clone(),
            "cursor": colors[1].clone(),
        },
        "colors": obj
    });

    println!("{}", profile.to_string());
}
