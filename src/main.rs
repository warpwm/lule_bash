mod create;
mod cli;

use create::palette;

fn main() {
    let colors = palette::colors("/home/bresilla/sets/.wallpaper/africa_feeling_feel_4k.jpg", 16);
    println!("{}", colors.join("\n"));

    let app = cli::build_cli();
    let _global_matches = app.get_matches();
}
