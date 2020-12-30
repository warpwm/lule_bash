// extern crate serde;
// extern crate serde_json;
// extern crate serde_derive;
// use serde_json::Value as jval;

use crate::scheme::*;
use crate::helper::*;

pub fn write_colors(output: &WRITE) {
    let mut record = Vec::new();
    for color in output.colors().iter() {
        record.push(format!("{}", color.to_rgb_hex_string(true)));
    }
    write_temp_file("lule_colors", record.join("\n").as_bytes());


    write_temp_file("lule_wallpaper", output.wallpaper().as_bytes());
}
