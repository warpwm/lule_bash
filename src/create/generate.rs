use pastel;
use clap;

pub fn gen_main_six(colors: &mut Vec<pastel::Color>) -> Vec<pastel::Color> {
    colors.retain(|x| x.to_lab().l > 30.0);
    colors.retain(|x| x.to_lab().l < 90.0);

    while colors.len() < 6 {
        colors.push(colors[0].complementary());
    }

    colors.sort_by_key(|c| (c.to_lch().c) as i32);
    colors.reverse();

    let mut main_colors: Vec<pastel::Color> = Vec::new();
    for i in 0..6 {
        main_colors.push(colors[i].saturate(0.15))
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

pub fn gen_prime_six(colors: Vec<pastel::Color>, mix: f64, theme: bool) -> Vec<pastel::Color> {
    let mut second_colors: Vec<pastel::Color> = Vec::new();

    for col in colors.iter() {
        let new_col = if theme { col.lighten(mix) } else { col.darken(mix) };
        second_colors.push(new_col)
    }
    second_colors
}

pub fn gen_second_six(colors: Vec<pastel::Color>, mix: f64, theme: bool) -> Vec<pastel::Color> {
    let mut second_colors: Vec<pastel::Color> = Vec::new();

    for col in colors.iter() {
        let new_col = if ! theme { col.lighten(mix) } else { col.darken(mix) };
        second_colors.push(new_col)
    }
    second_colors
}

pub fn gen_gradients(ac: pastel::Color, col0: pastel::Color, col15: pastel::Color, black: pastel::Color, white: pastel::Color) -> Vec<pastel::Color> {
    let mut color_scale = pastel::ColorScale::empty();
    let mut gradients: Vec<pastel::Color> = Vec::new();
    let colors = vec![black, col0, ac, col15, white];

    for color in colors.iter().enumerate() {
        let position = pastel::Fraction::from(color.0 as f64 / (colors.len() as f64 - 1.0));
        color_scale.add_stop(color.1.clone(), position);
    }

    let mix = Box::new(|c1: &pastel::Color, c2: &pastel::Color, f: pastel::Fraction| c1.mix::<pastel::Lab>(c2, f));
    let count = 24;
    for i in 0..count {
        let position = pastel::Fraction::from(i as f64 / (count as f64 - 1.0));
        let color = color_scale.sample(position, &mix).expect("gradient color");
        gradients.push(color)
    }
    gradients
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


    let prime = gen_prime_six(main.clone(), 0.1, theme);
    let acc = prime.get(0).unwrap().clone();
    let (col0, col15, black, white) = get_black_white(&acc, 0.12, 0.2, theme);
    let (col7, col8) = get_two_grays(&acc, 0.2, theme);
    let second = gen_second_six(main.clone(), 0.1, theme);

    let gradients = gen_gradients(acc.clone(), col0.clone(), col15.clone(), black, white);

    let mut colors: Vec<pastel::Color> = Vec::new();
    colors.push(col0);
    colors.extend(prime);
    colors.push(col7);
    colors.push(col8);
    colors.extend(second);
    colors.push(col15);
    colors.extend(gradients);

    colors
}
