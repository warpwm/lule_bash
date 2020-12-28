use pastel;
use clap;

pub fn gen_main_six(colors: &mut Vec<pastel::Color>) -> Vec<pastel::Color> {
    colors.retain(|x| x.to_lab().l > 30.0);
    colors.retain(|x| x.to_lab().l < 90.0);

    while colors.len() < 5 {
        colors.push(colors[0].complementary());
    }

    colors.sort_by_key(|c| (c.to_lch().c) as i32);
    colors.reverse();

    let mut main_colors: Vec<pastel::Color> = Vec::new();
    for i in 0..6 {
        let new_col = colors[i].lighten(0.05).saturate(0.1);
        main_colors.push(new_col)
    }

    main_colors.sort_by_key(|c| (c.to_lch().l) as i32);
    main_colors.reverse();
    main_colors
}

pub fn get_black_white(ac: &pastel::Color, black_mix: f64, white_mix: f64, theme: bool) -> (pastel::Color, pastel::Color, pastel::Color, pastel::Color) {
    let black = pastel::Color::from_rgb(0,0,0);
    let white = pastel::Color::from_rgb(255,255,255);
    let dark = black.mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(black_mix));
    let light = white.mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(white_mix));
    if theme { (dark, light, black, white) } else { (light, dark, white, black) }
}

pub fn get_two_grays(ac: &pastel::Color, mix: f64, theme: bool) -> (pastel::Color, pastel::Color) {
    let darker = pastel::Color::from_rgb(100,100,100);
    let lighter = pastel::Color::from_rgb(170,170,170);
    let dark = darker.mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(mix));
    let light = lighter.mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(mix));
    if theme { (dark, light) } else { (light, dark) }
}

pub fn gen_second_six(colors: Vec<pastel::Color>, mix: f64, theme: bool) -> Vec<pastel::Color> {
    let mut second_colors: Vec<pastel::Color> = Vec::new();

    for col in colors.iter() {
        let new_col = if theme { col.lighten(mix) } else { col.darken(mix) };
        second_colors.push(new_col)
    }
    second_colors
}

pub fn get_all_colors(app: &clap::App, col: &mut Vec<pastel::Color>) -> Vec<pastel::Color> {


    let opts = app.clone().get_matches();
    let sub = opts.subcommand_matches("create").unwrap();

    let mut theme = true;
    if let Some(arg) = sub.value_of("theme") {
        theme = match arg.as_ref() {
            "light" => false,
            _ => true
        }
    }


    let main = gen_main_six(&mut col.clone());
    let accent = main.get(0).unwrap().clone();
    let (col0, col15, _black, _white) = get_black_white(&accent, 0.12, 0.2, theme);
    let (col7, col8) = get_two_grays(&accent, 0.2, theme);
    let second = gen_second_six(main.clone(), 0.2, theme);

    let mut colors: Vec<pastel::Color> = Vec::new();
    colors.push(col0);
    colors.extend(main.clone());
    colors.push(col7);
    colors.push(col8);
    colors.extend(second);
    colors.push(col15);

    colors
}
