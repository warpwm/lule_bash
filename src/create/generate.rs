extern crate pastel;
use colored::*;

pub fn gen_main_six(col: Vec<pastel::Color>) -> Vec<pastel::Color> {
    let mut colors = col.clone();
    colors.retain(|x| x.to_lab().l > 20.0);
    colors.retain(|x| x.to_lab().l < 90.0);

    if colors.len() < 5 {
        eprintln!("{} {}", 
            "error:".red().bold(), 
            "Not enough colors to generate a palette"); 
        std::process::exit(1);
    }
    let six = &mut colors[0..6].to_vec();
    six.sort_by_key(|c| (c.luminance() * 1000.0) as i32);
    six.reverse();
    return six.to_owned()
}

pub fn get_black_white(ac: &pastel::Color, black_mix: f64, white_mix: f64) -> (pastel::Color, pastel::Color) {
    let black = pastel::Color::from_rgb(0,0,0).mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(black_mix));
    let white = pastel::Color::from_rgb(255,255,255).mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(white_mix));
    (black, white)
}

pub fn get_all_colors(col: Vec<pastel::Color>) -> Vec<pastel::Color> {
    let mut colors: Vec<pastel::Color> = Vec::new();
    let main = gen_main_six(col.clone());
    let accent = main.get(0).unwrap().clone();
    let (col0, col15) = get_black_white(&accent, 0.10, 0.20);
    colors.push(col0);
    colors.extend(main.clone());
    colors.push(col15);

    colors
}
