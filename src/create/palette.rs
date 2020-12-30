use colored::*;
use crate::create::kmeans;
use crate::helper::*;
use crate::scheme::*;

pub fn palette_pigment(scheme: &mut SCHEME) {
    let image = Some(scheme.image()).unwrap().clone().unwrap();
    let colors_lab = kmeans::pigments(&image, 16, Some(300))
        .unwrap_or_else(|err| {
            eprintln!("{} {} {}",
                "error:".red().bold(), 
                "Problem creating palette ->", 
                err);
            std::process::exit(1);
        });

    let mut record = Vec::new();
    let mut colors = Vec::new();
    for (color, _) in colors_lab.iter() {
        let lab_color = pastel::Color::from_lab(
                color.l.into(),
                color.a.into(),
                color.b.into(),
                1.into());
        colors.push(pastel::Color::from(lab_color.clone()));
        record.push(format!("{}", lab_color.to_rgb_hex_string(true)));
    }
    write_temp_file("lule_palette", record.join("\n").as_bytes());
    scheme.set_colors(Some(colors));
}

pub fn colors_from_file(scheme: &mut SCHEME) {
    let mut colors = Vec::new();
    for line in lines_to_vec("lule_palette") {
        colors.push(pastel::Color::from_hex(&line));
    }
    scheme.set_colors(Some(colors));
}
