use clap::{App};
use crate::create::palette;

pub fn run_create(app: App) {

    let colors = palette::colors("/home/bresilla/sets/.wallpaper/africa_feeling_feel_4k.jpg", 16);
    println!("{}", colors.join("\n"));

}
